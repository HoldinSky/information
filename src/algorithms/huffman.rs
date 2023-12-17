use super::create_quantity_map;
use crate::types::{FileStats, Node, Quantity};
use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

pub fn encode(stats: FileStats) -> HashMap<u8, Vec<u8>> {
    let probs = create_quantity_map(&stats.0);
    let mut min_max_heap: MinMaxHeap<Node<Quantity>> = MinMaxHeap::new();

    for probability in probs {
        if probability.quantity != 0 {
            min_max_heap.push(Node::new(probability));
        }
    }

    // actual encoding
    enc(min_max_heap)
}

fn enc(mut min_heap: MinMaxHeap<Node<Quantity>>) -> HashMap<u8, Vec<u8>> {
    let mut codes = HashMap::new();

    while min_heap.len() > 1 {
        let left_node = min_heap.pop_min().unwrap();
        let right_node = min_heap.pop_min().unwrap();

        let mut new_node = Node::new(Quantity {
            byte: 0,
            quantity: left_node.val.quantity + right_node.val.quantity,
        });

        new_node.push_left(left_node);
        new_node.push_right(right_node);

        min_heap.push(new_node);
    }

    let mut root = min_heap.pop_min().unwrap();

    if let Some(mut left_subtree) = root.left.take() {
        let mut vec = vec![0];
        walk_tree_with_action(&mut *left_subtree, &mut vec, &mut codes);
    };
    if let Some(mut right_subtree) = root.right.take() {
        let mut vec = vec![1];
        walk_tree_with_action(&mut *right_subtree, &mut vec, &mut codes);
    };

    codes
}

fn walk_tree_with_action(
    root: &mut Node<Quantity>,
    codes_array: &mut Vec<u8>,
    codes_map: &mut HashMap<u8, Vec<u8>>,
) {
    if root.left.is_none() && root.right.is_none() {
        codes_map.insert(root.val.byte, codes_array.clone());

        codes_array.pop();
        return;
    }

    if let Some(mut left_subtree) = root.left.take() {
        codes_array.push(0);
        walk_tree_with_action(&mut *left_subtree, codes_array, codes_map);
    };
    if let Some(mut right_subtree) = root.right.take() {
        codes_array.push(1);
        walk_tree_with_action(&mut *right_subtree, codes_array, codes_map);
    };

    codes_array.pop();
}
