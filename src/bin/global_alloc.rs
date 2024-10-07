use memory_allocator_performance_rs::SimpleAlloc;

#[global_allocator]
static ALLOCATOR: SimpleAlloc = SimpleAlloc::new();

fn main() {
    let mut s = String::new();
    s.push_str("hello world");

    let currently = ALLOCATOR.offset.load(std::sync::atomic::Ordering::Relaxed);
    println!("allocated so far: {}", currently);
}
