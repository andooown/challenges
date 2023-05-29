struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let set: HashSet<_> = nums.iter().collect();

        let mut ns: Vec<_> = nums.iter().collect();
        ns.sort_unstable();
        let ns = ns;

        let len = ns.len();
        let mut result = vec![];
        for i in 0..len - 2 {
            if i > 0 && ns[i - 1] == ns[i] {
                continue;
            }
            if *ns[i] > 0 {
                break;
            }

            for j in i + 1..len - 1 {
                if j > i + 1 && ns[j - 1] == ns[j] {
                    continue;
                }

                match -(ns[i] + ns[j]) {
                    x if x == *ns[j] && x == *ns[j + 1] => result.push(vec![*ns[i], *ns[j], x]),
                    x if x > *ns[j] && set.contains(&x) => result.push(vec![*ns[i], *ns[j], x]),
                    x if x < *ns[j] => break,
                    _ => (),
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
    }
}
