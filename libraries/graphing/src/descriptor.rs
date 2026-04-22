// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use alloc::collections::BTreeMap;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use irox_structs_derive::Shared;
use irox_tools::identifier::{Identifiable, Identifier, SharedIdentifier};

#[derive(Debug, Clone, PartialEq, Eq, Shared)]
pub struct Descriptor {
    pub id: SharedIdentifier,
    pub description: Option<String>,
    pub attrs: BTreeMap<String, String>,
}
impl Identifiable for Descriptor {
    fn id(&self) -> Option<Identifier> {
        Some(self.id.deref().clone())
    }
}
impl Hash for Descriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}
impl From<Identifier> for Descriptor {
    fn from(value: Identifier) -> Self {
        Descriptor {
            id: value.into(),
            description: None,
            attrs: BTreeMap::default(),
        }
    }
}
impl From<SharedIdentifier> for Descriptor {
    fn from(value: SharedIdentifier) -> Self {
        Descriptor {
            id: value,
            description: None,
            attrs: BTreeMap::default(),
        }
    }
}

macro_rules! impl_descriptor {
    ($name:ident, $shname: ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        pub type $shname = alloc::sync::Arc<std::sync::RwLock<$name>>;
    };
}

impl_descriptor!(EdgeDescriptor, SharedEdgeDescriptor);
impl_descriptor!(NodeDescriptor, SharedNodeDescriptor);
impl_descriptor!(FlowDescriptor, SharedFlowDescriptor);
