// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, EnumIterItem, EnumTryFromStr)]
pub enum DataTypeBinding {
    #[default]
    String,
    NormalizedString,
    Boolean,
    Base64Binary,
    HexBinary,

    Float,
    Double,
    Decimal,

    Byte,
    Short,
    Int,
    Long,
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
    PositiveInteger,
    NegativeInteger,
    NonPositiveInteger,
    NonNegativeInteger,

    Time,
    Date,
    DateTime,
    GYearMonth,
    GYear,
    GMonthDay,
    GDay,
    GMonth,
    Duration,
    YearMonthDuration,
    DateTimeStamp,

    AnyURI,
    NOTATION,
    QName,
}

impl EnumName for DataTypeBinding {
    fn name(&self) -> &'static str {
        match self {
            DataTypeBinding::String => "string",
            DataTypeBinding::NormalizedString => "normalizedString",
            DataTypeBinding::Boolean => "boolean",
            DataTypeBinding::Base64Binary => "base64Binary",
            DataTypeBinding::HexBinary => "hexBinary",
            DataTypeBinding::Float => "float",
            DataTypeBinding::Double => "double",
            DataTypeBinding::Decimal => "decimal",
            DataTypeBinding::Byte => "byte",
            DataTypeBinding::Short => "short",
            DataTypeBinding::Int => "int",
            DataTypeBinding::Long => "long",
            DataTypeBinding::UnsignedByte => "unsignedByte",
            DataTypeBinding::UnsignedShort => "unsignedShort",
            DataTypeBinding::UnsignedInt => "unsignedInt",
            DataTypeBinding::UnsignedLong => "unsignedLong",
            DataTypeBinding::PositiveInteger => "positiveInteger",
            DataTypeBinding::NegativeInteger => "negativeInteger",
            DataTypeBinding::NonPositiveInteger => "nonPositiveInteger",
            DataTypeBinding::NonNegativeInteger => "nonNegativeInteger",
            DataTypeBinding::Time => "time",
            DataTypeBinding::Date => "date",
            DataTypeBinding::DateTime => "dateTime",
            DataTypeBinding::GYearMonth => "gYearMonth",
            DataTypeBinding::GYear => "gYear",
            DataTypeBinding::GMonthDay => "gMonthDay",
            DataTypeBinding::GDay => "gDay",
            DataTypeBinding::GMonth => "gMonth",
            DataTypeBinding::Duration => "duration",
            DataTypeBinding::YearMonthDuration => "yearMonthDuration",
            DataTypeBinding::DateTimeStamp => "dateTimeStamp",
            DataTypeBinding::AnyURI => "anyURI",
            DataTypeBinding::NOTATION => "NOTATION",
            DataTypeBinding::QName => "QName",
        }
    }
}
