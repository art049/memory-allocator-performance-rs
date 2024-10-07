#![feature(allocator_api)]

use std::ptr::addr_of_mut;

use memory_allocator_performance_rs::ArenaAllocator;
use memory_allocator_performance_rs::VerboseAllocator;

static mut STATIC_ARENA_MEM: [u8; 1024] = [0; 1024];
fn main() {
    let allocator = VerboseAllocator::new(ArenaAllocator::from_ptr(unsafe {
        addr_of_mut!(STATIC_ARENA_MEM)
    }));
    println!("Creating a new Vec with VerboseAllocator");
    let mut v = Vec::new_in(&allocator);
    println!("Pushing 1u8 to the Vec");
    v.push(1u8);
    println!("Pushing 2u8 to the Vec");
    v.push(2u8);
    println!("Dropping the Vec");
    drop(v);

    println!("Creating a new Vec with capacity 100");
    let mut v = Vec::with_capacity_in(100, &allocator);
    println!("Pushing 1u8 to the Vec");
    v.push(1u8);
    println!("Extending the Vec with 100 elements");
    v.extend((0..100).map(|x| x as u8));
    println!("Shrinking to fit");
    v.shrink_to_fit();
    println!("Vec goes out of scope");
}
