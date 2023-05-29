struct Solution;

use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut sum = 0;
        let mut stack = vec![];
        for (i, h) in height.iter().enumerate() {
            let mut last_popped_h = None;
            while let Some(&(hh, ii)) = stack.last() {
                if let Some(l) = last_popped_h {
                    sum += (cmp::min(h, hh) - l) * (i - ii - 1) as i32;
                }

                if h < hh {
                    break;
                }

                stack.pop();
                last_popped_h = Some(hh);
            }

            stack.push((h, i));
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
