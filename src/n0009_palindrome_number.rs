struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }

        fn reverse(x: i32) -> i32 {
            let mut num = x as i64;
            let mut result: i64 = 0;

            while num != 0 {
                result = result * 10 + (num % 10);
                if result > i64::from(std::i32::MAX) || result < i64::from(std::i32::MIN) {
                    return 0;
                }

                num /= 10;
            }

            result as i32
        }

        x == reverse(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_given_121() {
        assert_eq!(true, Solution::is_palindrome(121));

    }

    #[test]
    fn should_return_false_given_negative_number() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }

    #[test]
    fn shold_return_false_given_10() {
        assert_eq!(false, Solution::is_palindrome(10));
    }

    #[test]
    fn should_return_true_given_0() {
        assert_eq!(true, Solution::is_palindrome(0));
    }
}
