use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

const SEP: char = ';';
const NONE: &'static str = "None";

fn get_node(data: Option<&str>) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_none() {
        return None;
    }
    let data = data.unwrap();
    if data == NONE {
        None
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(
            data.parse::<i32>().unwrap(),
        ))))
    }
}

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);

        let mut ser = String::new();
        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                let node = node.replace(TreeNode::new(0));
                ser += &format!("{}{}", node.val, SEP);
                queue.push_back(node.left);
                queue.push_back(node.right);
            } else {
                ser += &format!("{}{}", NONE, SEP);
            }
        }

        while ser.ends_with(SEP) {
            ser.pop();
            if ser.ends_with(NONE) {
                while ser.len() > 0 && !ser.ends_with(SEP) {
                    ser.pop();
                }
            }
        }

        ser
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }
        let mut splitted = data.split(SEP);
        let root = get_node(splitted.next());
        if root.is_none() {
            return None;
        }
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(Rc::clone(root.as_ref().unwrap()));
        while let Some(par) = queue.pop_front() {
            let node = get_node(splitted.next());
            if node.is_some() {
                par.borrow_mut().left = Some(Rc::clone(node.as_ref().unwrap()));
                queue.push_back(Rc::clone(node.as_ref().unwrap()));
            }
            let node = get_node(splitted.next());
            if node.is_some() {
                par.borrow_mut().right = Some(Rc::clone(node.as_ref().unwrap()));
                queue.push_back(Rc::clone(node.as_ref().unwrap()));
            }
        }
        root
    }
}

#[test]
fn test() {
    let codec = Codec::new();
    for expr in vec!["1;2;3;None;None;4;5", "", "1", "1;2"] {
        let tree = codec.deserialize(expr.to_string());
        let encoded = codec.serialize(tree);
        assert_eq!(expr.to_string(), encoded);
    }
}
