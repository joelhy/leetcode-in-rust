struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' => {
                    match stack.pop() {
                        Some('(') => continue,
                        _ => return false,
                    }
                },
                '}' => {
                    match stack.pop() {
                        Some('{') => continue,
                        _ => return false,
                    }
                },
                ']' => {
                    match stack.pop() {
                        Some('[') => continue,
                        _ => return false,
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vaild_test1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn is_valid_test2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn is_valid_test3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn is_valid_test4() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
