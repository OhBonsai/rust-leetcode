struct Solution ();
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut x = 0;
        let mut y = numbers.len() - 1;

        loop {
            let v = numbers[x] + numbers[y];
            if v > target {
                y -= 1
            } else if v < target {
               x += 1
            } else {
                return vec![x as i32 + 1, y as i32 + 1];
            }
        }


    }

    pub fn two_sum1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut temp= HashMap::new();

        for (i, x) in numbers.iter().enumerate() {
            if temp.contains_key(&(target - x)) {
                return vec![*temp.get(&(target - x)).unwrap() as i32 + 1 , i as i32 + 1]
            } else {
                temp.insert(x, i);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1, 2])
    }


}