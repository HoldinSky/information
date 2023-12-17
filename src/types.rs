use std::fs::File;

// actual structs and types

#[derive(Clone, Copy)]
pub enum CodeType {
    ShannonFano,
    Huffman,
}

/// byte -> its probability
pub type Probability = (u8, f64);

/// file -> its full path
pub type FileInfo = (File, String);

// dictionary -> total count of symbols
pub type FileStats = ([u64; 256], u64);

#[derive(Copy, Clone, Debug)]
pub struct Quantity {
    pub byte: u8,
    pub quantity: u128,
}

pub struct EncodingSettings {
    pub file_info: FileInfo,
    pub code_type: CodeType,
    pub hamming_code_length: Option<u8>,
}

type NodePtr<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Copy> {
    pub val: T,
    pub left: NodePtr<T>,
    pub right: NodePtr<T>,
}

impl<T: Copy> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn push_left(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    pub fn push_right(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }
}

impl<T: Copy> From<Node<T>> for NodePtr<T> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

// implementations on structs

impl Ord for Quantity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quantity.cmp(&other.quantity)
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.quantity.partial_cmp(&other.quantity) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.byte.partial_cmp(&other.byte)
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        self.byte == other.byte && self.quantity == other.quantity
    }
}

impl Eq for Quantity {}

impl<T: Ord + Copy> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T: PartialOrd + Copy> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T: PartialEq + Copy> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Eq + Copy> Eq for Node<T> {}
