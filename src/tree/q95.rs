//! Given an integer n, generate all structurally unique BST's (binary search trees) that store values 1 ... n.
//!
//! Example:
//! ```shell
//! Input: 3
//! Output:
//! [
//!   [1,null,3,2],
//!   [3,2,null,1],
//!   [3,1,null,null,2],
//!   [2,1,3],
//!   [1,null,2,null,3]
//! ]
//! Explanation:
//! The above output corresponds to the 5 unique BST's shown below:
//!
//! 1         3     3      2      1
//! \       /     /      / \      \
//! 3     2     1      1   3      2
//! /     /       \                 \
//! 2     1         2                 3
//!
//! ```
use super::*;



struct Solution{}
impl Solution {
    pub fn helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        if start > end {
            result.push(None);
            return result
        }

        for i in start..=end {
            let left_result = Solution::helper(start, i-1);
            let right_result = Solution::helper(i+1, end);

            for j in 0..left_result.len() {
                for k in 0..right_result.len() {
                    let mut node = TreeNode::new(i);
                    node.left = left_result[j].clone();
                    node.right = right_result[k].clone();
                    result.push(Some(Rc::new(RefCell::new(node))));
                }
            }

        }

        result
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec!()
        } else if n == 1 {
            vec!(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
        } else {
            Solution::helper(1, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        Solution::generate_trees(3).iter().map(|x| {
            if let Some(v) = x {
                println!("{}", (**v).borrow());
                println!("-----------------");
            }
        }).collect::<()>();
        assert_eq!(1, 1)
    }
}

