// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::{load_authenticode_hash, OptionalHeader, PEFile};
use cms::cert::x509::attr::Attribute;
use cms::cert::x509::der::{Decode, Encode, Length};
use cms::cert::x509::spki::ObjectIdentifier;
use cms::cert::CertificateChoices;
use cms::signed_data::SignerIdentifier;
use core::fmt::{Debug, Formatter};
use irox_bits::{BitsErrorKind, Error, SeekRead};
use irox_tools::hash::{Hasher, SHA1, SHA256};
use irox_tools::hex;
use irox_tools::hex::HexDump;
use irox_tools::iterators::permuted::Permutation;
use p384::ecdsa::signature::hazmat::PrehashVerifier;
use rsa::pkcs8::DecodePublicKey;
use rsa::Pkcs1v15Sign;
use std::fs::File;
use std::path::Path;

pub fn read_certificate_data(
    file: &mut File,
    pe: &PEFile,
) -> Result<cms::signed_data::SignedData, Error> {
    let Some(optional) = &pe.optional_header else {
        return Error::err(BitsErrorKind::FormatError, "Missing optional header");
    };
    let OptionalHeader::OptionalPEPlusHeader(plus) = &optional else {
        return Error::err(BitsErrorKind::FormatError, "Missing 64bit header");
    };
    let ct = plus.data_directories.certificate_table;
    let off = ct.virtual_address + 8;
    let len = ct.size as usize - 8;
    let mut buf = vec![0u8; len];
    file.seek_read_all(&mut buf, off as u64)?;

    // buf.hexdump();

    let data = match cms::content_info::ContentInfo::from_der(buf.as_slice()) {
        Ok(data) => data,
        Err(e) => {
            if let cms::cert::x509::der::ErrorKind::TrailingData { decoded, .. } = e.kind() {
                let b = buf.as_slice().split_at(u32::from(decoded) as usize).0;
                let Ok(data) = cms::content_info::ContentInfo::from_der(b) else {
                    return Error::err(
                        BitsErrorKind::FormatError,
                        "Unable to read PKCS7 content from the file",
                    );
                };
                data
            } else {
                return Error::err(
                    BitsErrorKind::FormatError,
                    "Unable to read PKCS7 content from the file",
                );
            }
        }
    };
    let oid = data.content_type;

    if oid.as_bytes() != hex!("2A864886F70D010702") {
        return Error::err(BitsErrorKind::FormatError, "Expecting Signed Data OID");
    }

    let Ok(signed) = data.content.decode_as() else {
        return Error::err(BitsErrorKind::FormatError, "Unable to decode signed data");
    };
    Ok(signed)
}

pub fn verify_authenticode_signatures<T: AsRef<Path>>(
    file: T,
) -> Result<Vec<ValidSignature>, Error> {
    let mut file = File::open(file)?;
    let pe = PEFile::parse_from(&mut file)?;
    let signed = read_certificate_data(&mut file, &pe)?;

    let Some(certificates) = &signed.certificates else {
        return Error::err(BitsErrorKind::FormatError, "Missing certificates");
    };

    let Some(alg) = signed.digest_algorithms.get(0) else {
        return Error::err(BitsErrorKind::FormatError, "Missing digest algorithm");
    };
    if alg.oid.as_bytes() != hex!("608648016503040201") {
        return Error::err(
            BitsErrorKind::FormatError,
            "Expecting SHA256 Signed Data OID",
        );
    }
    let mut hasher = Hasher::SHA256(SHA256::default());
    load_authenticode_hash(&mut file, &pe, &mut hasher)?;
    let calculated_hash = hasher.finish();

    let mut ha = calculated_hash.as_ref();
    ha.hexdump();
    let Some(sigdata) = signed.encap_content_info.econtent else {
        return Error::err(BitsErrorKind::FormatError, "Expecting EC Content Header");
    };
    let sigdata = sigdata.value();

    let constructed_sigdata_prelim = hex!(
        // ASN.1 DER spcIndirectDataContext + spcPEImageData prefix and SHA-256 OID up to the hash value
        "3017060A2B06010401823702010F3009030100A004A20280003031300D060960864801650304020105000420"
    );
    let mut constructed_sigdata = Vec::from(constructed_sigdata_prelim);
    constructed_sigdata.extend(calculated_hash);
    if constructed_sigdata.len() != 0x4C {
        return Error::err(
            BitsErrorKind::FormatError,
            "Invalid constructed signature data length - not 0x4C",
        );
    }
    let mut cs = constructed_sigdata.as_slice();
    cs.hexdump();
    if constructed_sigdata != sigdata {
        return Error::err(
            BitsErrorKind::FormatError,
            "Invalid constructed signature data - file signature doesn't match calculated value",
        );
    }

    if signed.signer_infos.0.is_empty() {
        return Error::err(BitsErrorKind::FormatError, "Empty signer infos");
    }
    let mut out = Vec::new();

    let sig = SHA256::default().hash(sigdata);
    let mut sh = sig.as_slice();
    sh.hexdump();
    for info in signed.signer_infos.0.iter() {
        let Some(attrs) = &info.signed_attrs else {
            continue;
        };
        // find messageDigest OID
        let md = attrs
            .iter()
            .find(|attr| attr.oid.as_bytes() == hex!("2A864886F70D010904"));
        let Some(md) = md else {
            return Error::err(
                BitsErrorKind::FormatError,
                "Expecting authenticated message digest attribute",
            );
        };
        let Some(val) = md.values.get(0) else {
            return Error::err(
                BitsErrorKind::FormatError,
                "Expecting authenticated message value",
            );
        };
        if sig != val.value() {
            return Error::err(BitsErrorKind::FormatError, "signed data hash didn't match!");
        }

        let Some(authattr) = &info.signed_attrs else {
            continue;
        };
        let mut attrhasher = AttrHasher::new(authattr.as_slice());

        let sid = &info.sid;
        let cert = match sid {
            SignerIdentifier::IssuerAndSerialNumber(isn) => certificates.0.iter().find(|cert| {
                let CertificateChoices::Certificate(cert) = cert else {
                    return false;
                };
                cert.tbs_certificate.serial_number.as_bytes() == isn.serial_number.as_bytes()
            }),
            SignerIdentifier::SubjectKeyIdentifier(skid) => certificates.0.iter().find(|cert| {
                let CertificateChoices::Certificate(cert) = cert else {
                    return false;
                };
                let Some(exts) = &cert.tbs_certificate.extensions else {
                    return false;
                };
                for ext in exts {
                    if ext.extn_id.as_bytes() == hex!("551D0E") {
                        return ext.extn_value.as_bytes() == skid.0.as_bytes();
                    }
                }
                false
            }),
        };
        let Some(cert) = cert else {
            return Error::err(BitsErrorKind::FormatError, "Missing certificate");
        };
        let CertificateChoices::Certificate(cert) = cert else {
            return Error::err(BitsErrorKind::FormatError, "Invalid certificate format.");
        };
        let mut certdata = Vec::new();
        if cert.encode_to_vec(&mut certdata).is_err() {
            return Error::err(
                BitsErrorKind::FormatError,
                "Unable to encode certificate data for fingerprinting",
            );
        };
        let certfp_sha1 = SHA1::new().hash(&certdata);
        let certfp_sha256 = SHA256::new().hash(&certdata);
        let pk = &cert.tbs_certificate.subject_public_key_info;
        let oid = pk.algorithm.oid.as_bytes();
        match oid {
            [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01] => {
                // ecdsa
                let Some(params) = &pk.algorithm.parameters else {
                    continue;
                };
                let Ok(oid) = params.decode_as::<ObjectIdentifier>() else {
                    return Error::err(
                        BitsErrorKind::FormatError,
                        "ECDSA didn't contain an algorithm OID",
                    );
                };
                if oid.as_bytes() == hex!("2B81040022") {
                    // p384
                    let Some(pk) = pk.subject_public_key.as_bytes() else {
                        return Error::err(
                            BitsErrorKind::FormatError,
                            "Missing public key in certificate",
                        );
                    };
                    let Ok(vk) = p384::ecdsa::VerifyingKey::from_sec1_bytes(pk) else {
                        return Error::err(
                            BitsErrorKind::FormatError,
                            "Unable to parse p384 verifying key from certificate",
                        );
                    };
                    let Ok(encsig) = p384::ecdsa::Signature::from_der(info.signature.as_bytes())
                    else {
                        return Error::err(
                            BitsErrorKind::FormatError,
                            "Unable to parse signature from certificate",
                        );
                    };
                    while let Some(signed_hash_data) = attrhasher.next_hash() {
                        let Ok(()) = vk.verify_prehash(&signed_hash_data, &encsig) else {
                            continue;
                        };
                        out.push(ValidSignature {
                            signer_sha1_fingerprint: certfp_sha1,
                            signer_sha256_fingerprint: certfp_sha256,
                        });
                        break;
                    }
                } else {
                    return Error::err(
                        BitsErrorKind::Unsupported,
                        "Unsupported public key algorithm",
                    );
                }
            }
            [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01] => {
                // rsa
                let mut pkv = Vec::new();
                let Ok(_) = pk.encode_to_vec(&mut pkv) else {
                    continue;
                };
                let Ok(pk) = rsa::RsaPublicKey::from_public_key_der(&pkv) else {
                    continue;
                };
                while let Some(signed_hash_data) = attrhasher.next_hash() {
                    let Ok(()) = pk.verify(
                        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
                        signed_hash_data.as_slice(),
                        info.signature.as_bytes(),
                    ) else {
                        continue;
                    };
                    out.push(ValidSignature {
                        signer_sha1_fingerprint: certfp_sha1,
                        signer_sha256_fingerprint: certfp_sha256,
                    });
                    break;
                }
            }
            _ => {
                return Error::err(
                    BitsErrorKind::Unsupported,
                    "Unsupported public key algorithm",
                );
            }
        }
    }

    Ok(out)
}

#[derive(Clone, Eq, PartialEq)]
pub struct ValidSignature {
    pub signer_sha1_fingerprint: [u8; 20],
    pub signer_sha256_fingerprint: [u8; 32],
}
impl Debug for ValidSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ValidSignature")
            .field(
                "signer_sha1_fingerprint",
                &hex::to_hex_str_upper(&self.signer_sha1_fingerprint),
            )
            .field(
                "signer_sha256_fingerprint",
                &hex::to_hex_str_upper(&self.signer_sha256_fingerprint),
            )
            .finish()
    }
}

struct AttrHasher {
    permuted: Permutation<Attribute>,
}
impl AttrHasher {
    pub fn new(attrs: &[Attribute]) -> Self {
        let permuted = Permutation::new(attrs.to_vec());
        Self { permuted }
    }
    pub fn next_hash(&mut self) -> Option<[u8; 32]> {
        let next = self.permuted.next()?;
        let mut encoded = Vec::new();
        for attr in next {
            if let Err(_e) = attr.encode_to_vec(&mut encoded) {
                return None;
            }
        }
        let mut out = vec![0x31];
        if let Err(_e) = Length::new(encoded.len() as u16).encode_to_vec(&mut out) {
            return None;
        }
        out.extend(encoded);
        Some(SHA256::default().hash(&out))
    }
}
