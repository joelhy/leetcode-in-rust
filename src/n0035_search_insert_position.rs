struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 || nums[0] > target {
            return 0;
        }

        let mut start = 0;
        let mut end = len - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }

        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_test() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 0), 0);
    }
}
