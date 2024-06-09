// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! DOT Graph Description Language writer, compatible with GraphViz
//!

#![forbid(unsafe_code)]

use irox_bits::{BitsError, FormatBits, MutBits};
use std::fmt::Write;
use std::string::String;
use std::vec::Vec;

pub trait DotLine {
    fn get_line(&self) -> String;
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GraphType {
    #[default]
    Graph,
    Digraph,
}
impl GraphType {
    pub fn get_arrow(&self) -> &'static str {
        match self {
            GraphType::Graph => "--",
            GraphType::Digraph => "->",
        }
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            GraphType::Graph => "graph",
            GraphType::Digraph => "digraph",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Element {
    Node(Node),
    NodeAttr(Attribute),
    Edge(Edge),
    EdgeAttr(Attribute),
    Attribute(Attribute),
    Subgraph(Subgraph),
}
impl DotLine for Element {
    fn get_line(&self) -> String {
        match self {
            Element::Node(n) => n.get_line(),
            Element::Edge(e) => e.get_line(),
            _ => {
                todo!()
            }
        }
    }
}
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Graph {
    pub is_strict: bool,
    pub graph_type: GraphType,
    pub id: Option<String>,
    pub elements: Vec<Element>,
}
impl Graph {
    pub fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), BitsError> {
        let mut out = FormatBits(out);
        if self.is_strict {
            write!(out, "strict ")?;
        }
        write!(out, "{} ", self.graph_type.get_name())?;
        if let Some(name) = &self.id {
            write!(out, "{name} ")?;
        }
        writeln!(out, "{{")?;
        for elem in &self.elements {
            writeln!(out, "\t{}", elem.get_line())?;
        }
        writeln!(out, "}}")?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Subgraph {
    pub id: Option<String>,
    pub elements: Vec<Element>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AttrList(pub Vec<Attribute>);
impl DotLine for AttrList {
    fn get_line(&self) -> String {
        if self.0.is_empty() {
            String::default()
        } else {
            format!(
                "[{}]",
                self.0
                    .iter()
                    .map(|v| format!("{} = {}", v.name, v.value))
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    pub id: String,
    pub attributes: AttrList,
}
impl DotLine for Node {
    fn get_line(&self) -> String {
        format!("{} {}", self.id, self.attributes.get_line())
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Edge {
    pub edge_type: GraphType,
    pub first_node: String,
    pub second_node: String,
    pub attributes: AttrList,
}
impl DotLine for Edge {
    fn get_line(&self) -> String {
        format!(
            "{} {} {} {}",
            self.first_node,
            self.edge_type.get_arrow(),
            self.second_node,
            self.attributes.get_line()
        )
    }
}
