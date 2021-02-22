//! 给定一个含有n个正整数的数组和一个正整数 target 。
//!
//! 找出该数组中满足其和 ≥ target 的长度最小的 连续子数组[numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
//!
//!
//! 示例 1：
//! ```
//! 输入：target = 7, nums = [2,3,1,2,4,3]
//! 输出：2
//! 解释：子数组[4,3]是该条件下的长度最小的子数组。
//! ```
//! 示例 2：
//! ```
//! 输入：target = 4, nums = [1,4,4]
//! 输出：1
//! ```
//! 示例 3：
//! ```
//! 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
//! 输出：0
//! ```

struct Solution();
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = usize::MAX;
        let mut left_index = 0;
        let mut sum = 0;

        for right_index in 0..nums.len() {
            sum += nums[right_index];
            while sum >= target {
                result = result.min(right_index - left_index);
                sum -= nums[left_index];
                left_index+=1;
            }
        }

        if result == usize::MAX {
            0
        } else {
            result as i32 + 1
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2);



    }
}