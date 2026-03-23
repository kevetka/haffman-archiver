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

    pub fn new_internal(left: Node, right: Node) -> Self {
        Node {
            weight: left.weight + right.weight,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

pub fn build_huffman_tree(frequencies: [u64; 256]) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (byte, &freq) in frequencies.iter().enumerate() {
        if freq > 0 {
            let node = Node::new_leaf(byte as u8, freq);
            heap.push(Box::new(node));
        }
    }

    if heap.is_empty() {
        return None;
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let parent = Node::new_internal(*left, *right);

        heap.push(Box::new(parent));
    }

    Some(heap.pop().unwrap())
}