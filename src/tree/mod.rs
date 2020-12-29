use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::{Display, Formatter, Result};
use std::borrow::Borrow;


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
            right: None
        }
    }

    fn str(&self, mut lvl: usize) -> String {
        let mut result = format!("|{}{}\x0a", "__".repeat(lvl), self.val);
        lvl+=1;
        match (&self.left, &self.right) {
            (Some(l), Some(r)) => {
                result.push_str(&(**l).borrow().str(lvl));
                result.push_str(&(**r).borrow().str(lvl));
            },
            (None, Some(r)) => {
                result.push_str(&format!("|{}null\x0a", "__".repeat(lvl)));
                result.push_str(&(**r).borrow().str(lvl));
            },
            (Some(l), None) => {
                result.push_str(&(**l).borrow().str(lvl));
                result.push_str(&format!("|{}null\x0a", "__".repeat(lvl)));
            },
            (None, None) => ()
        }
        result
    }
}


impl Display for TreeNode {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.str(0))
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back((*parent).borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back((*parent).borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}


mod q108;