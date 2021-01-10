//! Link：https://leetcode-cn.com/problems/balance-a-binary-search-tree
use super::*;
struct Solution{}



impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums: Vec<i32> = Vec::new();

        // 中序深度遍历拿到有序数组
        fn build_sort_vector(r: Option<Rc<RefCell<TreeNode>>>, n: &mut Vec<i32>) {
            if let Some(v) = r {
                build_sort_vector(RefCell::borrow(&v).left.clone(), n);
                n.push(RefCell::borrow(&v).val);
                build_sort_vector(RefCell::borrow(&v).right.clone(), n);
            }
        }
        ;

        // 递归构建二叉平衡术
        fn build_bbst(n: &Vec<i32>, s: usize, e: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if s > e {
                return None
            }

            let m = (s + e) / 2;
            let mut root = TreeNode::new(n[m]);
            if s == e {
                return Some(Rc::new(RefCell::new(root)))
            }

            if m >= 1 {
                let left_node = build_bbst(n, s, m - 1);
                root.left = left_node.clone();
            }

            let right_node = build_bbst(n, m + 1, e);
            root.right = right_node.clone();
            return Some(Rc::new(RefCell::new(root)))
        }

        build_sort_vector(root, &mut nums);
        build_bbst(&nums, 0, nums.len()-1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let trees = Solution::balance_bst(tree!(1, null, 2, null, 3, null, 4, null));
        println!("{:?}", trees.unwrap());

    }
}