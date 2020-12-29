//! Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
//! For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
//! 
//! ### Example:
//! Given the sorted array: `[-10,-3,0,5,9]`,
//! One possible answer is: `[0,-3,9,-10,null,5]`, which represents the following height balanced BST:
//! ```shell
//!       0
//!      / \
//!    -3   9
//!    /   /
//!  -10  5
//! ```
//! 
use super::*;

struct Solution ();
impl Solution {
    /// 递归方法
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            None
        } else if nums.len() == 1 {
            Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))))
        } else {
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode{
                val: nums[mid],
                left: Solution::sorted_array_to_bst(nums[..mid].to_owned()),
                right: Solution::sorted_array_to_bst(nums[mid + 1..].to_owned())
            })))
        }
    }

    /// 迭代方法
    pub fn sorted_array_to_bst2(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        let mut stack: Vec<(u32, u32, Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> = vec!((0, (nums.len() - 1) as u32, None, None));

        while !stack.is_empty() {
            let (start, end, lp, rp) = stack.pop().unwrap();
            if start > end {
                continue;
            }

            let mid = (end + start) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
            if root.is_none() {
                root = Some(Rc::clone(&node));
            }

            if !lp.is_none() {
                (*lp.unwrap()).borrow_mut().right = Some(Rc::clone(&node))
            }

            if !rp.is_none() {
                (*rp.unwrap()).borrow_mut().left = Some(Rc::clone(&node))
            }

            if mid > 1 {
                stack.push((start, mid - 1, None, Some(Rc::clone(&node))));
            }
            stack.push((mid+1, end, Some(Rc::clone(&node)), None));
        }

        return root
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec!(-10,-3,0,5,9);
        let tree = Solution::sorted_array_to_bst(nums).unwrap();
        println!("{}", (*tree).borrow());
        assert_eq!((*tree).borrow().val, 0);
    }

    #[test]
    fn test_2() {
        let nums = vec!(-10,-3,0,5,9);
        let tree = Solution::sorted_array_to_bst2(nums).unwrap();
        println!("{}", (*tree).borrow());
        assert_eq!((*tree).borrow().val, 0);
    }
}
