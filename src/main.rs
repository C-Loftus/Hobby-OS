#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hobby_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hobby_os::{println, print};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hobby_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr, structures::paging::Page};

    println!("Hello World{}", "!");
    hobby_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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


