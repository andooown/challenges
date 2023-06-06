struct Solution;

use std::cmp;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut piles = piles;
        piles.sort_unstable();

        let len = piles.len();
        let (mut min, mut max) = (1, piles[len - 1]);
        while min <= max {
            let k = (min + max) / 2;
            match Self::calc_hour(&piles, k) {
                x if x == h as u64 => return Self::search_min_speed(&piles, min, k, h),
                x if x < h as u64 => max = k - 1,
                _ => min = k + 1,
            };
        }

        max = cmp::max(max, 1);
        if Self::calc_hour(&piles, max) == h as u64 {
            max
        } else {
            min
        }
    }

    fn search_min_speed(piles: &Vec<i32>, min: i32, max: i32, h: i32) -> i32 {
        let mut last_k = max;
        for k in (min..max).rev() {
            if Self::calc_hour(piles, k) != h as u64 {
                return last_k;
            }

            last_k = k
        }

        min
    }

    fn calc_hour(piles: &Vec<i32>, k: i32) -> u64 {
        piles
            .iter()
            .fold(0u64, |sum, p| sum + ((p + k - 1) / k) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_eating_speed() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);

        assert_eq!(Solution::min_eating_speed(vec![312884470], 312884469), 2);
        assert_eq!(Solution::min_eating_speed(vec![312884470], 968709470), 1);
        assert_eq!(
            Solution::min_eating_speed(vec![1, 1, 1, 999999999], 10),
            142857143
        );
        assert_eq!(
            Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }
}
