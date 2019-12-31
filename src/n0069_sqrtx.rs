struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return x;
        }

        let x = x as usize;
        let mut r = x;
        while r > x / r {
            r = (r + x / r) / 2;
        }
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_sqrt_test() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(10), 3);
    }
}
