// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::NamedPrimitive;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PrimitiveSchema {
    identifier: String,
    variables: Vec<NamedPrimitive>,
    size_bytes: usize,
    varbl_offsets: HashMap<NamedPrimitive, usize>,
}

impl PrimitiveSchema {
    pub fn new(identifier: String, variables: Vec<NamedPrimitive>) -> Self {
        let mut size_bytes = 0;
        let mut varbl_offsets = HashMap::<NamedPrimitive, usize>::with_capacity(variables.len());
        for varbl in &variables {
            varbl_offsets.insert(varbl.clone(), size_bytes);
            size_bytes += varbl.primitive().bytes_length();
        }
        PrimitiveSchema {
            identifier,
            variables,
            size_bytes,
            varbl_offsets,
        }
    }
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
    pub fn get_variables(&self) -> &impl IntoIterator<Item = NamedPrimitive> {
        &self.variables
    }
    pub fn get_offset(&self, varbl: &NamedPrimitive) -> Option<usize> {
        self.varbl_offsets.get(varbl).copied()
    }
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }
}

#[derive(Default, Debug, Clone)]
pub struct PrimitiveSchemaBuilder {
    identifier: Option<String>,
    variables: Vec<NamedPrimitive>,
}

impl PrimitiveSchemaBuilder {
    pub fn add(&mut self, varbl: NamedPrimitive) {
        self.variables.push(varbl);
    }
    pub fn set_name(&mut self, name: &str) {
        self.identifier = Some(name.to_string());
    }
    pub fn build(self) -> PrimitiveSchema {
        PrimitiveSchema::new(self.identifier.unwrap_or_default(), self.variables)
    }
}
