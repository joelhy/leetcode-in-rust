struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn roman_char_to_int(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let mut last = 0;
        let mut num = 0;
        for ch in s.chars().rev() {
            let curr = roman_char_to_int(ch);
            // if curr<last, such as IV,XL,CD,IV..
            if curr < last {
                num -= curr;
            } else {
                num += curr;
            }
            last = curr;
        }

        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_3() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn test_roman_4() {
        assert_eq!(4, Solution::roman_to_int("IV".to_string()));
    }

    #[test]
    fn test_roman_9() {
        assert_eq!(9, Solution::roman_to_int("IX".to_string()));
    }

    #[test]
    fn test_roman_58() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn test_roman_1994() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
