struct Solution ();

impl Solution {


    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

        match nums.len() {
            v if v == 0 || nums.first().unwrap() >= &target => 0,
            v if nums.last().unwrap() == &target => (v - 1) as i32,
            v if nums.last().unwrap() < &target => v as i32,
            v => {
                let mut left = 0;
                let mut right = v - 1;

                while left <= right {
                    let mid = left + (right - left) / 2;
                    match nums[mid] {
                        m if m > target => right = mid - 1,
                        m if m < target => left = mid + 1,
                        _ => return mid as i32
                    }
                }

                (right+1) as i32
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
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1 );
        assert_eq!(Solution::search_insert(vec![1,3], 3), 1 );
        assert_eq!(Solution::search_insert(vec![1], 1), 0 );
    }
}