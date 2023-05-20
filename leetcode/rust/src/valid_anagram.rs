use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::with_capacity(s.len());
        for c in s.chars() {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for c in t.chars() {
            let v = match map.get(&c) {
                Some(v) => v,
                None => return false,
            };

            match v {
                1 => map.remove(&c),
                _ => map.insert(c, v - 1),
            };
        }

        map.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
