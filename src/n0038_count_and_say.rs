struct Solution {}

use std::char::from_digit;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec!['1'];
        for _ in 0..n-1 {
            let mut curr = Vec::new();
            let mut slow = 0_usize;
            let result_len = result.len();
            while slow < result_len {
                let mut fast = slow + 1;
                while fast < result_len && result[fast] == result[slow] {
                    fast += 1;
                }
                curr.push(from_digit((fast - slow) as u32, 10).unwrap());
                curr.push(result[slow]);
                slow = fast;
            }
            result = curr;
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_say_test() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(2), "11".to_string());
        assert_eq!(Solution::count_and_say(3), "21".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
        assert_eq!(Solution::count_and_say(5), "111221".to_string());
        assert_eq!(Solution::count_and_say(6), "312211".to_string());
    }
}
