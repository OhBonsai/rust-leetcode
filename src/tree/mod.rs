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



mod q108;