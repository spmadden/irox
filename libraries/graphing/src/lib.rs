// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Graphs (Nodes/Edges) Algorithms, and Graph Drawing
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod descriptor;
mod edge;
pub mod fdp;
mod flow;
mod graph;
mod node;
mod paths;
cfg_feature_egui! {
    pub mod egui;


}

pub use descriptor::*;
pub use edge::*;
pub use flow::*;
pub use graph::*;
use irox_tools::cfg_feature_egui;
pub use node::*;
pub use paths::*;
