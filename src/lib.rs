#![no_std]
#![no_main]

mod cpu;
#[macro_use]
mod vga;
mod misc;

use crate::misc::banner;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        println!();

        vga::WRITER
            .set_foreground(vga::Color::White)
            .set_background(vga::Color::Red)
            .write(
                "================================================================================",
            );

        println!("KERNEL PANIC:");
        println!("{}", info);

        vga::WRITER.fill_line().write(
            "================================================================================",
        );
    };

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _rust_main() {
    vga::WRITER.clear_screen();
    banner::print_banner();

    println!(
        "Running on CPU: {:?} ({:?})\n",
        cpu::basic_cpuid(),
        cpu::advanced_cpuid()
    );

    println!("Color test:");
    for row in 0..16 {
        let bg: vga::Color = row.into();
        let fg = bg.inverse();

        vga::WRITER.set_foreground(fg).set_background(bg);
        print!(" {:?} + {:?} ", fg, bg);
    }
    vga::WRITER
        .set_foreground(vga::Color::White)
        .set_background(vga::Color::Black)
        .write("\n");

    todo!("do things");
}
