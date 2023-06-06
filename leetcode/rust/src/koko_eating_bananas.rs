struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut piles = piles;
        piles.sort_unstable();

        let len = piles.len();
        let (mut min, mut max) = (1, piles[len - 1]);
        while min < max {
            let k = (min + max) / 2;
            match Self::calc_hour(&piles, k).cmp(&h) {
                Ordering::Greater => min = k + 1,
                _ => max = k,
            };
        }

        min
    }

    #[inline(always)]
    fn calc_hour(piles: &Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for &p in piles {
            sum += (p + k - 1) / k;
        }

        sum
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
