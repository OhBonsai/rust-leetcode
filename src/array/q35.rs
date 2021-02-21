//! Given a sorted array of distinct integers and a target value, return the index if the target is found.
//! If not, return the index where it would be if it were inserted in order.
//!
//! Example 1:
//! ```
//! Input: nums = [1,3,5,6], target = 5
//! Output: 2
//! ```
//!
//! Example 2:
//! ```
//! Input: nums = [1,3,5,6], target = 2
//! Output: 1
//! ```
//!
//! Example 3:
//! ```
//! Input: nums = [1,3,5,6], target = 7
//! Output: 4
//! ```
//!
//! Example 4:
//! ```
//! Input: nums = [1,3,5,6], target = 0
//! Output: 0
//! ```
//!
//! Example 5:
//! ```
//! Input: nums = [1], target = 0
//! Output: 0
//! ```
//!
//! Constraints:
//! - 1 <= nums.length <= 104
//! - -104 <= nums[i] <= 104
//! - nums contains distinct values sorted in ascending order.
//! - -104 <= target <= 104


///  avg_time O(logn) space O(1)
struct Solution ();
impl Solution {

    pub fn search_insert3(nums: Vec<i32>, target: i32) -> i32 {
        match nums.len() {
            0 => 0,
            v if &target >= nums.last().unwrap() => v as i32,
            v if &target <= nums.first().unwrap() => 0,
            v => {
                let mut left = 0;
                let mut right = v - 1;

                while left <= right {
                    let middle = left + (right - left ) /2;
                    if nums[middle] > target {
                        right = middle - 1;
                    }  else if nums[middle] < target {
                        left = middle + 1
                    }  else {
                        return middle as i32;
                    }

                }
                (right + 1) as i32
            }

        }
    }


    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.len() {
            v if v == 0 || &target <= nums.first().unwrap() => 0,
            v if &target > nums.last().unwrap() =>  v as i32,
            v if &target == nums.last().unwrap() => (v - 1) as i32,
            v => {
                let mut left = 0;
                let mut right = v;

                while left < right {
                    let mid = left + (right - left) / 2;
                    match nums[mid] {
                        v if v > target => right = mid,
                        v if v < target => left = mid+1,
                        v => return mid as i32
                    }
                }

                right as i32
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2 );
        assert_eq!(Solution::search_insert(vec![1,3,5,6], -1), 0 );
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4 );
        assert_eq!(Solution::search_insert(vec![1], 1), 0 );
    }
}