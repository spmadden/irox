// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::io::Read;

use xml::reader::XmlEvent;

use crate::GPX;

impl crate::GPX {
    pub fn read_from<T: Read>(input: T) -> Result<GPX, std::io::Error> {
        let mut reader = xml::reader::EventReader::new(input);

        match reader.next()? {
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::EndDocument => {}
            XmlEvent::ProcessingInstruction { .. } => {}
            XmlEvent::StartElement { .. } => {}
            XmlEvent::EndElement { .. } => {}
            XmlEvent::CData(_) => {}
            XmlEvent::Comment(_) => {}
            XmlEvent::Characters(_) => {}
            XmlEvent::Whitespace(_) => {}
        }
        todo!()
    }
}
