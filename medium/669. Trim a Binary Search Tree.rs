use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node)= root {
            let mut node = node.replace(TreeNode::new(0));
            if node.val < low {
                return Self::trim_bst(node.right, low, high);    
            }
            if node.val > high {
                return Self::trim_bst(node.left, low, high);
            }
            node.left = Self::trim_bst(node.left, low, high);
            node.right = Self::trim_bst(node.right, low, high);
            return Some(Rc::new(RefCell::new(node)));
        }
        None
    }
}
