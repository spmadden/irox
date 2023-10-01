// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::{BufRead, BufReader};
use std::str::FromStr;

use log::trace;
use xml::reader::XmlEvent;

use crate::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaField {
    pub(crate) name: String,
    pub(crate) data_type: String,
    pub(crate) size: usize,
    pub(crate) offset: usize,
    pub(crate) param: Option<u32>,
}

impl SchemaField {
    pub(crate) fn decode_field(
        &self,
        data: &[u8],
        struct_index: usize,
        number_of_points: usize,
    ) -> Result<f64, Error> {
        let b0 = number_of_points * (self.offset) + struct_index;
        let b1 = number_of_points * (self.offset + 1) + struct_index;
        let b2 = number_of_points * (self.offset + 2) + struct_index;
        let b3 = number_of_points * (self.offset + 3) + struct_index;

        let Some(b0) = data.get(b0) else {
            return Error::decoding_err("Data underflow b0");
        };
        let Some(b1) = data.get(b1) else {
            return Error::decoding_err("Data underflow b1");
        };
        let Some(b2) = data.get(b2) else {
            return Error::decoding_err("Data underflow b2");
        };
        let Some(b3) = data.get(b3) else {
            return Error::decoding_err("Data underflow b3");
        };

        let mut val = u32::from_le_bytes([*b0, *b1, *b2, *b3]);
        match self.data_type.as_str() {
            "UInt32" => Ok(val as f64),
            "Float" => Ok(f32::from_bits(val) as f64),
            "Fixed32" => {
                let scale = self.param.unwrap_or(0);
                let mut mult = 1_f64;
                if val > 0x7FFF_FFFF {
                    val = !val;
                    mult = -1_f64;
                }
                let whole = (val >> scale) as f64;
                let div = 1_u32 << scale;
                let mask = div - 1;
                let part = (val & mask) as f64 / div as f64;

                let val = mult * (whole + part);
                Ok(val)
            }
            _ => Error::decoding_str(format!("Unknown data type: {}", self.data_type)),
        }
    }
}

#[derive(Default)]
pub struct SchemaFieldBuilder {
    pub(crate) name: Option<String>,
    pub(crate) data_type: Option<String>,
    pub(crate) size: Option<usize>,
    pub(crate) offset: Option<usize>,
    pub(crate) param: Option<u32>,
}
impl SchemaFieldBuilder {
    pub fn build(self) -> Result<SchemaField, Error> {
        let Some(name) = self.name else {
            return Error::xml_error("Missing name");
        };
        let Some(data_type) = self.data_type else {
            return Error::xml_error("Missing data type");
        };
        let Some(size) = self.size else {
            return Error::xml_error("Missing size");
        };
        let Some(offset) = self.offset else {
            return Error::xml_error("Missing offset");
        };
        Ok(SchemaField {
            name,
            data_type,
            size,
            offset,
            param: self.param,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SchemaContext {
    pub(crate) fields: Vec<SchemaField>,
    pub(crate) struct_size: usize,
}

impl SchemaContext {
    fn new() -> SchemaContext {
        SchemaContext {
            fields: Vec::new(),
            struct_size: 0,
        }
    }
    pub fn new_from_xml(data: &String) -> Result<SchemaContext, Error> {
        let mut buf = BufReader::new(data.as_bytes());
        if data.starts_with("<?xml") {
            let mut throwaway = String::new();
            buf.read_line(&mut throwaway)?;
        }
        let mut reader = xml::EventReader::new(buf);
        let mut schema = SchemaContext::new();
        loop {
            let event = reader.next()?;
            match event {
                XmlEvent::EndDocument => {
                    break;
                }
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _namespace,
                } => {
                    trace!("found start element: {}", name.local_name);
                    if name.local_name != "SdfFieldDefinition" {
                        continue;
                    }
                    let mut bldr = SchemaFieldBuilder::default();

                    for attr in attributes {
                        trace!("found attribute: {}", attr.name.local_name);
                        match attr.name.local_name.as_str() {
                            "type" => {
                                bldr.data_type = Some(attr.value.clone());
                            }
                            "name" => {
                                bldr.name = Some(attr.value.clone());
                            }
                            "size" => {
                                let val = usize::from_str(attr.value.as_str())?;
                                bldr.size = Some(val);
                            }
                            "offset" => {
                                let val = usize::from_str(attr.value.as_str())?;
                                bldr.offset = Some(val);
                            }
                            "param" => {
                                let val = u32::from_str(attr.value.as_str())?;
                                bldr.param = Some(val);
                            }
                            _ => {}
                        }
                    }

                    schema.fields.push(bldr.build()?);
                }
                _ => {}
            }
        }

        for field in &schema.fields {
            schema.struct_size += field.size;
        }
        Ok(schema)
    }
}
