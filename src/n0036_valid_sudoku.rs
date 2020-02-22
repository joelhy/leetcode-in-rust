struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut row_set: HashSet<(usize, char)> = HashSet::new();
        let mut col_set: HashSet<(usize, char)> = HashSet::new();
        let mut box_set: HashSet<(usize, usize, char)> = HashSet::new();

        for (i, line) in board.iter().enumerate() {
            for (j, &number) in line.iter().enumerate() {
                if number == '.' {
                    continue;
                }
                if !row_set.insert((i, number)) {
                    return false;
                }
                if !col_set.insert((j, number)) {
                    return false;
                }
                if !box_set.insert((i/3, j/3, number)) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_36_1() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }

    #[test]
    fn test_36_2() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }

    #[test]
    fn test_36_3() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
                vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
                vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
                vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
                vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
                vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
                vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']
            ]),
            false
        );
    }
}
