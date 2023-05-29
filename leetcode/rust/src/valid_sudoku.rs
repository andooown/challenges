struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mapped: Vec<Vec<Option<u32>>> = board
            .iter()
            .map(|row| row.iter().map(|c| c.to_digit(10)).collect())
            .collect();

        for i in 0..9 {
            // Row
            if !Solution::has_no_duplicates((0..9).filter_map(|x| mapped[i][x].as_ref()).collect())
            {
                return false;
            }
            // Column
            if !Solution::has_no_duplicates((0..9).filter_map(|y| mapped[y][i].as_ref()).collect())
            {
                return false;
            }
            // Sub-box
            if !Solution::has_no_duplicates(
                ((i / 3) * 3..(i / 3) * 3 + 3)
                    .flat_map(|y| {
                        mapped[y][(i % 3) * 3..(i % 3) * 3 + 3]
                            .iter()
                            .filter_map(|v| v.as_ref())
                    })
                    .collect(),
            ) {
                return false;
            }
        }

        true
    }

    fn has_no_duplicates(values: Vec<&u32>) -> bool {
        let mut set = HashSet::with_capacity(values.len());
        for v in values {
            if set.contains(v) {
                return false;
            }
            set.insert(v);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_valid_sudoku() {
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
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            true
        );
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
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
    }
}
