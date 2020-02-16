struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        
        let mut nums = nums;
        nums.sort();
        let mut results: Vec<Vec<i32>> = Vec::new();
        for i in 0..len-2 {
            if i >= 1 && nums[i-1] == nums[i] {
                continue;
            }
            let (mut left, mut right, sum) = (i + 1, len - 1, 0 - nums[i]);
            while left < right {
                if nums[left] + nums[right] == sum {
                    results.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if nums[left] + nums[right] < sum {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
