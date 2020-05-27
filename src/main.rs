#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again!").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {} \n", 42, 1.337).unwrap();

    println!("Hello There {}", "General Kenobi");

    panic!("Something's wrong I can feel it");
    loop {}
}
