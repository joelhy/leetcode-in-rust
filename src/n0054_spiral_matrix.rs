struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.len() < 1 {
            return result;
        }

        let (height, width) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut idx, mut i, mut j) = (0_usize, 0_i32, 0_i32);
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut matrix = matrix;
        for _ in 0..height*width {
            result.push(matrix[i as usize][j as usize]);
            matrix[i as usize][j as usize] = 0;
            let mut row = i + dirs[idx].0;
            let mut col = j + dirs[idx].1;
            if row < 0 || row >= height || col < 0 || col >= width || matrix[row as usize][col as usize] == 0 {
                idx = (idx + 1) % 4;
                row = i + dirs[idx].0;
                col = j + dirs[idx].1;
            }
            i = row;
            j = col;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54_1() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test_54_2() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
