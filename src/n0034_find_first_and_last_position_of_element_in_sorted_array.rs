struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        let len = nums.len();
        if len == 0 {
            return result;
        }
        let (mut start, mut end) = (0usize, len-1);

        while start <= end {
            let mid = (start + end) / 2;
            if target <= nums[mid] {
                if mid == 0 {
                    break;
                }
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        if start == len || nums[start] != target {
            return result;
        } else {
            result[0] = start as i32;
        }

        // start do not need to be reset
        end = len - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if target < nums[mid] {
                if mid == 0 {
                    break;
                }
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        result[1] = end as i32;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34_1() {
        assert_eq!(Solution::search_range(vec![5, 7, 7 ,8, 8, 10], 8), vec![3, 4]);
    }

    #[test]
    fn test_34_2() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}
