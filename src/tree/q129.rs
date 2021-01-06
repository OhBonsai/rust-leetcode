//! Given a binary tree containing digits from 0-9 only, each root-to-leaf path could represent a number.
//!
//! An example is the root-to-leaf path 1->2->3 which represents the number 123.
//!
//! Find the total sum of all root-to-leaf numbers.
//!
//! Note: A leaf is a node with no children.
//!
//! Example:
//!
//! ```shell
//! Input: [1,2,3]
//! 1
//! / \
//! 2   3
//! Output: 25
//! Explanation:
//! The root-to-leaf path 1->2 represents the number 12.
//! The root-to-leaf path 1->3 represents the number 13.
//! Therefore, sum = 12 + 13 = 25.
//! Example 2:
//! ```
//!
//! ```shell
//! Input: [4,9,0,5,1]
//! 4
//! / \
//! 9   0
//!  / \
//! 5   1
//! Output: 1026
//! Explanation:
//! The root-to-leaf path 4->9->5 represents the number 495.
//! The root-to-leaf path 4->9->1 represents the number 491.
//! The root-to-leaf path 4->0 represents the number 40.
//! Therefore, sum = 495 + 491 + 40 = 1026.
//! ```
//!
//! Link：https://leetcode-cn.com/problems/sum-root-to-leaf-numbers

use super::*;
struct Solution{}



impl Solution {
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(v) = root {
            let val = RefCell::borrow(&v).val;
            let mut left = Solution::helper(RefCell::borrow(&v).left.clone());
            let mut right = Solution::helper(RefCell::borrow(&v).right.clone());
            if left.len() == 0 && right.len() == 0 {
                left.push(vec!(val))
            } else {
                left.iter_mut().for_each(|x| x.push(val));
                right.iter_mut().for_each(|x| x.push(val));
                left.extend(right);
            }
            left
        } else {
            vec!()
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper2(r: Option<Rc<RefCell<TreeNode>>>, p: i32) -> Vec<i32> {
            let mut nums = vec!();
            if let Some(v) = r {
                let val = RefCell::borrow(&v).val + 10 * p ;
                let left_child = RefCell::borrow(&v).left.clone();
                let right_child = RefCell::borrow(&v).right.clone();
                if left_child.is_none() && right_child.is_none() {
                    nums.push(val);
                } else {
                    nums.extend(Solution::helper2(left_child, val));
                    nums.extend(Solution::helper2(right_child, val));
                }
            }
            nums
        }
        h(root, 0).iter().sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::sum_numbers(tree!(4,9,0,5,1)), 1026);
        assert_eq!(Solution::sum_numbers(tree!(1,2, 3)), 25)
    }
}