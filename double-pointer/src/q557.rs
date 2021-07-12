struct Solution();

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut v = s.into_bytes();
        let (mut l, mut r) = (0, 0);


        while r < v.len() {
            while r < v.len() && v[r] != ' ' as u8   {
                r += 1;
            }

            let new_pos = r + 1;

            while l < r {
                unsafe {
                    std::ptr::swap(&mut v[l], &mut v[r-1]);
                    l += 1;
                    r -= 1;
                }
            }


            l = new_pos;
            r = new_pos;
        }
        String::from_utf8(v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_words(String::from("Let's take LeetCode contest")),
            String::from("s'teL ekat edoCteeL tsetnoc"))
    }
}