//to build target for triple x86-64-unknown-none 

#![no_std]
#![no_main]
// #![feature(start)]
//extern crate libc;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_panic: &PanicInfo) -> !{
    loop {}
}


static HELLO: &[u8] = b"Mini Rust Os!";

//entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {

    let vga_buffer: *mut u8 = 0xb8000 as *mut u8; //raw pointer to vga buffer

    for (i, &byte) in HELLO.iter().enumerate(){

        unsafe { 
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }

    }

    loop {}
}








//for creating a minimal binary that runs on top of an existing operating system, including libc:
//<unstable feature>
// #[start]
// fn start(_argc: isize, _argv: *const *const u8) -> isize {
//     0
// }
