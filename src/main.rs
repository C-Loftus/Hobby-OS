#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hobby_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hobby_os::{println, print};

// at keyboard input

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    hobby_os::init(); 

    // let f= hobby_os::graphics::farbfeld::new();
    // f.unwrap().display();
    // hobby_os::graphics::hello_world_graphics();
    
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    hobby_os::hlt_loop();            
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hobby_os::hlt_loop();            
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hobby_os::test_panic_handler(info)
}


