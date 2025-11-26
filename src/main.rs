#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zita_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zita_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Hello world");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zita_os::test_panic_handler(info)
}




