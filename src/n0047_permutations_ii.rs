struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len <= 1 {
            return vec![nums];
        }
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();

        loop {
            let curr = nums.clone();
            results.push(curr);

            let mut i = (len - 2) as i32;

            while i >= 0 && nums[i as usize] >= nums[(i+1) as usize] {
                i -= 1;
            }
    
            if i >= 0 {
                let i = i as usize;
                let mut j = len - 1;
                while j > i && nums[i] >= nums[j] {
                    j -= 1;
                }
                nums.swap(i, j);
            } else {
                break;
            }
    
            let slice = &mut nums[((i+1) as usize)..len];
            slice.reverse();
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47_1() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![
                vec![1, 1, 2],
                vec![1, 2, 1],
                vec![2, 1, 1],
            ]
        )
    }

    #[test]
    fn test_47_2() {
        assert_eq!(Solution::permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1],]);
    }

    #[test]
    fn test_47_3() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1, 2]),
            vec![
                vec![1, 1, 1, 2], 
                vec![1, 1, 2, 1], 
                vec![1, 2, 1, 1], 
                vec![2, 1, 1, 1]
            ]
        );
    }
}
