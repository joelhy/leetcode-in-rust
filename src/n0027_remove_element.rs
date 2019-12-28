struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        for fast in 0..nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element_example1() {
        assert_eq!(Solution::remove_element(&mut vec![3,2,2,3], 3), 2);
    }

    #[test]
    fn remove_element_example2() {
        assert_eq!(Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    }
}
