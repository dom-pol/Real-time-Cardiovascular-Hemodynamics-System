mod mini_projects;
//use mini_projects::queue::Queue;
use mini_projects::fixed_size_ring_buffer::Ring_Buffer;

fn main() {
    let mut buffer: Ring_Buffer<i32, 5> = Ring_Buffer::new();
    buffer.push(10);
    buffer.push(20);
    buffer.push(30);
    buffer.push(40);
    buffer.push(50);

    while let Some(item) = buffer.pop() {
        println!("Popped: {}", item);
        }
}


// cd /workspaces for getting back to directory

// ==========================================
// ARCHIVED: Previous Queue Testing
// ==========================================
#[allow(dead_code)]
fn archived_queue_tests() {
    // let mut queue = Queue::new();
    // queue.enqueue(10);
    // queue.enqueue(20);
    // 
    // println!("Front: {:?}", queue.peek());
    // println!("Length: {}", queue.length());
    // println!("Dequeued: {:?}", queue.dequeue());
}