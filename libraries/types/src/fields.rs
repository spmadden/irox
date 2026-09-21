// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use alloc::string::{String, ToString};

pub struct Field<T: Clone + Default> {
    pub descriptor: FieldDescriptor<T>,
    pub value: Option<T>,
}
impl<T: Clone + Default> Field<T> {
    pub fn new<S: AsRef<str>>(descriptor: FieldDescriptor<T>) -> Self {
        Self {
            descriptor,
            value: None,
        }
    }
    pub fn new_with<S: AsRef<str>>(descriptor: FieldDescriptor<T>, value: T) -> Self {
        Self {
            descriptor,
            value: Some(value),
        }
    }
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_valid(&self) -> bool {
        let nullable = self.descriptor.nullable;
        let enforce_nn = self.descriptor.enforce_not_null;
        let is_null = self.value.is_none();
        match (nullable, enforce_nn, is_null) {
            // nullable - all values valid
            (true, _, _) => true,
            // enforce value, is null - not valid
            (false, true, true) => false,
            // enforce value, not null - valid
            (false, true, false) => true,

            // not nullable, no enforcement - valid.
            (false, false, _) => true,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum StringType {
    #[default]
    NoRestrictions,
    UniqueValue,
    RestrictedValues,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldType {
    String(StringType),
    Boolean,
    Integer,
    FloatingPoint,
    DateTimeString,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDescriptor<T: Clone + Default> {
    pub name: String,
    pub display_name: String,
    pub help_description: Option<String>,
    pub nullable: bool,
    pub enforce_not_null: bool,
    pub default_value: T,
}
impl<T: Clone + Default> FieldDescriptor<T> {
    #[must_use]
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDescriptorBuilder<T: Clone + Default> {
    pub name: String,
    pub display_name: Option<String>,
    pub help_description: Option<String>,
    pub nullable: bool,
    pub enforce_not_null: bool,
    pub default_value: T,
}

impl<T: Clone + Default> FieldDescriptorBuilder<T> {
    pub fn new<S: AsRef<str>>(name: &S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            display_name: None,
            help_description: None,
            nullable: true,
            enforce_not_null: false,
            default_value: T::default(),
        }
    }
    #[must_use]
    pub fn with_display_name<S: AsRef<str>>(mut self, display_name: &S) -> Self {
        self.display_name = Some(display_name.as_ref().to_string());
        self
    }
    #[must_use]
    pub fn with_help_description<S: AsRef<str>>(mut self, help_description: &S) -> Self {
        self.help_description = Some(help_description.as_ref().to_string());
        self
    }
    #[must_use]
    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
    #[must_use]
    pub fn with_enforce_not_null(mut self, enforce_not_null: bool) -> Self {
        self.enforce_not_null = enforce_not_null;
        self
    }
    #[must_use]
    pub fn with_default_value(mut self, value: T) -> Self {
        self.default_value = value;
        self
    }

    #[must_use]
    pub fn build(self) -> FieldDescriptor<T> {
        FieldDescriptor {
            display_name: self.display_name.unwrap_or(self.name.clone()),
            help_description: self.help_description,
            name: self.name,
            nullable: self.nullable,
            enforce_not_null: self.enforce_not_null,
            default_value: self.default_value,
        }
    }
}
