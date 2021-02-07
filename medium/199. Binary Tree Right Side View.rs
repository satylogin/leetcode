use std::rc::Rc;
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let n = queue.len();
            let mut v: i32 = std::i32::MIN;
            for _ in 0..n {
                let node = queue.pop_front().unwrap().unwrap().replace(TreeNode::new(0));
                v = node.val;
                if node.left.is_some() {
                    queue.push_back(node.left);
                }
                if node.right.is_some() {
                    queue.push_back(node.right);
                }
            }
            ans.push(v);
        }
        ans
    }
}
