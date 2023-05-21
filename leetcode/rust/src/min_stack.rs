use std::cmp;

struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            mins: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.mins
            .push(self.mins.last().map_or(val, |v| cmp::min(*v, val)));
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.mins.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}
