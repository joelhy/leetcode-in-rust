struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{min, max};

        let (mut left, mut right) = (0_usize, height.len() - 1);
        let mut max_size = 0;
        while left < right {
            let width = (right - left) as i32;
            let curr_size = width * min(height[left], height[right]);
            max_size = max(max_size, curr_size);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
