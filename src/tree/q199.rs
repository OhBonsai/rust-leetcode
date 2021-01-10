/// Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
///
/// Example:
///
/// Input: [1,2,3,null,5,null,4]
/// Output: [1, 3, 4]
/// Explanation:
///
/// 1            <---
/// /   \
/// 2     3         <---
/// \     \
/// 5     4       <---
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/binary-tree-right-side-view
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
use super::*;
use std::collections::VecDeque;

struct Solution ();
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        let mut result  = vec![];
        let mut queue = VecDeque::new();

        if root.is_some() {
            queue.push_front((root.unwrap(), 1))
        }

        let mut last_lvl = 0;

        while !queue.is_empty() {
            let (r, lvl) = queue.pop_back().unwrap();


            let rr = RefCell::borrow(&r);

            if lvl != last_lvl {
                last_lvl = lvl;
                result.push(rr.val);
            }
            if let Some(r) = &rr.right {
                queue.push_front((Rc::clone(r), lvl + 1))
            }
            if let Some(l) = &rr.left {
                queue.push_front((Rc::clone(l), lvl + 1))
            }
        }
        result

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let t =  tree!(1,2,3,null,5,null,4);
        println!("{:?}", Solution::right_side_view(t));
    }

}
