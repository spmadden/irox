pub mod local;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegistryPackage {
    pub(crate) name: String,
    pub(crate) vers: String,
    pub(crate) deps: Vec<RegistryDependency>,
    pub(crate) cksum: String,
    pub(crate) features: BTreeMap<String, Vec<String>>,
    pub(crate) yanked: Option<bool>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RegistryDependency {
    pub(crate) name: String,
    pub(crate) req: String,
    pub(crate) features: Vec<String>,
    pub(crate) optional: bool,
    pub(crate) default_features: bool,
    pub(crate) target: Option<String>,
    pub(crate) kind: Option<String>,
    pub(crate) package: Option<String>,
}

pub trait Registry {}
