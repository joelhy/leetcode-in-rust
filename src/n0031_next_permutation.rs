struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
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
        }

        let slice = &mut nums[((i+1) as usize)..len];
        slice.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31_1() {
        let mut vec1 = vec![1, 2, 3];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 3, 2]);
    }
    
    #[test]
    fn test_31_2() {
        let mut vec2 = vec![3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3]);
    }

    #[test]
    fn test_31_3() {
        let mut vec2 = vec![1, 1, 5];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 5, 1]);
    }
}
