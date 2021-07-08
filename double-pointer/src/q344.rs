struct Solution ();
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {

        let mut l = 0;
        let mut r = s.len() - 1;
        let mut tmp = '0';

        while l < r {
            tmp = s[l];
            s[l] = s[r];
            s[r] = tmp;

            l += 1;
            r -= 1;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_1() {
        let mut v = vec!['h','e','l','l','o'];
        Solution::reverse_string(& mut v );
        assert_eq!(v, vec!['o','l','l','e','h'])
    }


}