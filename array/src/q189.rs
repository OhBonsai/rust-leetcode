struct Solution ();

impl Solution {

    pub fn rotate1(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let mut result = nums.iter().cloned().collect::<Vec<_>>();

        for (i, v) in result.iter().enumerate() {
            if i < n-k as usize {
                nums[k as usize + i] = *v
            } else {
                nums[i - (n -k as usize)] = *v
            }

        }

    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let x = k % nums.len();

        nums.reverse();
        nums[0..x as usize].reverse();
        nums[x as usize..].reverse();
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {
        let mut nums = vec![1,3,5,6, 7];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![6, 7, 1,3,5]);
    }
}