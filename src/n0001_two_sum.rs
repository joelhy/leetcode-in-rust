struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            let sub = target - nums[i];
            match map.get(&sub) {
                Some(sub_index) => return vec![*sub_index as i32, i as i32],
                None => {
                    map.insert(nums[i], i as i32);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 99), vec![]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::two_sum(vec![3, 6 ,18], 9), vec![0, 1]);
    }
}
