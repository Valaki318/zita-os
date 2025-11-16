#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod output_driver;
use output_driver::Output;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut out = Output::new();

    for i in 1..=31 {
        writeln!(out, "{}", i).unwrap();
    }
    write!(out, "32").unwrap();
    
    loop {
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

