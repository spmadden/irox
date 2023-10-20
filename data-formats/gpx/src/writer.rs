// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::borrow::Cow;
use std::clone::Clone;
use std::io::Write;

use xml::common::XmlVersion;
use xml::writer::XmlEvent;
use xml::{EmitterConfig, EventWriter};

use irox_time::format::iso8601::BASIC_DATE_TIME_OF_DAY;

use crate::error::Error;
use crate::{Track, Waypoint, GPX};

#[derive(Default, Copy, Clone)]
pub struct GPXWriter {
    pub pretty_print: bool,
}

macro_rules! maybe_write_val {
    ($writer:ident, $name:literal, $value:expr) => {
        if let Some(value) = $value {
            $writer.write(XmlEvent::start_element($name))?;
            $writer.write(XmlEvent::characters(format!("{}", value).as_str()))?;
            $writer.write(XmlEvent::end_element())?;
        }
    };
}
macro_rules! write_link {
    ($writer:ident, $value:expr) => {
        for link in $value {
            $writer.write(XmlEvent::start_element("link").attr("href", &link.href))?;
            maybe_write_val!($writer, "text", &link.text);
            maybe_write_val!($writer, "link_type", &link.link_type);
            $writer.write(XmlEvent::end_element())?;
        }
    };
}
macro_rules! info_block {
    ($writer:ident, $value:expr) => {
        maybe_write_val!($writer, "name", &$value.name);
        maybe_write_val!($writer, "cmt", &$value.cmt);
        maybe_write_val!($writer, "desc", &$value.desc);
        maybe_write_val!($writer, "src", &$value.src);
        write_link!($writer, &$value.link);
    };
}

impl GPXWriter {
    pub fn new() -> GPXWriter {
        GPXWriter::default()
    }
    pub fn write_to<T: Write>(&self, data: &GPX, out: &mut T) -> Result<(), Error> {
        let config = EmitterConfig {
            perform_indent: self.pretty_print,
            indent_string: Cow::Borrowed("\t"),
            ..EmitterConfig::new()
        };
        let mut writer = config.create_writer(out);

        writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: None,
        })?;

        writer.write(
            XmlEvent::start_element("gpx")
                .attr("version", data.version.as_str())
                .attr("creator", data.creator.as_str())
                .default_ns(crate::NAMESPACE),
        )?;

        if !data.trk.is_empty() {
            GPXWriter::write_trk(&mut writer, &data.trk)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }

    fn write_wpt<T: Write>(
        writer: &mut EventWriter<T>,
        name: &'static str,
        wpt: &Waypoint,
    ) -> Result<(), Error> {
        writer.write(
            XmlEvent::start_element(name)
                .attr(
                    "lat",
                    format!("{}", &wpt.lat.0.as_degrees().value()).as_str(),
                )
                .attr(
                    "lon",
                    format!("{}", &wpt.lon.0.as_degrees().value()).as_str(),
                ),
        )?;

        maybe_write_val!(writer, "ele", &wpt.ele);
        maybe_write_val!(
            writer,
            "time",
            &wpt.time
                .as_ref()
                .map(|v| { v.format(&BASIC_DATE_TIME_OF_DAY) })
        );

        maybe_write_val!(
            writer,
            "magvar",
            &wpt.magvar.map(|v| v.as_degrees().value())
        );
        maybe_write_val!(
            writer,
            "geoidheight",
            &wpt.geoidheight.map(|v| v.value().as_meters().value())
        );
        info_block!(writer, wpt);
        maybe_write_val!(writer, "sym", &wpt.sym);
        maybe_write_val!(writer, "type", &wpt.wpt_type);
        maybe_write_val!(writer, "fix", &wpt.fix);
        maybe_write_val!(writer, "sat", &wpt.sat);
        maybe_write_val!(writer, "hdop", &wpt.hdop);
        maybe_write_val!(writer, "vdop", &wpt.vdop);
        maybe_write_val!(writer, "pdop", &wpt.pdop);
        maybe_write_val!(
            writer,
            "ageofdgpsdata",
            &wpt.ageofdgpsdata
                .as_ref()
                .map(irox_units::units::duration::Duration::as_seconds_f64)
        );
        maybe_write_val!(writer, "dgpsid", &wpt.dgpsid.as_ref().map(|v| v.0));
        // TODO: extensions

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    fn write_trk<T: Write>(writer: &mut EventWriter<T>, tracks: &Vec<Track>) -> Result<(), Error> {
        for track in tracks {
            writer.write(XmlEvent::start_element("trk"))?;
            info_block!(writer, track);
            maybe_write_val!(writer, "number", &track.number);
            maybe_write_val!(writer, "type", &track.trk_type);
            //  TODO: extensions
            for seg in &track.trkseg {
                writer.write(XmlEvent::start_element("trkseg"))?;

                for pnt in &seg.track_point {
                    GPXWriter::write_wpt(writer, "trkpt", pnt)?;
                }
                // TODO: extensions

                writer.write(XmlEvent::end_element())?;
            }
            writer.write(XmlEvent::end_element())?;
        }
        Ok(())
    }
}
