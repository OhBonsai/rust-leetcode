mod q121;


struct Solution ();
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut max = 0;
        let mut min_item  =  prices[0];


        for x in prices.into_iter().skip(1) {
            min_item = min_item.min(x);
            max = max.max(x - min_item);


        }


        max
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn is_ok() {

        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5)


    }
}