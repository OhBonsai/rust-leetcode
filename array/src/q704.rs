
struct Solution ();
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        match nums.len() {
            0 => -1,
            1 => if nums[0] == target { 0 } else { -1 }
            _ => {
                let mut left = 0;
                let mut right = nums.len() - 1;
                let mut pos = nums.len();

                while left <= right && right != 0 {
                    let mid = (left + right) / 2;
                    if nums[mid] > target {
                        right = if mid > 0 {mid - 1 } else { 0 }
                    } else if nums[mid] < target {
                        left = mid + 1
                    } else {
                        pos = mid;
                        break
                    }
                }

                println!("{} {} {}", left, right, pos);
                if right == 0 && nums.last().unwrap() == &target {
                    (nums.len() - 1) as i32
                } else if right ==0 && nums.first().unwrap() == &target {
                    0
                } else {
                    if pos < nums.len() && nums[pos] == target {
                        return pos as i32
                    } else {
                        -1
                    }
                }
            }
        }





    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1 () {
        // assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
        // assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
        // assert_eq!(Solution::search(vec![2, 5], 0), -1);
        assert_eq!(Solution::search(vec![-1, 2, 5], -1), 0);
    }

}