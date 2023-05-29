struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let (mut left, mut right) = (vec![0; len], vec![0; len]);
        let (mut left_acc, mut right_acc) = (1, 1);
        for i in 0..len {
            left_acc *= nums[i];
            right_acc *= nums[len - 1 - i];

            left[i] = left_acc;
            right[len - 1 - i] = right_acc;
        }

        nums.iter()
            .enumerate()
            .map(|(i, _)| {
                (if i > 0 { left[i - 1] } else { 1 }) * (if i < len - 1 { right[i + 1] } else { 1 })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );

        assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
        assert_eq!(Solution::product_except_self(vec![0, 4, 0]), vec![0, 0, 0]);
    }
}
