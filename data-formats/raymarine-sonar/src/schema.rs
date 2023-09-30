use std::collections::VecDeque;

use irox_types::{NamedPrimitive, Primitives};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaField {
    pub(crate) field: NamedPrimitive,
    pub(crate) size: usize,
}

impl SchemaField {
    pub fn new(name: &'static str, prim: Primitives, size: usize) -> SchemaField {
        SchemaField {
            field: NamedPrimitive::new(String::from(name), prim),
            size,
        }
    }
}

pub struct SchemaFieldMem {
    pub(crate) field: SchemaField,
    pub(crate) points: Vec<SchemaEntryMem>,
}

impl SchemaFieldMem {
    pub fn new(n_points: usize, field: SchemaField) -> SchemaFieldMem {
        let points = (0..n_points).map(|i| SchemaEntryMem::default()).collect();
        SchemaFieldMem { points, field }
    }
}

pub struct SchemaWorkingMem {
    pub(crate) fields: Vec<SchemaFieldMem>,
}

impl SchemaWorkingMem {
    pub fn new(n_points: usize, context: &SchemaContext) -> SchemaWorkingMem {
        let fields = context
            .fields
            .iter()
            .map(|f| SchemaFieldMem::new(n_points, f.clone()))
            .collect();
        SchemaWorkingMem { fields }
    }
}

#[derive(Default)]
pub struct SchemaEntryMem {
    pub(crate) mem: VecDeque<u8>,
}

#[derive(Debug, Clone)]
pub struct SchemaContext {
    pub(crate) fields: Vec<SchemaField>,
    pub(crate) struct_size: usize,
}

impl Default for SchemaContext {
    fn default() -> Self {
        SchemaContext {
            fields: vec![
                SchemaField::new("x", Primitives::f32, 4),
                SchemaField::new("y", Primitives::f32, 4),
                SchemaField::new("z", Primitives::f32, 4),
                SchemaField::new("t", Primitives::u32, 4),
                SchemaField::new("sog_kn", Primitives::f32, 4),
                SchemaField::new("water_speed_kn", Primitives::f32, 4),
            ],
            struct_size: 24,
        }
    }
}
