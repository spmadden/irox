// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use clap::{
    builder::{TypedValueParser, ValueParser},
    error::{ContextKind, ContextValue, ErrorKind},
    Command, Parser, ValueHint,
};
use std::collections::VecDeque;
use std::{num::ParseFloatError, str::FromStr};

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Bounding box in WGS84 degrees, formatted as a list of lon,lat values like:
    /// 30.123,-133.234,25.2982,-120.8423
    #[arg(
        long,
        required = true,
        value_parser=ValueParser::new(BBOXParser)
    )]
    pub bbox: [f64; 4],

    /// Tile XYZ URL to download from.  
    /// Known substitutions:
    /// `{x}`: Tile X index - zero is dateline incrementing positive east
    /// `{y}`: Tile Y index - zero is north pole incrementing positive south
    /// `{-y}`: Tile Y index - zero is south pole incremenging positive north
    /// `{z}`: Tile Z index in the range 0..31
    /// `{s}`: Server parts, must specify the --server-parts argument

    #[arg(short, long, value_hint=ValueHint::Url)]
    pub url: String,

    /// The output MBTiles file to save the data
    #[arg(short, long, value_hint=ValueHint::FilePath)]
    pub out_file: String,

    /// The layername for the MBTiles file
    #[arg(short, long)]
    pub name: String,

    /// A list of comma separated zoom levels to go out and download
    #[arg(short, long, num_args = 1..=30, required=true, value_delimiter = ',')]
    pub zoom_levels: Vec<u8>,

    /// Optional list of 'server part' replacements for the `${s}` parameter in the URL.
    /// Example: "-s 0,1,2,3"
    #[arg(short, long, num_args=1..255, value_delimiter=',')]
    pub server_parts: Option<Vec<String>>,

    /// Optional Referrer to add to the HTTP request
    #[arg(short, long)]
    pub referrer: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct BBOXParser;

impl TypedValueParser for BBOXParser {
    type Value = [f64; 4];

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let str = String::from(value.to_string_lossy());
        let mut splits = str.split(',').collect::<VecDeque<_>>();
        let len = splits.len();
        if len != 4 {
            let mut err = clap::Error::new(ErrorKind::WrongNumberOfValues).with_cmd(cmd);
            err.insert(
                ContextKind::InvalidArg,
                ContextValue::String("--bbox".into()),
            );
            err.insert(
                ContextKind::ActualNumValues,
                ContextValue::Number(len as isize),
            );
            err.insert(ContextKind::ExpectedNumValues, ContextValue::Number(4));
            return Err(err);
        }

        let out: [f64; 4] = [
            str_to_f64(splits.pop_front(), cmd)?,
            str_to_f64(splits.pop_front(), cmd)?,
            str_to_f64(splits.pop_front(), cmd)?,
            str_to_f64(splits.pop_front(), cmd)?,
        ];

        Ok(out)
    }
}

fn parse_float_error<T>(e: &ParseFloatError, cmd: &Command, val: &str) -> Result<T, clap::Error> {
    let mut error = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
    error.insert(
        ContextKind::InvalidArg,
        ContextValue::String("--bbox".into()),
    );
    error.insert(ContextKind::InvalidValue, ContextValue::String(val.into()));
    error.insert(
        ContextKind::Suggested,
        ContextValue::StyledStrs(vec![e.to_string().into()]),
    );
    Err(error)
}

fn str_to_f64(val: Option<&str>, cmd: &Command) -> Result<f64, clap::Error> {
    let Some(val) = val else {
        let mut error = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
        error.insert(
            ContextKind::InvalidArg,
            ContextValue::String("--bbox".into()),
        );
        return Err(error);
    };
    match f64::from_str(val) {
        Ok(v) => Ok(v),
        Err(e) => parse_float_error(&e, cmd, val),
    }
}
