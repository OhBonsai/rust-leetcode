struct Solution ();
impl Solution {
    pub fn reverse_words(s: String) -> String {

        let mut v = s.into_bytes();
        let (mut left, mut right) = (0, 0);

        while right < v.len() {
            while right < v.len() && v[right] != ' ' as u8 {
                right += 1
            }

            let next = right + 1;

            while left < right {
                unsafe {
                    std::ptr::swap(&mut v[right -1], &mut v[left]);
                }
                left += 1;
                right -= 1;
            }

            left = next;
            right = next;
        }

        String::from_utf8(v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {



        assert_eq!(
            Solution::reverse_words(String::from("Let's take LeetCode contest")),
            String::from("s'teL ekat edoCteeL tsetnoc"))
    }


}