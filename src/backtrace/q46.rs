


struct Solution ();
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut arr: Vec<i32> = vec![];
        Solution::backtrace(&nums, &mut arr, &mut ans);


        let mut ans = Solution::backtrace2(&nums, vec![]);
        return ans;
    }


    fn backtrace(nums: &Vec<i32>, arr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if arr.len() == nums.len() {
            ans.push(arr.clone());
        } else {

            for i in nums {
                if !arr.contains(i) {
                    arr.push(*i);
                    Solution::backtrace(nums, arr, ans);
                    arr.pop();
                }
            }
        }
    }

    fn backtrace2(nums: &Vec<i32>, arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        if arr.len() == nums.len() {
            return vec![arr];
        }

        for x in nums {
            if !arr.contains(x) {

                let mut new_arr = arr.clone();
                new_arr.push(*x);
                result.append(&mut Solution::backtrace2(
                    nums,
                    new_arr
                ));
            }
        }

        result

    }


}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]fn test_ok() {

        println!("{:?}", Solution::permute(vec![1,2,3]))

    }
}