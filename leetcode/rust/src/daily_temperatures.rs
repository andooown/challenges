use super::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];

        let mut stack = vec![(temperatures.len() - 1, *temperatures.last().unwrap())];
        for (i, &temp) in temperatures.iter().enumerate().rev().skip(1) {
            while let Some(&(_, t)) = stack.last() {
                if t > temp {
                    break;
                }
                stack.pop();
            }

            if let Some(&(j, _)) = stack.last() {
                result[i] = (j - i) as i32;
            }
            stack.push((i, temp));
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
