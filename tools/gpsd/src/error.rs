use std::error::Error;
use std::fmt::Display;

pub struct GPSdError {
    pub(crate) msg: String,
}

impl GPSdError {
    pub fn new(err: &str) -> GPSdError {
        GPSdError {
            msg: err.to_string(),
        }
    }

    pub fn new_str(msg: String) -> GPSdError {
        GPSdError { msg }
    }

    pub fn err_str<T, S: AsRef<str>>(msg: S) -> Result<T, GPSdError> {
        Err(GPSdError {
            msg: msg.as_ref().to_string(),
        })
    }

    pub fn err<T: Error, R>(err: T) -> Result<R, GPSdError> {
        Err(GPSdError {
            msg: err.to_string(),
        })
    }

    pub fn conv<T: Error, R>(input: Result<R, T>) -> Result<R, GPSdError> {
        input.map_err(|e| GPSdError { msg: e.to_string() })
    }
}

impl Display for GPSdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl std::fmt::Debug for GPSdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Error for GPSdError {}

impl From<std::io::Error> for GPSdError {
    fn from(err: std::io::Error) -> Self {
        GPSdError {
            msg: format!("{:?} : {}", err.kind(), err),
        }
    }
}

impl From<serde_json::Error> for GPSdError {
    fn from(value: serde_json::Error) -> Self {
        GPSdError::new_str(format!("{value:?}"))
    }
}

impl From<irox_nmea0183::Error> for GPSdError {
    fn from(value: irox_nmea0183::Error) -> Self {
        GPSdError::new_str(format!("{value:?}"))
    }
}

impl From<irox_sirf::error::Error> for GPSdError {
    fn from(value: irox_sirf::error::Error) -> Self {
        GPSdError::new_str(format!("{value:?}"))
    }
}

#[cfg(target_os = "windows")]
impl From<irox_winlocation_api::Error> for GPSdError {
    fn from(value: irox_winlocation_api::Error) -> Self {
        GPSdError::new_str(format!("{value:?}"))
    }
}
