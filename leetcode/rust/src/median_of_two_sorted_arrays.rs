struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut v = [&nums1[..], &nums2[..]].concat();
        v.sort_unstable();

        let len = v.len();
        if len % 2 == 0 {
            (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
        } else {
            v[len / 2] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
