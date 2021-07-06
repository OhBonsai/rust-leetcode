struct Solution ();
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {

        let n = nums.len();

        if n == 1{
            return;
        }

        let mut l = 0;
        let mut r = 0;

        while  r <= n - 1{

            if nums[l] == 0  {
                while r < n-1 && nums[r] == 0 {
                    r += 1
                }
                nums[l] = nums[r];
                nums[r] = 0;

            }
            l += 1;
            r += 1;
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);


        let mut nums = vec![0, 2];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![2, 0]);


        let mut nums = vec![0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0]);
    }


}