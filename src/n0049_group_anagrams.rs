struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            let mut bytes: [u8; 26] = [0; 26];
            for byte in s.bytes() {
                bytes[(byte - b'a') as usize] += 1u8;
            }
            map.entry(bytes).or_insert_with(Vec::new).push(s);
        }
        map.into_iter().map(|(_,v)| v ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO 每次测试vec列表的输出顺序不一致, 还不知道如何测试这种情况
    fn group_anagrams_test() {
        let mut result = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|s| s.to_string()).collect()
        );
        result.iter_mut().for_each(|v| v.sort_unstable());
        result.sort_unstable();

        let answer = vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]];
        assert_eq!(
            result,
            answer
        );
    }
}
