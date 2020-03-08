struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len < 1 {
            return -1;
        }

        let mut left = 0_usize;
        let mut right = len - 1;
        while left <= right {
            let mid = (left + right)/2;
            if nums[mid] == target {
                return mid as i32;
            }
 
            if nums[mid] < nums[right] {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            } else {
                if nums[left] <= target && target < nums[mid] {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33_1() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn test_33_2() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn test_33_3() {
        assert_eq!(Solution::search(vec![1,3], 0), -1);
    }
}
