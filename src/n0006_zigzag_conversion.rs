struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows: Vec<String> = (0..num_rows).map(|_| String::new()).collect();
        let mut curr_row = 0;
        let mut going_down = false;

        for ch in s.chars() {
            rows[curr_row as usize].push(ch);
            if curr_row == 0 || curr_row == num_rows - 1 {
                going_down = !going_down;
            }
            curr_row += if going_down { 1 } else { -1 };
        }

        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR".to_owned());
        assert_eq!(Solution::convert("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI".to_owned());
    }
}