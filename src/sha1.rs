
type RAW = [u8;20];

#[derive(Clone)]
pub struct SHA1 {
    pub(crate) raw : RAW,
    pub(crate) dir : String,
    pub(crate) file : String,
}

impl SHA1 {
    pub fn new(raw : RAW) -> SHA1 {
        let dir = hex::encode(&raw[..1]);
        let file = hex::encode(&raw[1..]);
        SHA1{raw, dir, file}
    }
}

impl std::fmt::Display for SHA1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.dir, self.file))
    }
}

impl std::fmt::Debug for SHA1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SHA1").field("raw", &self.raw).field("dir", &self.dir).field("file", &self.file).finish()
    }
}

impl TryFrom<String> for SHA1 {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.len();
        let cpy = value.clone();
        if len != 40 {
            return Err(format!("Expected length = 40, but was {}", len).into())
        }

        let result = hex::decode(value)?;
        let raw : Result<RAW, Vec<u8>> = result.try_into();
        match raw {
            Ok(r) => Ok(SHA1::new(r)),
            Err(_) => Err(format!("Unable to convert {} into SHA1", cpy).into()),
        }
    }
}

impl TryFrom<&'static str> for SHA1 {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        String::from(value).try_into()
    }
}

