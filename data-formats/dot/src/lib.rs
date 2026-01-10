// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! DOT Graph Description Language writer, compatible with GraphViz
//!
//! ### Example:
//! ```
//! # use irox_dot::*;
//! # use std::fs::File;
//! #
//! # fn main() -> Result<(), irox_bits::Error>{
//!     let mut graph = Graph::named("TestGraph");
//!     graph.graph_type = GraphType::Digraph;
//!
//!     // add a top-level graph attribute
//!     graph.add_graph_attr("landscape", "true");
//!
//!     // add a basic node with no attributes
//!     graph.add_node(Node::new("Node 1"));
//!
//!     // add an edge
//!     graph.add_edge(Edge::new(&graph, "Node 1", "Node 2"));
//!
//!     let mut out = String::with_capacity(256);
//!     graph.write_to(&mut out)?;
//!     println!("{out}");
//!     assert_eq!(out, "\
//!digraph TestGraph {\n\
//!\tlandscape=true\n\
//!\t\"Node 1\" \n\
//!\t\"Node 1\" -> \"Node 2\" \n\
//!}\n"
//! );
//! # Ok(())
//! # }
//! ```
//!

#![forbid(unsafe_code)]

/// Enables feature-specific code.
/// Use this macro instead of `cfg(feature = "drawing")` to generate docs properly.
#[macro_export]
macro_rules! cfg_feature_drawing {
    ($($item:item)*) => {
        $(
            #[cfg(any(all(doc, docsrs), feature = "drawing"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "drawing")))]
            $item
        )*
    }
}

cfg_feature_drawing! {
    pub mod drawing;
}

use irox_bits::{BitsError, FormatBits, MutBits};
use std::collections::HashSet;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
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
            Element::Attribute(a) => a.get_line(),
            _ => {
                todo!()
            }
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Graph {
    pub is_strict: bool,
    pub graph_type: GraphType,
    pub id: Option<String>,
    pub elements: Vec<Element>,
    pub known_nodes: HashSet<String>,
}
impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_strict.hash(state);
        self.graph_type.hash(state);
        self.id.hash(state);
        self.elements.hash(state);
    }
}

impl Graph {
    pub fn named(name: &str) -> Self {
        Self {
            id: Some(name.to_string()),
            ..Default::default()
        }
    }
    pub fn add_node(&mut self, node: Node) {
        self.elements.push(Element::Node(node));
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.elements.push(Element::Edge(edge));
    }
    pub fn add_graph_attr(&mut self, key: &str, val: &str) {
        self.elements
            .push(Element::Attribute(Attribute::new(key, val)))
    }
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
impl Attribute {
    pub fn new(key: &str, val: &str) -> Self {
        Attribute {
            name: key.to_string(),
            value: val.to_string(),
        }
    }
}
impl DotLine for Attribute {
    fn get_line(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
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
                    .map(DotLine::get_line)
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

impl Node {
    pub fn new(id: &str) -> Node {
        Node {
            id: id.to_string(),
            attributes: AttrList::default(),
        }
    }
}

impl DotLine for Node {
    fn get_line(&self) -> String {
        format!("\"{}\" {}", self.id, self.attributes.get_line())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Edge {
    pub edge_type: GraphType,
    pub first_node: String,
    pub second_node: String,
    pub attributes: AttrList,
}
impl Edge {
    pub fn new(graph: &Graph, first: &str, second: &str) -> Self {
        Edge {
            edge_type: graph.graph_type,
            first_node: first.to_string(),
            second_node: second.to_string(),
            attributes: AttrList::default(),
        }
    }
}

impl DotLine for Edge {
    fn get_line(&self) -> String {
        format!(
            "\"{}\" {} \"{}\" {}",
            self.first_node,
            self.edge_type.get_arrow(),
            self.second_node,
            self.attributes.get_line()
        )
    }
}
