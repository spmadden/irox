// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::BTreeMap;
use std::io::Read;

use log::debug;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;
use xml::EventReader;

use irox_tools::options::MaybeMap;

use crate::error::Error;
use crate::types::DataTypeBinding;

#[derive(Debug, Clone, Default)]
pub struct Schema {
    pub elements: Vec<SchemaElement>,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum SchemaElement {
    Element(Element),
    ComplexType(ComplexType),
    SimpleType(SimpleType),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Use {
    #[default]
    Optional,

    Prohibited,
    Required,
}

#[derive(Debug, Clone, Default)]
pub enum Defaults {
    #[default]
    None,

    Fixed(String),
    Default(String),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub required: bool,
    pub data_type: DataTypeBinding,
    pub defaults: Defaults,
}
#[derive(Debug, Clone)]
pub struct SimpleType {
    pub name: String,
    pub data_type: DataTypeBinding,
}

#[derive(Debug, Clone)]
pub struct ComplexType {
    pub name: String,
}

impl Schema {
    pub fn read_from<T: Read>(input: T) -> Result<Schema, Error> {
        let mut reader = xml::EventReader::new(input);
        let mut schema = Schema::default();
        while let Ok(elem) = reader.next() {
            match elem {
                XmlEvent::EndDocument => break,
                XmlEvent::Whitespace(_) => continue,
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    println!("Start: {name:?}");
                    match name.local_name.as_str() {
                        "element" => schema.elements.push(Element::create_from(
                            &mut reader,
                            attributes,
                            namespace,
                        )),
                        "complexType" => schema
                            .elements
                            .push(ComplexType::create_from(&mut reader, attributes)),
                        "simpleType" => schema
                            .elements
                            .push(SimpleType::create_from(&mut reader, attributes)),
                        _ => {}
                    }
                }
                e => println!("{e:?}"),
            }
        }

        debug!("{schema:#?}");
        Ok(schema)
    }
}

impl Element {
    pub(crate) fn create_from<T: Read>(
        reader: &mut EventReader<T>,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> SchemaElement {
        let mut name = get_field_from_attr(&attributes, "name").unwrap_or_default();

        consume_rest(reader);

        SchemaElement::Element(Element { name })
    }
}

impl SimpleType {
    pub(crate) fn create_from<T: Read>(
        reader: &mut EventReader<T>,
        attributes: Vec<OwnedAttribute>,
    ) -> SchemaElement {
        let name = get_field_from_attr(&attributes, "name").unwrap_or_default();
        let data_type: DataTypeBinding = get_field_from_attr(&attributes, "type")
            .maybe_map(|e| DataTypeBinding::try_from(e.as_str()).ok())
            .unwrap_or_default();
        consume_rest(reader);

        SchemaElement::SimpleType(SimpleType { name, data_type })
    }
}

impl ComplexType {
    pub(crate) fn create_from<T: Read>(
        reader: &mut EventReader<T>,
        attributes: Vec<OwnedAttribute>,
    ) -> SchemaElement {
        let name = get_field_from_attr(&attributes, "name").unwrap_or_default();

        consume_rest(reader);

        SchemaElement::ComplexType(ComplexType { name })
    }
}

fn get_field_from_attr(attrs: &Vec<OwnedAttribute>, search: &'static str) -> Option<String> {
    for attr in attrs {
        if attr.name.local_name == search {
            return Some(attr.value.clone());
        }
    }
    None
}

fn consume_rest<T: Read>(reader: &mut EventReader<T>) {
    let mut waiting_for: u32 = 1;
    while let Ok(elem) = reader.next() {
        match elem {
            XmlEvent::StartElement { .. } => {
                waiting_for += 1;
            }
            XmlEvent::EndElement { .. } => {
                waiting_for -= 1;
            }
            _ => {}
        }
        if waiting_for == 0 {
            break;
        }
    }
}
