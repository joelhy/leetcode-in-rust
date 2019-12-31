struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut x = (1, 1);
        for _ in 0..n {
            x = (x.1, x.0 + x.1)
        }
        x.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climb_stairs_test() {
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}