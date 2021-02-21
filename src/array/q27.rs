//! 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
//!
//! 不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并「原地」修改输入数组。
//!
//! 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
//!
//! 示例 1:
//! 给定 nums = [3,2,2,3], val = 3,
//! 函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。
//! 你不需要考虑数组中超出新长度后面的元素。
//!
//! 示例 2:
//! 给定 nums = [0,1,2,2,3,0,4,2], val = 2,
//! 函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。



/// 时间复杂度
/// 空间复杂度
struct Solution ();
impl Solution {

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow_index = 0;
        for fast_index in 0..nums.len()  {
            if nums[fast_index] != val {
                nums[slow_index] = nums[fast_index];
                slow_index += 1;
            }
        }

        slow_index as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {

        let mut v = vec![3,2,2,3];
        assert_eq!(Solution::remove_element(&mut v, 3), 2);

        assert_eq!(v[0], 2);
        assert_eq!(v[1], 2);



        let mut v = vec![0, 1, 2,2 ,3, 0, 4,  2];
        assert_eq!(Solution::remove_element(&mut v, 2), 5);

        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 3);
        assert_eq!(v[3], 0);
        assert_eq!(v[4], 4);
    }


}