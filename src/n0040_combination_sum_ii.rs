struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let answer: Vec<i32> = Vec::new();

        let mut seq = candidates;
        seq.sort_unstable();

        Self::solve(&seq, target, &mut result, answer, 0);
        result
    }

    fn solve(seq: &[i32], target: i32, result: &mut Vec<Vec<i32>>, answer: Vec<i32>, start: usize) {
        if target < 0 {
            return;
        }
        if target == 0 {
            result.push(answer);
        } else {
            for i in start..seq.len() {
                if i > start && seq[i] == seq[i-1] {
                    continue;
                }

                let mut curr = answer.clone();
                curr.push(seq[i]);
                Self::solve(&seq, target-seq[i], result, curr, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39_1() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_39_2() {
        assert_eq!(
            Solution::combination_sum2(vec![2,5,2,1,2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
