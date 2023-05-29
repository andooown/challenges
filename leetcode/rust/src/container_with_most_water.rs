struct Solution;

use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let (mut left, mut right) = (height[i], height[j]);

        let mut max = cmp::min(left, right) * (j - i) as i32;
        while i < j {
            if left < right {
                i += 1;
                left = height[i];
            } else {
                j -= 1;
                right = height[j];
            }

            let a = cmp::min(left, right) * (j - i) as i32;
            if max < a {
                max = a;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
