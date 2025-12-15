// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use irox_tools::identifier::Identifier;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub struct Descriptor {
    pub id: Identifier,
    pub description: String,
    pub attrs: BTreeMap<String, String>,
}
impl Hash for Descriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

pub type SharedDescriptor = Arc<Mutex<Descriptor>>;

macro_rules! impl_descriptor {
    ($name:ident, $shname: ident) => {
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $name(pub Descriptor);
        impl Deref for $name {
            type Target = Descriptor;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        pub type $shname = Arc<Mutex<$name>>;
    };
}

impl_descriptor!(EdgeDescriptor, SharedEdgeDescriptor);
impl_descriptor!(NodeDescriptor, SharedNodeDescriptor);
impl_descriptor!(FlowDescriptor, SharedFlowDescriptor);
