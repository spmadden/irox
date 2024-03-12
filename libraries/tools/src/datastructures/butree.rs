// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::collections::VecDeque;
use crate::sync::SynchronizedCell;

#[derive(Debug)]
pub(crate) struct BUNode {
    level: u32,
    filled: bool,
    left_zero: SynchronizedCell<BUNode>,
    right_one: SynchronizedCell<BUNode>,
}

impl BUNode {
    pub fn new(level: u32, filled: bool) -> BUNode {
        BUNode {
            level,
            filled,
            left_zero: SynchronizedCell::empty(),
            right_one: SynchronizedCell::empty(),
        }
    }

    pub fn mark_zero(&mut self) {
        if self.filled {
            return;
        }
        let _ = self.left_zero.get_or_init(|| {
            BUNode::new(self.level + 1, false)
        });
        self.check_mark_filled();
    }

    pub fn mark_one(&mut self) {
        if self.filled {
            return;
        }
        let _ = self.right_one.get_or_init(|| {
            BUNode::new(self.level + 1, false)
        });
        self.check_mark_filled();
    }

    pub fn check_mark_filled(&mut self) {
        if self.filled {
            self.left_zero.take();
            self.right_one.take();
            return;
        }
        let mut filled = false;
        self.left_zero.get(|v| {
            filled = v.map(|n|n.filled).unwrap_or_default();
        });
        self.right_one.get(|v| {
            filled &= v.map(|n|n.filled).unwrap_or_default()
        });

        if filled {
            self.filled = true;
            self.left_zero.take();
            self.right_one.take();
        }
    }
}

#[derive(Debug)]
pub struct BinaryUtilizationTree {
    head: SynchronizedCell<BUNode>,
}

impl BinaryUtilizationTree {
    pub fn new() -> Self {
        let tree = BinaryUtilizationTree {
            head: SynchronizedCell::new(BUNode::new(0, false))
        };
        tree
    }

    pub fn mark_utilized(&mut self, path: &[u8]) {
        let mut current : SynchronizedCell<BUNode> = self.head.clone();
        let mut treepath = Vec::new();
        treepath.push(current.clone());

        path.iter().for_each(|b| {
            for shift in (0..=7).rev() {
                if current.maybe_map(|v|v.filled).unwrap_or_default() {
                    return;
                }
                let bit = (*b >> shift) & 1;

                if bit == 1 {
                    current.maybe_mutate(BUNode::mark_one);

                    let Some(one) = current.maybe_map(|v|v.right_one.clone()) else {
                        return;
                    };
                    treepath.push(one.clone());
                    current = one;
                } else {
                    current.maybe_mutate(BUNode::mark_zero);
                    let Some(zero) = current.maybe_map(|v|v.left_zero.clone()) else {
                        return;
                    };
                    treepath.push(zero.clone());
                    current = zero;
                }
            }
        });
        current.maybe_mutate(|v|{
            v.filled = true;
        });
        while let Some(node) = treepath.pop() {
            node.maybe_mutate(BUNode::check_mark_filled)
        }
    }
}

pub struct Value {
    values: Vec<u8>,
    prefix: u32,
}

pub struct Iter {
    to_return: VecDeque<Value>,
    current_address: Vec<SynchronizedCell<BUNode>>,
}

impl Iter {
    fn new(head: SynchronizedCell<BUNode>) -> Self {
        Iter {
            to_return: VecDeque::new(),
            current_address: Vec::new()
        }
    }
}

impl std::iter::Iterator for Iter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::datastructures::BinaryUtilizationTree;

    #[test]
    pub fn test() {
        let mut tree = BinaryUtilizationTree::new();
        println!("{tree:?}");
        tree.mark_utilized(&[0x01]);
        tree.mark_utilized(&[0x00]);
        tree.mark_utilized(&[0xFF]);
        println!("{tree:#?}");
    }

    #[test]
    pub fn test2() {
        let mut tree = BinaryUtilizationTree::new();
        println!("{tree:?}");
        for v in 0..=0xFF {
            tree.mark_utilized(&[v]);
        }
        println!("{tree:#?}");
    }
}