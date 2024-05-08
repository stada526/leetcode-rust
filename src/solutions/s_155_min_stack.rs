struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            Some(min) => {
                if val <= *min {
                    self.min_stack.push(val);
                }
            }
            None => {
                self.min_stack.push(val);
            }
        }
    }

    fn pop(&mut self) {
        match self.stack.pop() {
            Some(val) => {
                if val == self.get_min() {
                    self.min_stack.pop();
                }
            }
            None => return,
        }
    }

    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.min_stack.last().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
