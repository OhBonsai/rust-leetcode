//! 数字 n代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
//!
//!
//! 示例 1：
//!
//! 输入：n = 3
//! 输出：["((()))","(()())","(())()","()(())","()()()"]
//! 示例 2：
//!
//! 输入：n = 1
//! 输出：["()"]
//!


struct Solution();
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {


        fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
            let mut res = vec![];

            if open == 0 && close == 0 {
                return vec![s.to_owned()]
            }

            if open > 0 {
                res.append(&mut back_track(s.clone()+"(", open-1, close+1));
            }

            if close > 0 {
                res.append(&mut back_track(s.clone()+")", open.clone(), close-1));
            }
            res

        }

        back_track("".to_owned(), n, 0)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::generate_parenthesis(2);
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"(())".to_owned()));
        assert!(result.contains(&"()()".to_owned()));

    }
}
