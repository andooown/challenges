use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut memo = HashMap::new();
        memo.entry(temperatures.last().unwrap())
            .or_insert(temperatures.len() - 1);

        let mut result = vec![0; temperatures.len()];
        for (i, temp) in temperatures.iter().enumerate().rev() {
            if let Some(j) = (*temp + 1..=100).filter_map(|t| memo.get(&t)).min() {
                result[i] = (j - i) as i32;
            }

            memo.insert(temp, i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );

        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        );
    }
}
