use super::Solution;

#[derive(Clone)]
enum Parentheses {
    Open,
    Close,
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Solution::generate(&mut result, n, vec![], 0, 0);

        result
    }

    fn generate(v: &mut Vec<String>, n: i32, current: Vec<Parentheses>, opened: i32, closed: i32) {
        if opened + closed == 2 * n {
            v.push(Solution::build_string(current));
            return;
        }

        if opened < n {
            Solution::generate(
                v,
                n,
                [&current[..], &vec![Parentheses::Open]].concat(),
                opened + 1,
                closed,
            );
        }
        if closed < n && closed < opened {
            Solution::generate(
                v,
                n,
                [&current[..], &vec![Parentheses::Close]].concat(),
                opened,
                closed + 1,
            );
        }
    }

    fn build_string(v: Vec<Parentheses>) -> String {
        v.iter()
            .map(|p| match p {
                Parentheses::Open => "(".to_string(),
                Parentheses::Close => ")".to_string(),
            })
            .collect::<Vec<String>>()
            .concat()
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
