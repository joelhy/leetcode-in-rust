struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().map_or(0, |x| x.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_last_word_test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
        assert_eq!(Solution::length_of_last_word("   ".to_string()), 0);
        assert_eq!(Solution::length_of_last_word("".to_string()), 0);
        assert_eq!(Solution::length_of_last_word(" a ".to_string()), 1);
    }
}
