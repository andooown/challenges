struct Solution;

use std::collections::HashMap;

#[derive(Debug)]
struct Count {
    value: i32,
    count: i32,
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = Solution::count(nums);
        counts.sort_unstable_by(|lhs, rhs| lhs.count.cmp(&rhs.count).reverse());

        counts.iter().take(k as usize).map(|c| c.value).collect()
    }

    fn count(nums: Vec<i32>) -> Vec<Count> {
        let mut counts = HashMap::new();
        for n in nums {
            counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
        }

        counts
            .iter()
            .map(|(k, v)| Count {
                value: *k,
                count: *v,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn top_k_frequent() {
        assert_eq!(
            HashSet::from_iter(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)),
            HashSet::from([1, 2])
        );
        assert_eq!(
            HashSet::from_iter(Solution::top_k_frequent(vec![1], 1)),
            HashSet::from([1])
        );

        assert_eq!(
            HashSet::from_iter(Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2)),
            HashSet::from([-1, 2])
        );
    }
}
