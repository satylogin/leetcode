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
use std::borrow::Borrow;

impl Solution {
    fn update_node(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        let mut to_return = 0;
        if let Some(node) = node.borrow() {
            let mut node = node.as_ref().borrow_mut();
            let rval = Self::update_node(&node.right, val);
            let lval = Self::update_node(&node.left, val + rval + node.val);
            to_return = rval + lval + node.val;
            node.val += rval + val;
        } 
        to_return
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::update_node(&root, 0);
        root
    }
}
