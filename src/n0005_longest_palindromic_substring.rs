struct Solution {}

impl Solution {
    fn find_palindrome(s: &str, mut left: usize, mut right: usize) -> &str {
        let s_bytes = s.as_bytes();

        if s_bytes[left] != s_bytes[right] {
            return "";
        }
        while left > 0 && right < s.len() - 1 && s_bytes[left - 1] == s_bytes[right + 1] {
            left -= 1;
            right += 1;
        }

        &s[left..=right]
    }

    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n <= 1 {
            return s;
        }

        let mut longest = "";
        for i in 0..n - 1 {
            let ans = Self::find_palindrome(&s, i, i);
            if ans.len() > longest.len() {
                longest = ans;
            }
            let ans = Self::find_palindrome(&s, i, i + 1);
            if ans.len() > longest.len() {
                longest = ans;
            }
        }

        longest.to_string()
    }
}

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_5() {
    assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), "aaaaa");
    assert_eq!(Solution::longest_palindrome("babab".to_string()), "babab");
    assert_eq!(Solution::longest_palindrome("babcd".to_string()), "bab");
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb");
    assert_eq!(Solution::longest_palindrome("".to_string()), "");
   }
}