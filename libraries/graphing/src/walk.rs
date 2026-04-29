// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;

use crate::{Edge, Graph, Node, Path, PathItem, SharedEdgeIdentifier, SharedNodeIdentifier};
use alloc::collections::VecDeque;
use std::collections::HashMap;

pub trait Walker {
    fn previsit_node(&mut self, _node: &Node, _current_path: &Path) {}
    fn walk_edge(&mut self, _edge: &Edge, _current_path: &Path) {}
    fn postvisit_node(&mut self, _node: &Node, _current_path: &Path) {}
}
pub trait WalkerMut {
    fn previsit_node(&mut self, _node: &mut Node, _current_path: &Path) {}
    fn walk_edge(&mut self, _edge: &mut Edge, _current_path: &Path) {}
    fn postvisit_node(&mut self, _node: &mut Node, _current_path: &Path) {}
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
        while let Some(start_node) = self.next_nodes.pop_front() {
            let mut stack = Vec::<SharedEdgeIdentifier>::new();
            let mut path = Path::from_start(start_node.clone());

            // first iteration at the root
            let Some(node) = self.graph.nodes.get(&start_node).cloned() else {
                continue;
            };
            node.get(|node| {
                for edge in &node.navigable_edges {
                    stack.push(edge.clone());
                }
                _walker.previsit_node(node, &path);
            });

            while let Some(current_edge) = stack.pop() {
                let Some(edge) = self.graph.edges.get(&current_edge) else {
                    continue;
                };
                let Some((_left, right)) = edge.get_sides() else {
                    continue;
                };
                path.steps.push(PathItem::Edge(current_edge.clone()));
                edge.get(|edge| {
                    _walker.walk_edge(edge, &path);
                });
                path.steps.push(PathItem::Node(right.clone()));

                let Some(node) = self.graph.nodes.get(&start_node).cloned() else {
                    continue;
                };
                node.get(|node| {
                    for edge in &node.navigable_edges {
                        stack.push(edge.clone());
                    }
                    _walker.previsit_node(node, &path);
                });
            }
        }
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

pub struct PrevisitNodeWalker<T: FnMut(&Node, &Path)> {
    func: Box<T>,
}
impl<T: FnMut(&Node, &Path)> PrevisitNodeWalker<T> {
    pub fn new(func: T) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}
impl<T: FnMut(&Node, &Path)> Walker for PrevisitNodeWalker<T> {
    fn previsit_node(&mut self, node: &Node, current_path: &Path) {
        (self.func)(node, current_path)
    }
}

#[derive(Debug, Default, Clone)]
pub struct WalkerCreating {
    graph: Graph,
}
impl WalkerCreating {
    pub fn finish(self) -> Graph {
        self.graph
    }
}
impl Walker for WalkerCreating {
    fn previsit_node(&mut self, node: &Node, _current_path: &Path) {
        let _ = self.graph.add_node(node.clone().into());
    }

    fn walk_edge(&mut self, edge: &Edge, _current_path: &Path) {
        let _ = self.graph.add_edge(edge.into());
    }
}
#[derive(Debug)]
pub struct WalkerAdding<'a> {
    graph: &'a mut Graph,
}
impl<'a> WalkerAdding<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
        Self { graph }
    }
}
impl Walker for WalkerAdding<'_> {
    fn previsit_node(&mut self, node: &Node, _current_path: &Path) {
        let _ = self.graph.add_node(node.clone().into());
    }

    fn walk_edge(&mut self, edge: &Edge, _current_path: &Path) {
        let _ = self.graph.add_edge(edge.into());
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
