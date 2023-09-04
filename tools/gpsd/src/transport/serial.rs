use std::time::Duration;

use clap::{Parser, ValueEnum};
use log::info;
use serial::CharSize::{Bits5, Bits6, Bits7, Bits8};
use serial::{
    BaudRate, CharSize, Error, FlowControl, Parity, PortSettings, SerialPort, StopBits, SystemPort,
};

use crate::error::GPSdError;

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum EncodingType {
    #[default]
    Nmea0183,
    SirfBinary,
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct SerialConfig {
    /// Serial port path, like ("COM0", or "/dev/ttyS0")
    #[arg()]
    pub serial_port: String,

    #[arg(short = 'e', long)]
    pub encoding: EncodingType,

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

    /// Flow control, one of ("none", "software", "hardware")
    #[arg(short = 'f', long, default_value = "none")]
    pub flow_control: String,
}

pub struct SerErr(pub GPSdError);

pub fn open(config: SerialConfig) -> Result<SystemPort, SerErr> {
    let mut port = serial::open(config.serial_port.as_str())?;
    info!("Successfully opened serial port: {}", config.serial_port);
    configure(&mut port, &config)?;
    info!("Successfully configured serial port: {config:?}");
    Ok(port)
}

pub fn char_size(char_size: u8) -> Result<CharSize, GPSdError> {
    match char_size {
        5 => Ok(Bits5),
        6 => Ok(Bits6),
        7 => Ok(Bits7),
        8 => Ok(Bits8),
        e => GPSdError::err_str(format!("Invalid char_size: {}", e)),
    }
}

pub fn parity(parity: &str) -> Result<Parity, GPSdError> {
    match parity {
        "none" => Ok(Parity::ParityNone),
        "even" => Ok(Parity::ParityEven),
        "odd" => Ok(Parity::ParityOdd),
        e => GPSdError::err_str(format!("Invalid parity: {}", e)),
    }
}

pub fn stop_bits(stop_bits: u8) -> Result<StopBits, GPSdError> {
    match stop_bits {
        1 => Ok(StopBits::Stop1),
        2 => Ok(StopBits::Stop2),
        e => GPSdError::err_str(format!("Invalid stop bits: {}", e)),
    }
}

pub fn flow_control(flow_control: &str) -> Result<FlowControl, GPSdError> {
    match flow_control {
        "none" => Ok(FlowControl::FlowNone),
        "software" => Ok(FlowControl::FlowSoftware),
        "hardware" => Ok(FlowControl::FlowHardware),
        e => GPSdError::err_str(format!("Invalid flow control: {}", e)),
    }
}

pub fn configure<T: SerialPort>(port: &mut T, config: &SerialConfig) -> Result<(), SerErr> {
    let baud_rate = BaudRate::from_speed(config.baud_rate);
    let char_size = char_size(config.char_size)?;
    let parity = parity(&config.parity)?;
    let stop_bits = stop_bits(config.stop_bits)?;
    let flow_control = flow_control(&config.flow_control)?;

    let settings = PortSettings {
        baud_rate,
        char_size,
        parity,
        stop_bits,
        flow_control,
    };
    port.configure(&settings)?;
    port.set_timeout(Duration::from_secs(1))?;

    Ok(())
}

impl From<Error> for SerErr {
    fn from(value: Error) -> Self {
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
