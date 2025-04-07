use irox_tools::hash::SHA1;

pub trait KeyGrip {
    fn generate_keygrip(&self) -> Option<[u8; 20]>;
}

pub fn gen_keygrip(elems: &[(&str, &[u8])]) -> [u8; 20] {
    let mut hash = SHA1::new();

    for (name, val) in elems {
        hash.write(b"(");
        let exp = format!("{}:{}{}:", name.len(), name, val.len());
        hash.write(exp.as_bytes());
        hash.write(val);
        hash.write(b")");
    }

    hash.finish()
}

#[cfg(test)]
mod tests {
    use crate::types::ECC_Curve;
    use irox_cryptids::ed25519::Ed25519SecretKey;
    use irox_tools::{assert_eq_hex_slice, hex};

    #[test]
    pub fn test_gen_ed25519_keygrip1() {
        let sk = Ed25519SecretKey([0; 32]);
        let pk = sk.generate_public_key();
        let exp = hex!("BF01906C17B634A32DF4C0677299183CDD42513F");
        let gen = ECC_Curve::Ed25519Legacy.keygrip(pk.as_ref());
        assert_eq_hex_slice!(exp, gen);
    }
    #[test]
    pub fn test_gen_ed25519_keygrip2() {
        let pk = hex!("64EC46942F76F93C41CB67191F3711EB915616F263F9B534E4266F4644DF775D");
        let exp = hex!("2611D427C9D2F6DEA60923C6BB78156615350BF0");
        let gen = ECC_Curve::Ed25519Legacy.keygrip(pk.as_ref());
        assert_eq_hex_slice!(exp, gen);
    }
    #[test]
    pub fn test_gen_x25519_keygrip1() {
        let sk = Ed25519SecretKey([0; 32]);
        let pk = sk.generate_public_key();
        let exp = hex!("78D686E4A6FC3B0F59E17D4C77C49E64F1518A00");
        let gen = ECC_Curve::Curve25519Legacy.keygrip(pk.as_ref());
        assert_eq_hex_slice!(exp, gen);
    }

    #[test]
    pub fn test_gen_x25519_keygrip2() {
        let pk = hex!("3C66FB0DD882D951175CD74C5E46F05ECAE239ED9F43F5DF0DF695FDCC7E5D41");
        let exp = hex!("054EA952BE2F2018C13EF61790EB4EB0AC374302");
        let gen = ECC_Curve::Curve25519Legacy.keygrip(pk.as_ref());
        assert_eq_hex_slice!(exp, gen);
    }
}
