struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![0; n];
        let mut max = nums[0];
        dp[0] = nums[0];
        for i in 1..n {
            if dp[i - 1] < 0 {
                dp[i] = nums[i]
            } else {
                dp[i] = dp[i - 1] + nums[i]
            }

            max = i32::max(dp[i], max)
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_subarray_test() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2, 0]), 0);
    }
}
