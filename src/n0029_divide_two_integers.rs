struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == dividend {
            return 1;
        }
        if divisor == std::i32::MIN {
            return 0;
        }

        let mut dvd = dividend;
        let mut adjust = 0;
        if dividend == std::i32::MIN {
            if divisor == 1 {
                return std::i32::MIN;
            }
            if divisor == -1 {
                return std::i32::MAX;
            }

            dvd += divisor.abs();
            adjust += 1;
        }

        dvd = dvd.abs();
        let dvs = divisor.abs();
        let mut result = 0;
        let limit = std::i32::MAX >> 1;
        while dvs <= dvd {
            let mut shift = 1;
            while (dvs << shift) <= dvd {
                if shift >= 31 || dvs << (shift-1) > limit {
                    break;
                }
                shift += 1;
            }
            result += 1 << (shift-1);
            dvd -= dvs << (shift-1);
        }

        let positive = (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0);
        if positive { result + adjust } else { -result - adjust }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_29_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_29_2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_29_3() {
        assert_eq!(Solution::divide(0, -3), 0);
    }

    #[test]
    fn test_29_4() {
        assert_eq!(Solution::divide(-7, -3), 2);
    }

    #[test]
    fn test_29_5() {
        assert_eq!(Solution::divide(-7, 3), -2);
    }

    #[test]
    fn test_29_6() {
        assert_eq!(Solution::divide(std::i32::MIN, -1), std::i32::MAX);
    }

    #[test]
    fn test_29_7() {
        assert_eq!(Solution::divide(std::i32::MIN, 1), std::i32::MIN);
    }

    #[test]
    fn test_29_8() {
        assert_eq!(Solution::divide(-587130364, std::i32::MIN), 0);
    }

    #[test]
    fn test_29_9() {
        assert_eq!(Solution::divide(std::i32::MAX, 1), std::i32::MAX);
    }
 
    #[test]
    fn test_29_10() {
        assert_eq!(Solution::divide(std::i32::MIN, 2), -1073741824);
    }

    #[test]
    fn test_29_11() {
        assert_eq!(Solution::divide(1100540749, 1090366779), 1);
    }
}
