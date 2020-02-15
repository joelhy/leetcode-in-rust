struct Solution {}

impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let mut sign = 1;
        let mut result = 0;
        let mut curr: i32;
        let mut num_matched = false;
        let (i32_max, i32_min) = (std::i32::MAX, std::i32::MIN);

        for ch in input.chars() {
            match ch {
                ' ' | '-' | '+' => {
                    if num_matched {
                        return result * sign;
                    }

                    match ch {
                        '-' | '+' => {
                            num_matched = true;
                            sign = if ch == '-' { -1 } else { 1 };
                        },
                        _ => {}
                    }
                }
                '0'..='9' => {
                    num_matched = true;
                    curr = ch as i32 - '0' as i32;

                    // 是否会溢出判断
                    match sign {
                        1 => {
                            if result * sign > i32_max / 10 || (result * sign == i32_max / 10 && curr * sign > 7) {
                                return i32_max;
                            }
                        }
                        -1 => {
                            if result * sign < i32_min / 10 || (result * sign == i32_min / 10 && curr * sign < -8) {
                                return i32_min;
                            }
                        }
                        _ => {}
                    }

                    result = result * 10 + curr;
                }
                _ => return result * sign,
            }
        }

        result * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
