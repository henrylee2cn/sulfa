//! Traverse the binary tree

use std::borrow::Borrow;
use std::cell::RefCell;

use super::node::OptBinaryNode;

#[derive(Clone, Copy)]
pub enum Order {
    InOrder,
    PreOrder,
    PostOrder,
}

pub fn bfs<T: Clone>(root: &OptBinaryNode<T>, order: Order) -> Vec<T> {
    let mut stack: Vec<OptBinaryNode<T>> = Vec::new();
    let mut result: Vec<T> = Vec::new();
    if root.is_some() { stack.push(root.clone()) }
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if node.is_some() {
            let ref_node = RefCell::borrow(node.as_ref().unwrap());
            match order {
                Order::InOrder => {
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    stack.push(node.clone());
                    stack.push(None);
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                }
                Order::PreOrder => {
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                    stack.push(node.clone());
                    stack.push(None);
                }
                Order::PostOrder => {
                    stack.push(node.clone());
                    stack.push(None);
                    if ref_node.right.is_some() {
                        stack.push(ref_node.right.clone())
                    }
                    if ref_node.left.is_some() {
                        stack.push(ref_node.left.clone())
                    }
                }
            }
        } else {
            let node = stack.pop().unwrap();
            result.push(RefCell::borrow(node.unwrap().borrow()).val.clone())
        }
    }
    result
}

pub fn level_bfs<T: Clone>(root: &OptBinaryNode<T>) -> Vec<Vec<T>> {
    let mut queue: Vec<OptBinaryNode<T>> = Vec::new();
    let mut result: Vec<Vec<T>> = Vec::new();
    if root.is_some() { queue.push(root.clone()) }
    while !queue.is_empty() {
        let mut level: Vec<T> = Vec::new();
        for _ in 0..queue.len() {
            let node = queue.remove(0);
            let node = RefCell::borrow(node.as_ref().unwrap());
            level.push(node.val.clone());
            if node.left.is_some() {
                queue.push(node.left.clone())
            }
            if node.right.is_some() {
                queue.push(node.right.clone())
            }
        }
        result.push(level);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tree::{BinaryNode, OptBinaryNode};
    use crate::tree::order::{bfs, level_bfs, Order};

    fn tree() -> OptBinaryNode<i32> {
        let mut tn1 = BinaryNode::new(1);
        let mut tn3 = BinaryNode::new(3);
        let tn4 = BinaryNode::new(4);
        let tn2 = BinaryNode::new(2);
        tn3.right = Some(Rc::new(RefCell::new(tn2)));
        tn1.left = Some(Rc::new(RefCell::new(tn3)));
        tn1.right = Some(Rc::new(RefCell::new(tn4)));
        let root = Some(Rc::new(RefCell::new(tn1)));
        root
    }

    #[test]
    fn inorder_bfs() {
        assert_eq!(2 + 2, 4);
        let root = tree();
        let res = bfs(&root, Order::InOrder);
        println!("inorder_bfs:=================={:?}", res);
    }

    #[test]
    fn levelorder_bfs() {
        assert_eq!(2 + 2, 4);
        let root = tree();
        let res = level_bfs(&root);
        println!("levelorder_bfs:=================={:?}", res);
    }
}
