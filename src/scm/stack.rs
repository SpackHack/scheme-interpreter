pub struct Stack<T> {
    data: Vec<T>,
}

//TODO: resize stack
impl<T> Stack<T> {
    pub const fn new(size: usize) -> Self {
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

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
