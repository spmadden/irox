// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use crate::{Edge, Graph, Node, Path, PathItem, SharedEdgeIdentifier, SharedNodeIdentifier};
use alloc::collections::VecDeque;
use std::collections::HashMap;

pub trait Walker {
    fn previsit_node(&mut self, node: &Node, current_path: &Path);
    fn walk_edge(&mut self, edge: &Edge, current_path: &Path);
    fn postvisit_node(&mut self, node: &Node, current_path: &Path);
}
pub trait WalkerMut {
    fn previsit_node(&mut self, node: &mut Node, current_path: &Path);
    fn walk_edge(&mut self, edge: &mut Edge, current_path: &Path);
    fn postvisit_node(&mut self, node: &mut Node, current_path: &Path);
}
pub trait GraphWalk {
    fn walk<T: Walker>(&mut self, walker: &mut T);
}

pub struct BreadthWalk<'a> {
    pub graph: &'a Graph,
    pub next_nodes: VecDeque<SharedNodeIdentifier>,
    pub visited_nodes: Vec<SharedNodeIdentifier>,
    pub visited_edges: Vec<SharedEdgeIdentifier>,
}
impl<'a> BreadthWalk<'a> {
    pub fn new(graph: &'a Graph, start_node: SharedNodeIdentifier) -> Self {
        Self {
            graph,
            next_nodes: VecDeque::from([start_node]),
            visited_nodes: Vec::new(),
            visited_edges: Vec::new(),
        }
    }
}
impl GraphWalk for BreadthWalk<'_> {
    fn walk<T: Walker>(&mut self, walker: &mut T) {
        let mut path_map = HashMap::<SharedNodeIdentifier, Path>::new();
        while !self.next_nodes.is_empty() {
            let Some(node_id) = self.next_nodes.pop_front() else {
                continue;
            };
            if self.visited_nodes.contains(&node_id) {
                continue;
            }
            let Some(v) = self.graph.nodes.get(&node_id) else {
                continue;
            };
            let path = path_map
                .get(&node_id)
                .cloned()
                .unwrap_or_else(|| Path::from_start(node_id.clone()));
            v.get(|n| {
                walker.previsit_node(n, &path);
                for e in &n.navigable_edges {
                    if self.visited_edges.contains(e) {
                        continue;
                    }
                    let Some(edge) = self.graph.edges.get(e) else {
                        continue;
                    };
                    let mut path = path.clone();
                    path.steps.push(PathItem::Edge(e.clone()));
                    edge.read(|edge| {
                        walker.walk_edge(edge, &path);
                    });
                    self.visited_edges.push(e.clone());

                    let Some(e2) = edge.opposite_side_of(&node_id) else {
                        continue;
                    };
                    path.steps.push(PathItem::Node(e2.clone()));
                    let Some(e2) = self.graph.nodes.get(&e2) else {
                        continue;
                    };
                    let Some(e2) = e2.id() else {
                        continue;
                    };
                    path_map.insert(e2.clone().into(), path);
                    self.next_nodes.push_back(e2.into());
                }
                walker.postvisit_node(n, &path);
            });
            self.visited_nodes.push(node_id);
        }
    }
}

pub struct DepthWalk<'a> {
    pub graph: &'a Graph,
    pub next_nodes: VecDeque<SharedNodeIdentifier>,
    pub visited_nodes: Vec<SharedNodeIdentifier>,
    pub visited_edges: Vec<SharedEdgeIdentifier>,
}
impl<'a> DepthWalk<'a> {
    pub fn new(graph: &'a Graph, start_node: SharedNodeIdentifier) -> Self {
        Self {
            graph,
            next_nodes: VecDeque::from([start_node]),
            visited_nodes: Vec::new(),
            visited_edges: Vec::new(),
        }
    }
}
impl GraphWalk for DepthWalk<'_> {
    fn walk<T: Walker>(&mut self, _walker: &mut T) {
        todo!()
    }
}

pub struct WalkerPrinting;
#[allow(clippy::print_stdout)]
impl Walker for WalkerPrinting {
    fn previsit_node(&mut self, node: &Node, current_path: &Path) {
        println!(
            "previsit_node: {}, path: {current_path}",
            *node.descriptor.id
        )
    }

    fn walk_edge(&mut self, edge: &Edge, current_path: &Path) {
        println!("walk_edge: {}, path: {current_path}", edge.id())
    }

    fn postvisit_node(&mut self, node: &Node, current_path: &Path) {
        println!(
            "postvisit_node: {}, path: {current_path}",
            *node.descriptor.id
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::walk::{BreadthWalk, GraphWalk, WalkerPrinting};
    use crate::{Edge, EdgeDescriptor, Graph, Node};
    use irox_tools::identifier::Identifier;

    fn get_test_graph() -> Result<Graph, String> {
        let mut graph = Graph::default();
        graph.add_node(Node::from_id("r".into()).into())?;
        graph.add_node(Node::from_id("s".into()).into())?;
        graph.add_node(Node::from_id("t".into()).into())?;
        graph.add_node(Node::from_id("u".into()).into())?;
        graph.add_node(Node::from_id("v".into()).into())?;
        graph.add_node(Node::from_id("w".into()).into())?;
        graph.add_node(Node::from_id("x".into()).into())?;
        graph.add_node(Node::from_id("y".into()).into())?;
        graph.add_node(Node::from_id("z".into()).into())?;

        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("X-Z").into()),
                left: Identifier::from("x").into(),
                right: Identifier::from("z").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("S-R").into()),
                left: Identifier::from("s").into(),
                right: Identifier::from("r").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("S-V").into()),
                left: Identifier::from("s").into(),
                right: Identifier::from("v").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("S-U").into()),
                left: Identifier::from("s").into(),
                right: Identifier::from("u").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("R-T").into()),
                left: Identifier::from("r").into(),
                right: Identifier::from("t").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("T-U").into()),
                left: Identifier::from("t").into(),
                right: Identifier::from("u").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("U-Y").into()),
                left: Identifier::from("u").into(),
                right: Identifier::from("y").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("R-W").into()),
                left: Identifier::from("r").into(),
                right: Identifier::from("w").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("W-X").into()),
                left: Identifier::from("w").into(),
                right: Identifier::from("x").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("W-V").into()),
                left: Identifier::from("w").into(),
                right: Identifier::from("v").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("W-Z").into()),
                left: Identifier::from("w").into(),
                right: Identifier::from("z").into(),
            }
            .into(),
        )?;
        graph.add_edge(
            Edge::Undirected {
                descriptor: EdgeDescriptor(Identifier::from("Y-X").into()),
                left: Identifier::from("y").into(),
                right: Identifier::from("x").into(),
            }
            .into(),
        )?;
        Ok(graph)
    }

    #[test]
    pub fn test_bfs() -> Result<(), String> {
        let graph = get_test_graph()?;

        let mut walk = BreadthWalk::new(&graph, Identifier::from("s").into());
        walk.walk(&mut WalkerPrinting);

        Ok(())
    }
}
