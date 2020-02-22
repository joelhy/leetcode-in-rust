struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let answer: Vec<i32> = Vec::new();

        let mut seq = candidates;
        seq.sort_unstable();

        Self::solve(&seq, target, &mut result, answer, 0);
        result
    }

    fn solve(seq: &Vec<i32>, target: i32, result: &mut Vec<Vec<i32>>, answer: Vec<i32>, start: usize) {
        if target < 0 {
            return;
        }
        if target == 0 {
            result.push(answer);
        } else {
            for i in start..seq.len() {
                let mut curr = answer.clone();
                curr.push(seq[i]);
                Self::solve(&seq, target-seq[i], result, curr, i);
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
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn test_39_2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
