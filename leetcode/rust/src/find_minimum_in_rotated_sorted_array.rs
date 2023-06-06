struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let idx = (left + right) / 2;
            if nums[idx] > nums[right] {
                left = idx + 1;
            } else {
                right = idx;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
