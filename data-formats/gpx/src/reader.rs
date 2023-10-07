// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use log::info;
use std::io::Read;

use crate::error::Error;
use xml::reader::XmlEvent;

use crate::GPX;

impl crate::GPX {
    pub fn read_from<T: Read>(input: T) -> Result<GPX, Error> {
        let mut reader = xml::reader::EventReader::new(input);

        let gpx = GPX::new();
        while let Ok(elem) = reader.next() {
            match elem {
                XmlEvent::EndDocument => break,
                XmlEvent::StartElement { .. } => info!("start_element"),
                _ => {}
            }
        }
        Ok(gpx)
    }
}
