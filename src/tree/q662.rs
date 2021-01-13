/// Given a binary tree, write a function to get the maximum width of the given tree. The maximum width of a tree is the maximum width among all levels.
/// The width of one level is defined as the length between the end-nodes (the leftmost and right most non-null nodes in the level, where the null nodes between the end-nodes are also counted into the length calculation.
///
/// It is guaranteed that the answer will in the range of 32-bit signed integer.
///
/// Example 1:
///
/// Input:
///
/// 1
/// /   \
/// 3     2
/// / \     \
/// 5   3     9
///
/// Output: 4
/// Explanation: The maximum width existing in the third level with the length 4 (5,3,null,9).
/// Example 2:
///
/// Input:
///
/// 1
/// /
/// 3
/// / \
/// 5   3
///
/// Output: 2
/// Explanation: The maximum width existing in the third level with the length 2 (5,3).
/// Example 3:
///
/// Input:
///
/// 1
/// / \
/// 3   2
/// /
/// 5
///
/// Output: 2
/// Explanation: The maximum width existing in the second level with the length 2 (3,2).
/// Example 4:
///
/// Input:
///
/// 1
/// / \
/// 3   2
/// /     \
/// 5       9
/// /         \
/// 6           7
/// Output: 8
/// Explanation:The maximum width existing in the fourth level with the length 8 (6,null,null,null,null,null,null,7).
///  
///
/// Constraints:
///
/// The given binary tree will have between 1 and 3000 nodes.
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/maximum-width-of-binary-tree
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


use super::*;

use std::collections::VecDeque;
use std::cmp::max;

struct Solution{}
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        let mut max_width = 0;
        let mut current_height = 0;
        let mut left = 0;
        let mut queue = VecDeque::new();
        queue.push_front((root, 0, 0));

        while !queue.is_empty() {
            match queue.pop_back() {
                Some((Some(n), pos, height) ) => {
                    queue.push_front((RefCell::borrow(&n).left.clone(), pos * 2, height + 1));
                    queue.push_front((RefCell::borrow(&n).right.clone(), pos * 2 + 1, height + 1));

                    if current_height != height {
                        current_height = height;
                        left = pos
                    }

                    max_width = max(max_width, pos-left+1);
                },
                _ => ()
            }

        }

        max_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::width_of_binary_tree(tree!(1,2,3)), 2)
   }
}

