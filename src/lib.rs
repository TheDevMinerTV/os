#![no_std]
#![no_main]

mod cpu;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;

static mut VGA_BUFFER_WRITER: vga::VgaBufferWriter = vga::VgaBufferWriter::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        VGA_BUFFER_WRITER
            .foreground(vga::Color::White)
            .background(vga::Color::Red);

        VGA_BUFFER_WRITER.write(
            "================================================================================",
        );

        VGA_BUFFER_WRITER.write("KERNEL PANIC:\n");
        write!(VGA_BUFFER_WRITER, "{}\n", info).unwrap();

        VGA_BUFFER_WRITER.fill_line().write(
            "================================================================================",
        );
    };

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn _rust_main() {
    VGA_BUFFER_WRITER.clear_screen();

    write!(
        VGA_BUFFER_WRITER,
        "Running on CPU: {:?} ({:?})\n\n",
        cpu::basic_cpuid(),
        cpu::advanced_cpuid()
    )
    .unwrap();

    VGA_BUFFER_WRITER.write("Color test:\n");
    for row in 0..16 {
        let bg: vga::Color = row.into();
        let fg = bg.inverse();

        VGA_BUFFER_WRITER.foreground(fg).background(bg);
        write!(VGA_BUFFER_WRITER, "{:?} + {:?}\n", fg, bg).unwrap();
    }

    todo!("do things");
}
