mod mini_projects;
use mini_projects::queue::Queue;
use mini_projects::fixed_size_ring_buffer::Ring_buffer;

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);

    println!("Front: {:?}", queue.peek());
    println!("Length: {}", queue.length());
    println!("Dequeued: {:?}", queue.dequeue());
}

// cd /workspaces for getting back to directory
