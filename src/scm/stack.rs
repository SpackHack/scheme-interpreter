pub struct Stack<T> {
    data: Vec<T>,
}


//TODO: resize stack
impl<T> Stack<T> {
    pub const fn new(size: usize) -> Self {
        Stack {
            data: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.data.pop();
    }
    pub fn push(&mut self, ob: T) {
        self.data.push(ob);
    }
    pub fn get_length(&self) -> usize {
        self.data.len()
    }
}
