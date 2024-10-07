use libc::{getrlimit, rlimit, RLIMIT_STACK};
use std::io;

fn main() -> io::Result<()> {
    let mut limit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    unsafe {
        if getrlimit(RLIMIT_STACK, &mut limit) != 0 {
            eprintln!("Failed to get stack size limit");
            return Err(io::Error::last_os_error());
        }
    }

    println!("Current stack size limit: {} bytes", limit.rlim_cur);
    println!("Maximum stack size limit: {} bytes", limit.rlim_max);
    large_stack();
    Ok(())
}

fn large_stack() {
    // Attempt to allocate a large array on the stack
    let _large_array: [u8; 16 * 1024 * 1024] = [0; 16 * 1024 * 1024];
    println!("Successfully allocated a 16MB array on the stack");
}
