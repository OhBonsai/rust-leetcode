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

        fn nest_function(s: String, l:i32 , r: i32, n: i32)-> Vec<String>{
            let mut ret = vec![];

            if l == n && r == n {
                return vec![s]
            }


            if l < n {
                ret.append(&mut nest_function(s.clone() + "(", l + 1, r, n));
            }

            if r < n && l > r{
                ret.append(&mut nest_function(s.clone() + ")", l, r+1, n));
            }

            ret
        }

        nest_function("(".to_owned(), 1, 0 ,n)
    }



}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::generate_parenthesis(3);
        println!("{:?}", result);
        // assert_eq!(result.len(), 2);
        // assert!(result.contains(&"(())".to_owned()));
        // assert!(result.contains(&"()()".to_owned()));

    }
}
