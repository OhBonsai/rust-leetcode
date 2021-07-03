use std::io::{self, Write};

struct Solution ();
impl Solution {

    pub fn helper(nums: &Vec<i32>, target: i32) -> usize {

        if nums[0] == target {
            return 0
        }

        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut a = nums.len();

        while l <= r && r != 0 {
            let m = (l + r) / 2;
            println!("{} {} {}", l, m ,r);
            if nums[m] > target {
                r = if m >= 1 { m - 1 } else { 0  };
            } else if nums[m] < target {
                l = m + 1;
            } else {
                a = m;
                break
            }
        }

        if r == 0 && nums.last().unwrap() == &target {
            nums.len() - 1
        } else {
            a
        }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.len() {
            0 => vec![-1, -1],
            1 if nums[0] == target => vec![0, 0],
            1 => vec![-1, -1],
            _ => {
                let hit = Solution::helper(&nums, target);
                println!("{}", hit);
                if  hit < nums.len() && nums[hit] == target {
                    let mut s = hit;
                    let mut e = hit;

                    while s >0 && nums[s] == target {
                        s = s - 1
                    }
                    if nums[0] == target {
                        s = 0
                    } else {
                        s = s + 1
                    }

                    while e < nums.len() && nums[e] == target {
                        e = e + 1
                    }
                    if nums.last().unwrap() == &target {
                        e = nums.len() - 1
                    } else {
                        e = e - 1
                    }

                    vec![s as i32, e as i32]
                } else {
                    vec![-1, -1]
                }
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {
        println!("hello");
        // assert_eq!(Solution::search_range(
        //     vec![5,7,7,8,8,10], 8
        // ), vec![3, 4]);
        // assert_eq!(Solution::search_range(
        //     vec![5,7,7,8,8,10], 6
        // ), vec![-1, -1]);
        // assert_eq!(Solution::search_range(
        //     vec![2, 2], 1
        // ), vec![-1, -1]);
        //
        // assert_eq!(Solution::search_range(
        //     vec![1,2,3], 1
        // ), vec![0, 0]);

        assert_eq!(Solution::search_range(
            vec![1,4], 4
        ), vec![1, 1])
    }
}