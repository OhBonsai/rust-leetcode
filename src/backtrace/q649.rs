struct Solution ();
impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {

        fn helpers(nums: Vec<f32>) -> bool {
            let len = nums.len();

            if len == 1 {
                return nums[0] == 24_f32
            }

            for i in 0..len {
                for j in 0..len {
                    if j != i {
                        let mut new_nums = vec![];
                        for k in 0..len {
                            if k != i && k != j {
                                new_nums.push(nums[k] as f32)
                            }
                        }


                        let mut new_nums_add = new_nums.clone();
                        new_nums_add.push(nums[i] + nums[j]);
                        if helpers(new_nums_add) {
                            return true
                        }


                        let mut new_nums_sub = new_nums.clone();
                        new_nums_sub.push(nums[i] - nums[j]);
                        if helpers(new_nums_sub) {
                            return true
                        }

                        let mut new_nums_mul = new_nums.clone();
                        new_nums_mul.push(nums[i] * nums[j]);
                        if helpers(new_nums_mul) {
                            return true
                        }

                        if nums[j] != 0_f32 {
                            let mut new_nums_div = new_nums.clone();
                            new_nums_div.push(nums[i] / nums[j]);
                            if helpers(new_nums_div) {
                                return true
                            }
                        }


                    }


                }
            }

            false
        }


        helpers(
            nums.into_iter()
                .map(|v| v as f32)
                .collect::<Vec<f32>>()
        )

    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]fn it_works() {

        assert!(Solution::judge_point24(vec![4, 1, 8, 7]));
        assert!(Solution::judge_point24(vec![1, 9, 1, 2]));
        assert_eq!(Solution::judge_point24(vec![1,2,1,2]), false);



    }
}