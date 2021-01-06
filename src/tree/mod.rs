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

    pub fn str(&self) -> String {
        let (lines, _, _, _) = self.draw_aux();
        lines.join("\n")
    }


    fn draw_aux(&self) -> (Vec<String>, usize, usize, usize){
        return match (&self.left, &self.right) {
            (None, None) => {
                let line = format!("{}", self.val);
                let width = line.len();
                let height = 1;
                let mid = width / 2;
                (vec!(line), width, height, mid)
            },
            (Some(l), None) => {
                let (lines, n, p, x) = (**l).borrow().draw_aux();
                let s = format!("{}", self.val);
                let u = s.len();
                let first_line = " ".repeat(x + 1) + &"_".repeat(n - x - 1) + &s;
                let second_line = " ".repeat(x) + "/" + &" ".repeat(n - x - 1 + u);
                let mut shifted_lines = lines.iter().map(|x| x.to_owned() + &" ".repeat(u)).collect::<Vec<String>>();
                shifted_lines.insert(0, first_line);
                shifted_lines.insert(1, second_line);
                (shifted_lines, n + u, p + 2, n + u / 2)
            },

            (None, Some(r)) => {
                let (lines, n, p, x) = (**r).borrow().draw_aux();
                let s = format!("{}", self.val);
                let u = s.len();
                let first_line = s + &"_".repeat(x) + &" ".repeat(n - x);
                let second_line = " ".repeat(u + x) + "\\" + &" ".repeat(n - x - 1);
                let mut shifted_lines = lines.iter().map(|x| x.to_owned() + &" ".repeat(u)).collect::<Vec<String>>();
                shifted_lines.insert(0, first_line);
                shifted_lines.insert(1, second_line);
                (shifted_lines, n + u, p + 2, n + u / 2)
            },
            (Some(l), Some(r)) => {
                let (mut lefts, n, p, x) = (**l).borrow().draw_aux();
                let (mut rights, m, q, y) = (**r).borrow().draw_aux();
                let s = format!("{}", self.val);
                let u = s.len();
                let first_line = " ".repeat(x + 1) + &"_".repeat(n - x - 1) + &s + &"_".repeat(y) + &" ".repeat(m - y);
                let second_line = " ".repeat(x) + "/" + &" ".repeat(n - x - 1 + u + y) + "\\" + &" ".repeat(m - y - 1);
                if p < q {
                    for _ in 0..(q - p) {
                        lefts.push(" ".repeat(n))
                    }
                } else if q < p {
                    for _ in 0..(p - q) {
                        rights.push(" ".repeat(n))
                    }
                }

                let mut shifted_lines: Vec<String> = lefts.iter().zip(rights.iter()).map(|(x, y)| x.to_owned() + &" ".repeat(u) + y).collect();
                shifted_lines.insert(0, first_line);
                shifted_lines.insert(1, second_line);
                (shifted_lines, n + m + u, if p > q { p + 2 } else { q + 2 }, n + u / 2)
            }
        }

    }
}


impl Display for TreeNode {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.str())
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
mod q95;
mod q129;