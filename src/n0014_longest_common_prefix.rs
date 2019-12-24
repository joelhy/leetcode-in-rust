struct Solution {}

use std::str::Chars;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let (first_str, rest_strs) = match strs.split_first() {
            Some(x) => x,
            None => return "".to_string(),
        };

        let mut iters: Vec<Chars> = rest_strs.iter().map(|s| s.chars()).collect();
        first_str
            .chars()
            .take_while(|&c| iters.iter_mut().all(|i| i.next() == Some(c)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]), 
            "fl".to_string()
        );
    }

    #[test]
    fn test_match_no_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}