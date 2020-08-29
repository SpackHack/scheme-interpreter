pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub const fn new(size: usize) -> Self {
        Stack {
            data: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> T {
        return self.data.pop().unwrap();
    }
    pub fn push(&mut self, ob: T) {
        self.data.push(ob);
    }
}
