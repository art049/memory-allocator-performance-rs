use std::io::{Read, Write};

const POOL_SIZE: usize = 32;
static mut MEMORY_POOL: [u8; POOL_SIZE] = [0; POOL_SIZE]; // Statically allocated buffer

fn read_line_to_static_memory_pool() -> &'static str {
    let mut buffer = [0; 1]; // Temporary buffer to read one byte at a time
    let mut i = 0;
    loop {
        let n = std::io::stdin().read(&mut buffer).unwrap();
        if n == 0 || buffer[0] == b'\n' {
            break;
        }
        unsafe { MEMORY_POOL[i] = buffer[0] }; // Safely write to the static memory pool
        i += 1;
    }
    unsafe { std::str::from_utf8_unchecked(&MEMORY_POOL[0..i]) }
}

fn main() {
    println!("Enter your address:");
    std::io::stdout().flush().unwrap();

    let address = read_line_to_static_memory_pool();
    println!("You entered: {}", address);
}
