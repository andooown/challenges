struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut row = 0;
        for r in 0..m {
            if matrix[r][0] > target {
                break;
            }
            row = r
        }

        for c in 0..n {
            match matrix[row][c] {
                x if x == target => return true,
                x if x > target => return false,
                _ => (),
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_matrix() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );

        assert_eq!(Solution::search_matrix(vec![vec![1]], 2), false);
    }
}
