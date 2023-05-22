use super::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Solution::generate(&mut result, n, &mut vec![], 0, 0);

        result
    }

    fn generate(v: &mut Vec<String>, n: i32, current: &mut Vec<String>, opened: i32, closed: i32) {
        if opened + closed == 2 * n {
            v.push(current.concat());
            return;
        }

        if opened < n {
            current.push("(".to_string());
            Solution::generate(v, n, current, opened + 1, closed);
            current.pop();
        }
        if closed < n && closed < opened {
            current.push(")".to_string());
            Solution::generate(v, n, current, opened, closed + 1);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
