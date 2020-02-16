struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 4 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        let mut results: Vec<Vec<i32>> = Vec::new();
        for i in 0..len-3 {
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }
            for j in i+1..len-2 {
                if j > i+1 && nums[j-1] == nums[j] {
                    continue;
                }
                let (mut left, mut right, sum) = (j + 1, len - 1, target - nums[i] - nums[j]);
                while left < right {
                    if nums[left] + nums[right] == sum {
                        results.push(vec![nums[i], nums[j], nums[left], nums[right]]);
    
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
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );

        assert_eq!(Solution::four_sum(vec![0, 0, 0, 0], 0), vec![vec![0,0,0,0]]);
        assert_eq!(Solution::four_sum(vec![0, 0, 0, 0], 1), Vec::<Vec::<i32>>::new());
        assert_eq!(Solution::four_sum(vec![-3,-1,0,2,4,5], 0), vec![vec![-3,-1,0,4]]);
    }
}
