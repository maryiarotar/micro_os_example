#![no_std]
#![no_main]
// #![feature(start)]
//extern crate libc;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> !{
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

//for creating a minimal binary that runs on top of an existing operating system, including libc:
//<unstable feature>
// #[start]
// fn start(_argc: isize, _argv: *const *const u8) -> isize {
//     0
// }

/*
//for compilation without host OS:
//rustup target add thumbv7em-none-eabihf
//cargo build --target thumbv7em-none-eabihf

----or add this to .cargo/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]

[target.'cfg(target_os = "windows")']
rustflags = ["-C", "link-args=/ENTRY:_start /SUBSYSTEM:console"]

[target.'cfg(target_os = "macos")']
rustflags = ["-C", "link-args=-e __start -static -nostartfiles"]
*/