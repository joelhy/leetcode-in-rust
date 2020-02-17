struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return vec![];
        }

        let mut result = vec![];
        Self::backtrack(&mut result, String::new(), 0, 0, n);
        result
    }

    fn backtrack(result: &mut Vec<String>, curr: String, left: i32, right: i32, n: i32) {
        if curr.len() == 2 * n as usize {
            result.push(curr);
            return;
        }

        if left < n {
            Self::backtrack(result, curr.clone() + "(", left + 1, right, n);
        }
        if right < left {
            Self::backtrack(result, curr + ")", left, right + 1, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
