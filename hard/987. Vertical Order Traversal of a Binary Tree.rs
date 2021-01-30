// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }


use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, BTreeMap, BinaryHeap};

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue: VecDeque<((i32, i32), Option<Rc<RefCell<TreeNode>>>)> = VecDeque::new();
        let mut elements: BTreeMap<i32, BinaryHeap<(i32, i32)>> = BTreeMap::new();
        queue.push_back(((0, 0), root));
        while let Some(((x, y), node)) = queue.pop_front() {
            if let Some(node) = node {
                let node = node.replace(TreeNode::new(0));
                elements.entry(x).or_insert(BinaryHeap::new()).push((y, -node.val));
                queue.push_back(((x - 1, y - 1), node.left));
                queue.push_back(((x + 1, y - 1), node.right));
            }
        }
        elements.into_iter().map(|(_, mut h)| {
            (0..h.len()).map(|_| -h.pop().unwrap().1).collect()
        }).collect()
    }
}
