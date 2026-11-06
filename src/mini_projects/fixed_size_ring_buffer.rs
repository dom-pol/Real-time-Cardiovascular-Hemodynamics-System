// pub fn ring_buffer() {
//     let mut head = 
//     println!("The function works!");
// }

pub struct Queue<T> { //<T> is a rust vec that encapsulates internal data
    queue: Vec<T>
}

impl<T> Queue<T> { //attaches functionality to a struct
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) { //&mut self grants temporary mutable access to the stack in this scope
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn length(&self) -> usize { //looking up size or counts on object queue, means the function returns an unsigned pointer-size integer.
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}
