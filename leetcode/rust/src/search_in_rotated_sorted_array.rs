struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let offset = Self::detect_rotation(&nums);

        let ns = [&nums[offset..], &nums[..offset]].concat();
        ns.binary_search(&target)
            .map_or(-1, |i| ((i + offset) % nums.len()) as i32)
    }

    #[inline(always)]
    fn detect_rotation(nums: &Vec<i32>) -> usize {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let idx = (left + right) / 2;
            if nums[idx] > nums[right] {
                left = idx + 1;
            } else {
                right = idx;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
