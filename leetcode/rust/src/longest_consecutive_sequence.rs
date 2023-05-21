use super::Solution;

use std::cmp;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut ns = nums.clone();
        ns.sort_unstable();
        ns.dedup();

        let mut start = 0;
        let mut max = 0;
        for i in 1..=ns.len() {
            match ns.get(i) {
                Some(v) => {
                    if ns[i - 1] + 1 != *v {
                        max = cmp::max(max, i - start);
                        start = i;
                    }
                }
                None => {
                    max = cmp::max(max, i - start);
                }
            };
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );

        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
