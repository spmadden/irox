// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{BitsWrapper, Error, MutBits};
use irox_cryptids::ed25519::Ed25519PublicKey;
use irox_openpgp::armor::{ArmorType, Dearmor};
use irox_openpgp::keybox::Keybox;
use irox_openpgp::packets::{
    CreationTime, EdDSALegacy, EdDSALegacySignature, FeaturesSubpkt, Issuer, KeyExpiration,
    KeyFlags, KeyServerPreferences, OpenPGPMessage, OpenPGPPackeStream, OpenPGPPacket,
    OpenPGPPacketData, OpenPGPPacketHeader, OpenPGPPacketType, PreferredAEADSymCiphers,
    PreferredCompressionAlgorithms, PreferredHashAlgorithms, PreferredV1SEIPDSymCiphers,
    PubKeyData, PubKeyPacket, PubKeyV4, PubkeyAlgorithm, SigV4PacketBuilder, SignatureData,
    SignaturePacket, SignatureSubpacket, SignatureSubtype, Trust, ECDH,
};
use irox_openpgp::types::{
    CompressionAlgorithm, ECC_Curve, Features, HashAlgorithm, KeyFlag, KeyServerPreference,
    SymmetricKeyAlgorithm,
};
use irox_time::datetime::UTCDateTime;
use irox_time::Duration;
use irox_tools::hash::{SHA1, SHA512};
use irox_tools::hex;
use irox_tools::hex::{to_hex_str_upper, HexDump};
use std::io::Write;

static PUBKEY_30774409_A: &str = include_str!("30774409.opgp.pub");
static PUBKEY_30774409_B: &[u8] = include_bytes!("30774409.opgp.pgpg");
static PUBKEY_22363081_B: &[u8] = include_bytes!("22363081.opgp.pgpg");

#[test]
pub fn test_read_binary() -> Result<(), Error> {
    let mut key = PUBKEY_22363081_B;
    let mut out = std::io::stdout();
    let out = &mut BitsWrapper::Borrowed(&mut out);
    let mut stream = OpenPGPPackeStream::new(BitsWrapper::Borrowed(&mut key));
    while let Some((ty, len, pkt)) = stream.read_next_packet()? {
        writeln!(out, "TYPE: {ty}, LEN: {:02X}", len)?;
        pkt.hexdump_to(out)?;
        if ty == 14 || ty == 6 {
            let mut hash = SHA1::new();
            hash.write_u8(0x99)?;
            hash.write_be_u16(len as u16)?;
            let fp = hash.hash(&pkt);
            fp.as_slice().hexdump_to(out)?;
        }
    }
    Ok(())
}
#[test]
pub fn test_read_armor() -> Result<(), Error> {
    let mut key = PUBKEY_30774409_A.as_bytes();
    let mut stream = key.dearmor();
    {
        let message = OpenPGPMessage::build_from(&mut stream)?;
        let mut iter = message.packets.iter();
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::PublicKey,
                    packet_length: 51,
                },
                data: OpenPGPPacketData::PublicKey(PubKeyPacket::Version4(PubKeyV4 {
                    timestamp: UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap(),
                    algorithm: PubkeyAlgorithm::EdDSALegacy,
                    data: PubKeyData::EdDSALegacy(EdDSALegacy {
                        curve: ECC_Curve::Ed25519Legacy,
                        pubkey: hex!(
                            "64EC46942F76F93C41CB67191F3711EB915616F263F9B534E4266F4644DF775D"
                        )
                        .into(),
                    }),
                    fingerprint_data: vec![],
                    fingerprint: hex!("BE76B1D4011C36AF2E82A4F37AD0BDA36123F973"),
                    keygrip: Some(hex!("2611D427C9D2F6DEA60923C6BB78156615350BF0")),
                })),
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::UserID,
                    packet_length: 44,
                },
                data: OpenPGPPacketData::UserID(String::from(
                    "Sean P. Madden (YK5NO) <sean@seanmadden.net>"
                )),
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::Signature,
                    packet_length: 153,
                },
                data: OpenPGPPacketData::Signature(SignaturePacket::Version4(
                    SigV4PacketBuilder::default()
                        .with_hash_algorithm(HashAlgorithm::SHA512)
                        .with_subtype(SignatureSubtype::PositiveCertification)
                        .with_pubkey_algorithm(PubkeyAlgorithm::EdDSALegacy)
                        .with_upper_signed_hash(0x3C3F)
                        .with_signature_data(SignatureData::EdDSALegacy(EdDSALegacySignature {
                            r: hex!(
                                "947A1640B9053498A975FE0F429750C9FECAE55A6DAC0175B0D622431B548C68"
                            )
                            .into(),
                            s: hex!(
                                "26BB57A93B0DF5A0768320E6743F10C7E11D486C33111D5E4EDF1CC548159303"
                            )
                            .into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::IssuerFingerprint(Issuer {
                            vsn: 4,
                            issuer: hex!("BE76B1D4011C36AF2E82A4F37AD0BDA36123F973").into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::SigCreationTime(CreationTime(
                            UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap()
                        )))
                        .with_hashed_packet(SignatureSubpacket::KeyFlags(KeyFlags(vec![
                            KeyFlag::Certify,
                            KeyFlag::Sign
                        ])))
                        .with_hashed_packet(SignatureSubpacket::KeyExpirationTime(KeyExpiration(
                            Duration::from_days(1825)
                        )))
                        .with_hashed_packet(SignatureSubpacket::PreferredV1SEIPDSymCiphers(
                            PreferredV1SEIPDSymCiphers(vec![
                                SymmetricKeyAlgorithm::AES256,
                                SymmetricKeyAlgorithm::AES192,
                                SymmetricKeyAlgorithm::AES128,
                                SymmetricKeyAlgorithm::TripleDES
                            ])
                        ))
                        .with_hashed_packet(SignatureSubpacket::PreferredAEADCiphersuites(
                            PreferredAEADSymCiphers(vec![SymmetricKeyAlgorithm::TripleDES])
                        ))
                        .with_hashed_packet(SignatureSubpacket::PreferredHashAlgorithms(
                            PreferredHashAlgorithms(vec![
                                HashAlgorithm::SHA512,
                                HashAlgorithm::SHA384,
                                HashAlgorithm::SHA256,
                                HashAlgorithm::SHA224,
                                HashAlgorithm::SHA1
                            ])
                        ))
                        .with_hashed_packet(SignatureSubpacket::PreferredCompressionAlgorithms(
                            PreferredCompressionAlgorithms(vec![
                                CompressionAlgorithm::ZLIB,
                                CompressionAlgorithm::BZip2,
                                CompressionAlgorithm::ZIP
                            ])
                        ))
                        .with_hashed_packet(SignatureSubpacket::Features(FeaturesSubpkt(vec![
                            Features::Version1SymEncIPD
                        ])))
                        .with_hashed_packet(SignatureSubpacket::KeyServerPreferences(
                            KeyServerPreferences(vec![KeyServerPreference::NoModify])
                        ))
                        // .with_hashed_packet(SignatureSubpacket::PreferredKeyServer("https://keyserver.ubuntu.com/pks/lookup?op=get&search=0xbe76b1d4011c36af2e82a4f37ad0bda36123f973".to_string()))
                        .build()?
                ))
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::Signature,
                    packet_length: 121,
                },
                data: OpenPGPPacketData::Signature(SignaturePacket::Version4(
                    SigV4PacketBuilder::default()
                        .with_hash_algorithm(HashAlgorithm::SHA512)
                        .with_subtype(SignatureSubtype::GenericCertification)
                        .with_pubkey_algorithm(PubkeyAlgorithm::EdDSALegacy)
                        .with_upper_signed_hash(0x0319)
                        .with_signature_data(SignatureData::EdDSALegacy(EdDSALegacySignature {
                            r: hex!(
                                "2BCB5B1F191091D2BF9361B9AA021956CECCEC82C052E647847C9C357F6F5D7D"
                            )
                            .into(),
                            s: hex!(
                                "480A630700C74BAC6C8888E117C041C623CDF53268E71767050D3A0043A4FB0F"
                            )
                            .into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::IssuerFingerprint(Issuer {
                            vsn: 4,
                            issuer: hex!("9BF59DBBF87B6EA003D1B0083C73348B3992BBB9").into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::SigCreationTime(CreationTime(
                            UTCDateTime::try_from_iso8601("20250108T023530Z").unwrap()
                        )))
                        .with_hashed_packet(SignatureSubpacket::TrustSignature(Trust {
                            depth: 2,
                            trust_amount: 120,
                        }))
                        .build()?
                ))
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::PublicSubkey,
                    packet_length: 51,
                },
                data: OpenPGPPacketData::PublicSubkey(PubKeyPacket::Version4(PubKeyV4 {
                    timestamp: UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap(),
                    algorithm: PubkeyAlgorithm::EdDSALegacy,
                    data: PubKeyData::EdDSALegacy(EdDSALegacy {
                        curve: ECC_Curve::Ed25519Legacy,
                        pubkey: hex!(
                            "7939E192F733BC7E2B5F0310DE1744C64FEB00DDF41E52079B34019F18D0FCE9"
                        )
                        .into(),
                    }),
                    fingerprint_data: vec![],
                    fingerprint: hex!("D4C631AACC04A919DDCE22DA17A8855EE72A4672"),
                    keygrip: Some(hex!("2BD5D5B9C45ECAA0163D2F05E962BDC0D36CCB33")),
                }))
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::Signature,
                    packet_length: 126,
                },
                data: OpenPGPPacketData::Signature(SignaturePacket::Version4(
                    SigV4PacketBuilder::default()
                        .with_hash_algorithm(HashAlgorithm::SHA512)
                        .with_subtype(SignatureSubtype::SubkeyBinding)
                        .with_pubkey_algorithm(PubkeyAlgorithm::EdDSALegacy)
                        .with_upper_signed_hash(0x330C)
                        .with_signature_data(SignatureData::EdDSALegacy(EdDSALegacySignature {
                            r: hex!(
                                "BA76DF4D63F0784B94C233B0DFD3E87A5765AEB9FB8802EA0F8F6A4DA2461847"
                            )
                            .into(),
                            s: hex!(
                                "E57DC0265386E4F1AD83D1866A7539ABB9518A2B3F981D30FEC5439E739BCB0D"
                            )
                            .into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::IssuerFingerprint(Issuer {
                            vsn: 4,
                            issuer: hex!("BE76B1D4011C36AF2E82A4F37AD0BDA36123F973").into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::SigCreationTime(CreationTime(
                            UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap()
                        )))
                        .with_hashed_packet(SignatureSubpacket::KeyFlags(KeyFlags(vec![
                            KeyFlag::Authentication
                        ])))
                        .with_hashed_packet(SignatureSubpacket::KeyExpirationTime(KeyExpiration(
                            Duration::from_days(1825)
                        )))
                        .build()?
                ))
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::PublicSubkey,
                    packet_length: 56,
                },
                data: OpenPGPPacketData::PublicSubkey(PubKeyPacket::Version4(PubKeyV4 {
                    timestamp: UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap(),
                    algorithm: PubkeyAlgorithm::ECDH,
                    data: PubKeyData::ECDH(ECDH {
                        curve: ECC_Curve::Curve25519Legacy,
                        pubkey: hex!(
                            "3C66FB0DD882D951175CD74C5E46F05ECAE239ED9F43F5DF0DF695FDCC7E5D41"
                        )
                        .into(),
                        spare: 769,
                        hash_function: HashAlgorithm::SHA256,
                        sym_algorithm: SymmetricKeyAlgorithm::AES128,
                    }),
                    fingerprint_data: vec![],
                    fingerprint: hex!("EEA63F601932E28E704B21D75DA9760A6AA11C5E"),
                    keygrip: Some(hex!("054EA952BE2F2018C13EF61790EB4EB0AC374302")),
                }))
            })
        );
        assert_eq!(
            iter.next(),
            Some(&OpenPGPPacket {
                header: OpenPGPPacketHeader {
                    packet_type: OpenPGPPacketType::Signature,
                    packet_length: 126,
                },
                data: OpenPGPPacketData::Signature(SignaturePacket::Version4(
                    SigV4PacketBuilder::default()
                        .with_hash_algorithm(HashAlgorithm::SHA512)
                        .with_subtype(SignatureSubtype::SubkeyBinding)
                        .with_pubkey_algorithm(PubkeyAlgorithm::EdDSALegacy)
                        .with_upper_signed_hash(0x2402)
                        .with_signature_data(SignatureData::EdDSALegacy(EdDSALegacySignature {
                            r: hex!(
                                "0155016A9CC1F15261282B185A790FE92562112F1223018350C216D656D89092"
                            )
                            .into(),
                            s: hex!(
                                "F65091F0E32D838B217126F835C537DB8F89B2FE1D24DCE74BA84FF008160A03"
                            )
                            .into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::IssuerFingerprint(Issuer {
                            vsn: 4,
                            issuer: hex!("BE76B1D4011C36AF2E82A4F37AD0BDA36123F973").into(),
                        }))
                        .with_hashed_packet(SignatureSubpacket::SigCreationTime(CreationTime(
                            UTCDateTime::try_from_iso8601("20250108T021025Z").unwrap()
                        )))
                        .with_hashed_packet(SignatureSubpacket::KeyFlags(KeyFlags(vec![
                            KeyFlag::EncryptCommunications,
                            KeyFlag::EncryptStorage
                        ])))
                        .with_hashed_packet(SignatureSubpacket::KeyExpirationTime(KeyExpiration(
                            Duration::from_days(1825)
                        )))
                        .build()?
                ))
            })
        );
        while let Some(pkt) = iter.next() {
            panic!("extra PKT: {pkt:?}");
        }
    }
    let res = stream.finish()?;
    let mut iter = res.headers.iter();
    assert_eq!(
        iter.next(),
        Some(&(
            "User ID".to_string(),
            "Sean P. Madden (YK5NO) <sean@seanmadden.net>".to_string()
        ))
    );
    assert_eq!(
        iter.next(),
        Some(&("Valid from".to_string(), "2025-01-07 21:10".to_string()))
    );
    assert_eq!(
        iter.next(),
        Some(&("Valid until".to_string(), "2030-01-06 21:10".to_string()))
    );
    assert_eq!(
        iter.next(),
        Some(&(
            "Type".to_string(),
            "255-bit EdDSA (secret key available)".to_string()
        ))
    );
    assert_eq!(
        iter.next(),
        Some(&(
            "Usage".to_string(),
            "Signing, Encryption, Certifying User IDs, SSH Authentication".to_string()
        ))
    );
    assert_eq!(
        iter.next(),
        Some(&(
            "Fingerprint".to_string(),
            "BE76B1D4011C36AF2E82A4F37AD0BDA36123F973".to_string()
        ))
    );
    assert_eq!(ArmorType::PubKey, res.armor_type);

    Ok(())
}

#[test]
pub fn test_packets() -> Result<(), Error> {
    let mut key = PUBKEY_22363081_B;
    let msg = OpenPGPMessage::build_from(&mut key)?;
    for pkt in msg.packets {
        println!("{pkt:?}");
        match pkt.data {
            OpenPGPPacketData::PublicKey(pk) => {
                println!("  PUBKEY: {pk:#?}");
            }
            OpenPGPPacketData::PublicSubkey(psk) => {
                println!("  PUBSUBKEY: {psk:#?}");
            }
            OpenPGPPacketData::Signature(sig) => {
                println!("  SIG: {sig:#?}");
            }
            _ => {}
        }
    }
    Ok(())
}

#[test]
pub fn test_verify_sig() -> Result<(), Error> {
    let _sig = hex!(
        "900d03000a16c37e4043df6810f101cb78620067f17fb84c61646965732061"
        "6e642047656e746c656d656e206f662074686520636c617373206f66202739"
        "393a204966204920636f756c64206f6666657220796f75206f6e6c79206f6e"
        "652074697020666f7220746865206675747572652c2073756e73637265656e"
        "20776f756c642062652069742e88750400160a001d162104ad96e0200673e2"
        "b6d24a7d25c37e4043df6810f1050267f17fb8000a0910c37e4043df6810f1"
        "1d3300ff42e9327a8c1385f320122d4128633483fa5fbd39d1b46ba2766436"
        "cd51a55fe801009b395cff02254d474a8c83640f4c6ec123348d6419c81afd"
        "5ae6f84ba934fd0e");
    let sig = hex!(
        "900d03000a16c37e4043df6810f101cb0d620067f1a3ee4f70656e50475088"
        "750400160a001d162104ad96e0200673e2b6d24a7d25c37e4043df6810f105"
        "0267f1a3ee000a0910c37e4043df6810f14b180100f57ac130ab886e911fc2"
        "06a6f6b7e3a3ea925401e3c96c6d2cf1ccc2b9cca04d00fd1debe06011ac11"
        "9734d536c68aff80741c9e70edc0c10318492cf77f7383a602");
    let mut sig = sig.as_slice();
    let msg = OpenPGPMessage::build_from(&mut sig)?;
    let mut hash = SHA512::new();
    let mut count = 0u32;
    let mut buf = Vec::new();
    let mut sigdata = None;
    for pkt in msg.packets {
        println!("{pkt:?}");
        match pkt.data {
            OpenPGPPacketData::Signature(sig) => {
                println!("  SIG: {sig:#?}");
                let data = sig.get_hashed_data();
                buf.write_all_bytes(data)?;
                count += data.len() as u32;
                buf.write_u8(sig.get_version())?;
                buf.write_u8(0xFF)?;
                buf.write_be_u32(count)?;
                sigdata = Some(sig.try_into_ed25519_sig()?);
            }
            OpenPGPPacketData::LiteralData(data) => {
                println!("  DATA: {data:#?}");
                let data = data.data.as_slice();
                buf.write_all_bytes(data)?;
            }
            OpenPGPPacketData::OnePassSignature(sig) => {
                println!("  OPS: {sig:#?}");
            }
            _ => {}
        }
    }
    buf.hexdump();
    let _ = hash.write_all_bytes(&buf);
    let hash = hash.finish();
    println!("HASH {}", to_hex_str_upper(&hash));

    let pk: Ed25519PublicKey =
        hex!("EA914CD608365EF274D81FDBC02D8A85BC4490F521837ECCA9ED66C25FD0CCF3")
            .try_into()
            .unwrap();
    let sigdata = sigdata.unwrap();
    println!("PK: {}", to_hex_str_upper(pk.as_ref()));
    println!("SIG: {}", to_hex_str_upper(sigdata.as_slice()));
    pk.verify_signed_message(&hash, &sigdata).unwrap();
    println!("SIGNATURE VERIFIED!");
    Ok(())
}
macro_rules! check_msg {
    ($key:ident,$keybox:ident, $asserts:expr) => {
        let message = OpenPGPMessage::build_from(&mut $key)?;
        message.add_to_keybox(&mut $keybox)?;
        for result in message.validate_signatures(&$keybox)? {
            println!("{result:#?}");
        }
    };
}
#[test]
pub fn test_keybox() -> Result<(), Error> {
    let mut keybox = Keybox::default();
    let mut k1 = PUBKEY_30774409_A.as_bytes();
    let mut k1 = k1.dearmor();
    let mut k2 = PUBKEY_30774409_B;
    let mut k3 = PUBKEY_22363081_B;
    check_msg!(k1, keybox, []);
    check_msg!(k2, keybox, []);
    check_msg!(k3, keybox, []);
    Ok(())
}
