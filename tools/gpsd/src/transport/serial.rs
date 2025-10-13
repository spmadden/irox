use std::io::{Read, Write};

use crate::error::GPSdError;
use clap::Parser;
use irox_bits::BitsWrapper;
use log::info;
use serial2::{CharSize, FlowControl, IntoSettings, Parity, SerialPort, Settings, StopBits};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct SerialConfig {
    /// Serial port path, like ("COM0", or "/dev/ttyS0")
    #[arg()]
    pub serial_port: String,

    #[arg(short = 'e', long)]
    pub encoding: crate::config::EncodingType,

    /// Baud rate, one of (110, 300, 600, 1200, 2400, 4800, 9600, 19200, 38400,
    /// 57600, 115200)
    #[arg(short = 'b', long, default_value_t = 9600)]
    pub baud_rate: usize,

    /// Character size, one of (5, 6, 7, 8)
    #[arg(short='c', long, default_value_t=8, value_parser=clap::value_parser!(u8).range(5..9))]
    pub char_size: u8,

    /// Parity, one of ("none", "odd", "even")
    #[arg(short = 'p', long, default_value = "none")]
    pub parity: String,

    /// Number of stop bits, one of (1, 2)
    #[arg(short='s', long, default_value_t=1, value_parser=clap::value_parser!(u8).range(1..3))]
    pub stop_bits: u8,

    /// Flow control, one of ("none", "software", "hardware", "xonxoff", "rtscts")
    #[arg(short = 'f', long, default_value = "none")]
    pub flow_control: String,
}

impl IntoSettings for &SerialConfig {
    fn apply_to_settings(self, settings: &mut Settings) -> std::io::Result<()> {
        settings.set_raw();
        settings.set_baud_rate(self.baud_rate as u32)?;
        if self.stop_bits == 2 {
            settings.set_stop_bits(StopBits::Two);
        }
        if self.char_size != 8 {
            settings.set_char_size(match self.char_size {
                5 => CharSize::Bits5,
                6 => CharSize::Bits6,
                7 => CharSize::Bits7,
                8 => CharSize::Bits8,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid char size",
                    ))
                }
            })
        }
        if self.flow_control != "none" {
            settings.set_flow_control(match self.flow_control.to_ascii_lowercase().as_str() {
                "none" => FlowControl::None,
                "software" | "xonxoff" => FlowControl::XonXoff,
                "hardware" | "rtscts" => FlowControl::RtsCts,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid flow control",
                    ))
                }
            })
        }
        if self.parity != "none" {
            settings.set_parity(match self.parity.to_ascii_lowercase().as_str() {
                "none" => Parity::None,
                "odd" => Parity::Odd,
                "even" => Parity::Even,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid parity",
                    ))
                }
            });
        }
        Ok(())
    }
}

pub struct SerErr(pub GPSdError);

pub fn open(config: &SerialConfig) -> Result<BitsWrapper<impl Read + Write>, SerErr> {
    let port = SerialPort::open(&config.serial_port, config)?;
    info!("Successfully opened serial port: {}", config.serial_port);
    let port = BitsWrapper::Owned(port);
    Ok(port)
}

impl From<std::io::Error> for SerErr {
    fn from(value: std::io::Error) -> Self {
        SerErr(GPSdError::new_str(value.to_string()))
    }
}

impl From<GPSdError> for SerErr {
    fn from(value: GPSdError) -> Self {
        SerErr(value)
    }
}

impl<T> From<SerErr> for Result<T, SerErr> {
    fn from(value: SerErr) -> Self {
        Err(value)
    }
}
