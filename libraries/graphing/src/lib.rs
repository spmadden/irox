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
mod flow;
mod graph;
mod node;
mod paths;

pub use descriptor::*;
pub use edge::*;
pub use flow::*;
pub use graph::*;
pub use node::*;
pub use paths::*;
