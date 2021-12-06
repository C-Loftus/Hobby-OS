#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hobby_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hobby_os::println;

// at cpu exceptions

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    hobby_os::init(); 

    
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    hello_world_graphics();
    loop {
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hobby_os::test_panic_handler(info)
}


pub fn hello_world_graphics() -> () {
    use vga::colors::Color16;
    use vga::writers::{Graphics640x480x16, GraphicsWriter};

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);
    for (offset, character) in "Hello World!".chars().enumerate() {
        mode.draw_character(270 + offset * 8, 72, character, Color16::White)
    }
}