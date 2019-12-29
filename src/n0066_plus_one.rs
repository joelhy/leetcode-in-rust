struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                break;
            }
            digits[i] = 0;
        }
        if digits[0] == 0 {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_test() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![9,9,9]), vec![1,0,0,0]);
        assert_eq!(Solution::plus_one(vec![1,0,9]), vec![1,1,0]);
    }
}
