#![feature(asm)]

const SYS_WRITE: i32 = 1;
const STDOUT: i32 = 1;
static GREETING: &'static str = "Hello, world!\n";

fn main() {
    unsafe {
        asm!("syscall"
            :
            : "{rax}"(SYS_WRITE), "{rdi}"(STDOUT), "{rsi}"(GREETING.as_ptr()), "{rdx}"(GREETING.len())
            : "rax");
    }
}
