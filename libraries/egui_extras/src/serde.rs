// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! Adds an implementation of [`serde::Serializer`] that draws a [`egui`] debug window containing the contents of the object.

use egui::{CollapsingHeader, Ui};
use std::fmt::{Display, Formatter};
use std::usize;

use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};

use irox_tools::hex::HexDump;

///
/// [`serde::Serializer`] that will draw a series of nested [`egui::CollapsingHeader`]s with the contents of the object
/// for debug purposes.
#[derive(Default, Debug)]
pub struct EguiSerializer {
    values: Vec<Event>,
}

impl EguiSerializer {
    #[must_use]
    pub fn new() -> Self {
        EguiSerializer::default()
    }

    pub fn show(&self, ui: &mut Ui) {
        for event in &self.values {
            event.show(ui);
        }
    }

    pub(crate) fn named(&mut self, named_type: &'static str, name: String) -> Named {
        Named::new(self, named_type, name)
    }
}

#[derive(Debug)]
pub(crate) enum Event {
    Row(String),
    Group {
        name: String,
        values: Vec<Event>,
    },
    Field {
        name: String,
        field_type: &'static str,
        values: Vec<Event>,
    },
}

impl Event {
    pub fn show(&self, ui: &mut Ui) {
        match self {
            Event::Row(s) => {
                ui.label(s);
            }
            Event::Group { name, values } => {
                CollapsingHeader::new(name)
                    .id_source(ui.next_auto_id())
                    .show(ui, |ui| {
                        for event in values {
                            event.show(ui);
                        }
                    });
            }
            Event::Field {
                name,
                field_type,
                values,
            } => {
                if values.is_empty() {
                    ui.label(format!("{field_type} {name}: []"));
                } else if values.len() == 1 {
                    if let Some(some) = values.first() {
                        if let Event::Row(val) = some {
                            ui.label(format!("{field_type} {name}: {val}"));
                        } else {
                            ui.group(|ui| {
                                ui.label(format!("{field_type} {name}"));
                                some.show(ui);
                            });
                        }
                    }
                } else {
                    CollapsingHeader::new(format!("{name}: {field_type}"))
                        .id_source(ui.next_auto_id())
                        .show(ui, |ui| {
                            for val in values {
                                if let Event::Row(val) = val {
                                    ui.label(val);
                                } else {
                                    val.show(ui);
                                }
                            }
                        });
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    message: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {}
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            message: format!("{msg}"),
        }
    }
}

impl<'a> Serializer for &'a mut EguiSerializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Named<'a>;
    type SerializeTupleVariant = Named<'a>;
    type SerializeMap = Mapped<'a>;
    type SerializeStruct = Named<'a>;
    type SerializeStructVariant = Named<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}")));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_i8")));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_i16")));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_i32")));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_i64")));
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_i128")));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_u8")));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_u16")));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_u32")));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_u64")));
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_u128")));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_f32")));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_f64")));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("{v}_chr")));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("\"{v}\"")));
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut val = String::new();
        let _ = v.hexdump_to(&mut val);
        self.values.push(Event::Row(val));
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row("None".to_string()));
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.push(Event::Field {
            field_type: "Option",
            name: "Some".to_string(),
            values: ser.values,
        });
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row("()".to_string()));
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.values.push(Event::Row(format!("struct {name}")));
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.values
            .push(Event::Row(format!("{name}::{variant} = {variant_index}")));
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        let values = ser.values;
        if values.len() > 1 {
            self.values.push(Event::Group {
                name: format!("struct {name}"),
                values,
            });
        } else {
            self.values.push(Event::Field {
                name: format!("struct {name}"),
                field_type: "newtype",
                values,
            });
        }
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.push(Event::Group {
            name: format!("newtype {name}::{variant}"),
            values: ser.values,
        });
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self.named("struct", name.to_string()))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self.named("enum", format!("{name}::{variant}({variant_index})")))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Mapped {
            key: None,
            events: vec![],
            ser: self,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self.named("struct", name.to_string()))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self.named("enum", format!("struct {name}::{variant}({variant_index})")))
    }
}

impl SerializeSeq for &mut EguiSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.append(&mut ser.values);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTuple for &mut EguiSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.append(&mut ser.values);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct Named<'a> {
    ser: &'a mut EguiSerializer,
    name: String,
    named_type: &'static str,
    values: Vec<Event>,
}
impl<'a> Named<'a> {
    pub fn new(ser: &'a mut EguiSerializer, named_type: &'static str, name: String) -> Self {
        Named {
            ser,
            name,
            named_type,
            values: vec![],
        }
    }
    pub fn finish(self) -> Result<(), Error> {
        self.ser.values.push(Event::Field {
            name: self.name,
            field_type: self.named_type,
            values: self.values,
        });
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for Named<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.append(&mut ser.values);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl<'a> SerializeTupleVariant for Named<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        self.values.push(Event::Field {
            name: "unnamed".to_string(),
            field_type: "tuplevariant",
            values: ser.values,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl<'a> SerializeStruct for Named<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer { values: vec![] };
        value.serialize(&mut ser)?;
        self.values.push(Event::Field {
            name: key.to_string(),
            field_type: "field",
            values: ser.values,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl<'a> SerializeStructVariant for Named<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer { values: vec![] };
        value.serialize(&mut ser)?;
        self.values.push(Event::Field {
            name: key.to_string(),
            field_type: "field",
            values: ser.values,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

pub struct Mapped<'a> {
    key: Option<Vec<Event>>,
    events: Vec<Event>,
    ser: &'a mut EguiSerializer,
}

impl<'a> SerializeMap for Mapped<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        key.serialize(&mut ser)?;
        self.key = Some(ser.values);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut ser = EguiSerializer::new();
        value.serialize(&mut ser)?;
        if let Some(key) = self.key.take() {
            self.events.push(Event::Group {
                name: "entry".to_string(),
                values: vec![
                    Event::Group {
                        name: "key".to_string(),
                        values: key,
                    },
                    Event::Field {
                        name: "value".to_string(),
                        field_type: "map value",
                        values: ser.values,
                    },
                ],
            });
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.values.push(Event::Group {
            name: "map".to_string(),
            values: self.events,
        });
        Ok(())
    }
}
