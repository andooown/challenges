struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<_> = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        let a = chars.iter();
        let b = chars.iter().rev();
        for (a, b) in a.zip(b).take(chars.len() / 2 + 1) {
            if a.to_lowercase().to_string() != b.to_lowercase().to_string() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_palindrome() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);

        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
    }
}
