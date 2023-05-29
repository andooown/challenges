struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        for i in 0..len - 1 {
            for j in i + 1..len {
                if numbers[i] + numbers[j] == target {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum_ii_input_array_is_sorted::Solution;

    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
