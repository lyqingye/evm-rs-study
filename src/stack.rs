use alloy_primitives::U256;

pub struct Stack {
    pub stack: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::with_capacity(1024),
        }
    }

    pub fn push(&mut self, value: U256) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> U256 {
        self.stack.pop().unwrap()
    }

    pub fn pop_n<const N: usize>(&mut self) -> [U256; N] {
        let mut values: Vec<_> = (0..N).map(|_| self.pop()).collect();
        let result: [_; N] = values
            .try_into()
            .expect("Expected the correct number of elements");
        result
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> U256 {
        self.stack[self.stack.len() - 1].clone()
    }

    pub fn dup(&mut self, n: usize) {
        self.push(self.stack[self.stack.len() - n].clone())
    }

    pub fn swap(&mut self, n: usize) {
        let len = self.stack.len();
        let tmp = self.stack[len - 1].clone();
        self.stack[len - 1] = self.stack[len - n - 1].clone();
        self.stack[len - n - 1] = tmp;
    }

    pub fn print_stack(&self) {
        println!("{:<10} {:<64}", "Index", "Value");
        println!("{:-<10} {:-<64}", "", "");

        let len = self.stack.len();
        let start = if len > 16 { len - 16 } else { 0 };

        for i in (start..len).rev() {
            println!("{:<10} {:<64x}", len - i, self.stack[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;

    #[test]
    fn test_swap() {
        let mut stack = Stack::new();
        stack.push(U256::from(1u64));
        stack.push(U256::from(2u64));
        stack.swap(1);
        assert_eq!(stack.stack[0], U256::from(2u64));
        assert_eq!(stack.stack[1], U256::from(1u64));
    }

    #[test]
    fn test_dup() {
        let mut stack = Stack::new();
        stack.push(U256::from(1u64));
        stack.dup(1);
        assert_eq!(stack.stack[0], U256::from(1u64));
        assert_eq!(stack.stack[1], U256::from(1u64));
    }
}
