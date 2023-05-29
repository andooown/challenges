struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for t in tokens {
            match &*t {
                "+" => Solution::operation(&mut stack, |a, b| a + b),
                "-" => Solution::operation(&mut stack, |a, b| a - b),
                "*" => Solution::operation(&mut stack, |a, b| a * b),
                "/" => Solution::operation(&mut stack, |a, b| a / b),
                t => stack.push(t.parse().unwrap()),
            };
        }

        *stack.last().unwrap()
    }

    #[inline]
    fn operation<F>(stack: &mut Vec<i32>, op: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(op(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn eval_rpn() {
        assert_eq!(
            Solution::eval_rpn(["2", "1", "+", "3", "*"].map(|s| s.to_string()).to_vec()),
            9
        );
        assert_eq!(
            Solution::eval_rpn(["4", "13", "5", "/", "+"].map(|s| s.to_string()).to_vec()),
            6
        );
        assert_eq!(
            Solution::eval_rpn(
                ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .map(|s| s.to_string())
                    .to_vec()
            ),
            22
        );
    }
}
