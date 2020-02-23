struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result = 1.0;
        let mut i = n;
        let mut x = x;
        while i != 0 {
            if i % 2 != 0 {
                result *= x;
            }
            x *= x;
            i /= 2;
        }

        if n < 0 { 1.0 / result } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50_1() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }

    #[test]
    fn test_50_2() {
        assert_eq!(Solution::my_pow(2.1, 3), 9.261000000000001);
    }

    #[test]
    fn test_50_3() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
