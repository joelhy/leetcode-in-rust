struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        use std::char::from_digit;

        let mut result = String::new();
        let ch_0 = '0' as u32;
        let (m, n) = (num1.len(), num2.len());
        let mut vals: Vec<u32> = vec![0; m + n];

        let mut i = m;
        for multiplier in num1.chars().rev() {
            i -= 1;
            let mut j = n;
            for multiplicand in num2.chars().rev() {
                j -= 1;
                let product = (multiplier as u32 - ch_0) * (multiplicand as u32 - ch_0);
                let (p1, p2) = (i+j, i+j+1);
                let sum = product + vals[p2];
                vals[p1] += sum / 10;
                vals[p2] = sum % 10;

            }
        }
        for val in vals {
            if result.is_empty() && val == 0 {
                continue;
            }
            result.push(from_digit(val, 10).unwrap_or('0'));
        }

        if result.is_empty() { String::from("0") } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43_1() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6".to_string());
    }

    #[test]
    fn test_43_2() {
        assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088".to_string());
    }
}
