struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_321_given_123() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn should_return_negative_321_given_negative_123() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn should_return_0_given_0() {
        assert_eq!(0, Solution::reverse(0));
    }

    #[test]
    fn should_return_21_given_120() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn should_return_0_given_int_max() {
        assert_eq!(0, Solution::reverse(std::i32::MAX));
    }
}
