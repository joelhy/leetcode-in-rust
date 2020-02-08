struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let (mut i, mut res) = (0, 0);
        for (j, ch) in s.chars().enumerate() {
            if let Some(pos) = map.get(&ch) {
                i = i.max(*pos);
            }
            res = res.max(j + 1 - i);
            map.insert(ch, j + 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
