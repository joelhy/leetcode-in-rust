struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut min_distance = std::i32::MAX;
        let mut sum = 0;
        let len = nums.len();
        for i in 0..len-1 {
            let (mut left, mut right) = (i+1, len-1);
            while left < right {
                let curr_sum = nums[i] + nums[left] + nums[right];
                if curr_sum == target {
                    return curr_sum;
                }
                if curr_sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }

                let distance = (curr_sum - target).abs();
                if distance < min_distance {
                    min_distance = distance;
                    sum = curr_sum;
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}