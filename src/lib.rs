#![no_std]
#![no_main]

mod cpu;
#[macro_use]
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        println!();

        vga::WRITER
            .foreground(vga::Color::White)
            .background(vga::Color::Red)
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

    println!(
        "Running on CPU: {:?} ({:?})\n",
        cpu::basic_cpuid(),
        cpu::advanced_cpuid()
    );

    println!("Color test:");
    for row in 0..16 {
        let bg: vga::Color = row.into();
        let fg = bg.inverse();

        vga::WRITER.foreground(fg).background(bg);
        print!(" {:?} + {:?} ", fg, bg);
    }
    vga::WRITER
        .foreground(vga::Color::White)
        .background(vga::Color::Black)
        .write("\n");

    todo!("do things");
}
