struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::recursive_search(&nums, target).map_or(-1, |i| i as i32)
    }

    fn recursive_search(nums: &[i32], target: i32) -> Option<usize> {
        println!("{nums:?}");
        match nums.len() {
            0 => None,
            1 => {
                if nums[0] == target {
                    Some(0)
                } else {
                    None
                }
            }
            len => match nums[len / 2] {
                v if v == target => Some(len / 2),
                v if v < target => Solution::recursive_search(&nums[len / 2 + 1..], target)
                    .map(|i| i + len / 2 + 1),
                _ => Solution::recursive_search(&nums[..len / 2], target),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);

        assert_eq!(Solution::search(vec![2, 5], 5), 1);
    }
}
