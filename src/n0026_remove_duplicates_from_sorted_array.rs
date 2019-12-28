struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return len as i32;
        }
        let mut i = 0;
        for j in 1..len {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_0_given_empty_array() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
    }

    #[test]
    fn should_return_2_given_1_1_2() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
    }

    #[test]
    fn should_return_5_given_0_0_1_1_1_2_2_3_3_4() {
        assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}
