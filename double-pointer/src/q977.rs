struct Solution ();
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        if n == 0 {
            return vec![]
        }

        if n == 1 {
            return vec![nums[0] * nums[0]]
        }

        if nums[0] >= 0 {
            return nums.iter().map(|x| x * x).collect()
        }

        if *nums.last().unwrap() <= 0 {
            return nums.iter().rev().map(|x| x * x).collect()
        }

        let mut result = vec![];
        let mut p1 = 0;
        let mut p2 = 1;
        let mut left_hit = false;
        let mut right_hit =  false;

        for _ in 0..n {
            if nums[p1] < 0 && nums[p2] >= 0 {
                break
            }
            p1 = p1 + 1;
            p2 = p2 + 1;
        }

        loop {
            let l = nums[p1] * nums[p1];
            let r = nums[p2] * nums[p2];

            if l >= r {
                result.push(r);
                if p2 == n-1 {
                    right_hit = true;
                    break
                }
                p2 = p2 + 1;
            }
            if l <= r {
                result.push(l);
                if p1 == 0 {
                    left_hit = true;
                    break
                }
                p1 = p1 - 1;
            }

        }

        if right_hit {
            for i in (0..p1+1).rev() {
                result.push(nums[i] * nums[i])
            }
        }



        if left_hit {
            for i in p2..n {
                result.push(nums[i] * nums[i])
            }
        }


        result
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {
        // assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-3,0,2]), vec![0,4, 9])

    }


}