use std::{cmp::Ordering, collections::BinaryHeap};

pub struct Node {
    pub weight: u64,
    pub symbol: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new_leaf(symbol: u8, weight: u64) -> Self {
        Node {
            weight,
            symbol: Some(symbol),
            left: None,
            right: None,
        }
    }

    pub fn new_internal(left: Box<Node>, right: Box<Node>) -> Self {
        Node {
            weight: left.weight + right.weight,
            symbol: None,
            left: Some(left),
            right: Some(right),
        }
    }
}

pub fn build_huffman_tree(frequencies: [u64; 256]) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (byte, &freq) in frequencies.iter().enumerate() {
        if freq > 0 {
            heap.push(Box::new(Node::new_leaf(byte as u8, freq)));
        }
    }

    match heap.len() {
        0 => return None,
        1 => {
            let single = heap.pop().unwrap();
            return Some(Box::new(Node {
                weight: single.weight,
                symbol: None,
                left: Some(single),
                right: None,
            }));
        }
        _ => {}
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        heap.push(Box::new(Node::new_internal(left, right)));
    }

    Some(heap.pop().unwrap())
}