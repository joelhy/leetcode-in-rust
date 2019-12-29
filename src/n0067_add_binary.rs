struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ret = Vec::with_capacity(usize::max(a.len(), b.len()) + 1);
        let mut a: Vec<_> = a.bytes().collect();
        let mut b: Vec<_> = b.bytes().collect();
        let mut carry = 0;
        while !(a.is_empty() && b.is_empty()) {
            let mut bit = a.pop().unwrap_or(b'0') + b.pop().unwrap_or(b'0') - b'0' * 2 + carry;
            carry = 0;
            if bit >= 2 {
                bit -= 2;
                carry = 1;
            }
            ret.push((bit + b'0') as char);
        }
        if carry == 1 {
            ret.push('1');
        }
        ret.reverse();
        ret.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary_test() {
        assert_eq!(Solution::add_binary("11".to_owned(), "1".to_owned()), "100".to_owned());
        assert_eq!(Solution::add_binary("1010".to_owned(), "1011".to_owned()), "10101".to_owned());
        assert_eq!(Solution::add_binary("0".to_owned(), "0".to_owned()), "0".to_owned());
        assert_eq!(Solution::add_binary("111".to_owned(), "111".to_owned()), "1110".to_owned());

    }
}