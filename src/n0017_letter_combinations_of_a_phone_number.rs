struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let letter_chars: Vec<(char, Vec<char>)> = vec![
            ('0', vec![]),
            ('1', vec![]),
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ];

        let mut results: Vec<String> = vec![String::with_capacity(digits.len())];
        for d_ch in digits.chars() {
            let d = d_ch as i32 - '0' as i32;
            let curr_letters = &letter_chars[d as usize].1;
            let mut queue: Vec<String> = vec![];
            for result in results.iter_mut() {
                for curr_letter in curr_letters {
                    let mut new_result = (*result).clone();
                    new_result.push(*curr_letter);
                    queue.push(new_result);
                }
            }
            results = vec![];
            results.append(&mut queue);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(Solution::letter_combinations("23".to_string()), 
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }
}
