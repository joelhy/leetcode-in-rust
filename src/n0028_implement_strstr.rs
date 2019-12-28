struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if haystack_len < needle_len {
            return -1;
        }

        let mut i = 0;
        while i <= haystack_len - needle_len {
            if haystack[i..i+needle_len] == needle {
                return i as i32
            }
            i += 1;
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_str_test_example1() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn str_str_test_example2() {
        assert_eq!(Solution::str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }

    #[test]
    fn str_str_test_empty_needle() {
        assert_eq!(Solution::str_str("aaaaa".to_string(), "".to_string()), 0);
    }

    #[test]
    fn str_str_test_empty_haystack() {
        assert_eq!(Solution::str_str("".to_string(), "a".to_string()), -1);
    }
}
