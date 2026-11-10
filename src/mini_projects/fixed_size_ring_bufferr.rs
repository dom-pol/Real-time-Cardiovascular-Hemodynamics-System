pub struct Ring_Buffer<T, const N: usize>
//trait bound that tells rust that T must be able to create a default version of itself like[0,0,0]
where
    T: Default 
{
    //1. Fixed-size array pre-filled with default values
    storage: [Option<T>; N],
    //2. The index where the next item will be read
    head: usize,
    //3. The index where the next item will be written
    tail: usize,
    //4. Tracker to distinguish between completely empty and completely full
    is_full: bool,
}

impl<T, const N: usize> Ring_Buffer<T, N> { 
    //repeats the generic parameters so the compiler knows this implementation applies to any 
    pub fn new() -> Self {
        Self {
            storage: std::array::from_fn(|_| None),
            // assigns a value to storage field, santard library function that creates an array of size N by calling a closure for each index
            head: 0,
            // initializes write pointer at index 0
            tail: 0,
            // initializes read pointer at index 0
            is_full: false,
        }
    }

    pub fn push(&mut self, item: T) { //T to accept any Type
        self.storage[self.head] = Some(item);
        // pushes the item
        self.head = (self.head + 1) % N;
        // increments write pointer to next location whilst making sure it doesnt excede array size using modulus
        if self.is_full {
            self.tail = (self.tail + 1) % N;
        } else {
            if self.head == self.tail {
                self.is_full = true;
            }
        }
    } //closes push

    pub fn pop(&mut self) -> Option<T> { //Option<T>, function may return a value or not at all, (Optional T)
        if self.head == self.tail && self.is_full == false {
            return None;
        } else {
            let item = self.storage[self.tail].take();
            // .take() is a method called on an Option<T>, takes the value out of the Option and gives it to me, replaces the slot it left behind wiht None
            self.storage[self.tail] = None;
            self.tail = (self.tail + 1) % N;
            self.is_full = false;

            item //hands the Option<T> back because of -> at the start of the function, must return
        }
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail && !self.is_full //one boolean statement
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else if self.is_full() {
            N
        } else {
            let len = self.head + N;
            (len - self.tail) % N
        }
    }
} //closes impl