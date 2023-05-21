use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.iter()
            .fold(
                HashMap::with_capacity(strs.len()),
                |mut acc: HashMap<[i32; 26], Vec<String>>, str| {
                    acc.entry(Solution::calc_histogram(str))
                        .or_insert(vec![])
                        .push(str.to_string());

                    acc
                },
            )
            .into_values()
            .collect()
    }

    fn calc_histogram(str: &str) -> [i32; 26] {
        let mut hist = [0; 26];
        for b in str.bytes() {
            hist[b as usize - 97] += 1;
        }

        hist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn group_anagrams() {
        assert_case(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"],
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
        );
        assert_case(vec![""], vec![vec![""]]);
        assert_case(vec!["a"], vec![vec!["a"]]);
    }

    fn assert_case(strs: Vec<&str>, expected: Vec<Vec<&str>>) {
        assert_eq!(
            {
                let mut result =
                    Solution::group_anagrams(strs.iter().map(|s| s.to_string()).collect());
                sort(&mut result);

                result
            },
            {
                let mut exp = expected
                    .into_iter()
                    .map(|v| v.iter().map(|s| s.to_string()).collect())
                    .collect();
                sort(&mut exp);

                exp
            }
        )
    }

    fn sort(values: &mut Vec<Vec<String>>) {
        values.iter_mut().for_each(|vs| vs.sort());
        values.sort_by(|lhs, rhs| match lhs.len().cmp(&rhs.len()) {
            Ordering::Equal => lhs.cmp(rhs),
            o => o,
        });
    }
}
