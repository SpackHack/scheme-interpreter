pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub const fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.data.pop();
    }
    pub fn push(&mut self, ob: T) {
        self.data.push(ob);
    }

    pub fn get_length(&self) -> i64 {
        self.data.len() as i64
    }

    pub fn remove(&mut self, index: i64) -> Option<T> {
        if index < self.data.len() as i64 {
            return Some(self.data.remove(index as usize));
        }
        None
    }
}
