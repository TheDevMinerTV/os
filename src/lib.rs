#![no_std]
#![no_main]

mod cpu;
mod misc;
mod vga;

use crate::misc::banner;
use core::panic::PanicInfo;

use misc::klog::{kdbg, kinfo};
use vga::{print, println};

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
pub unsafe extern "C" fn _rust_main(mb_magic: usize, mb_addr: usize) {
    vga::WRITER.clear_screen();
    banner::print_banner();

    if mb_magic != 0x36d76289 {
        panic!("Invalid multiboot magic, 0x{:08X} != 0x36d76289", mb_magic);
    }

    kinfo!("Received multiboot2 information at: {:?}", mb_addr);
    let boot_info = multiboot2::load(mb_addr).unwrap();
    let memory_map_tag = boot_info.memory_map_tag().unwrap();

    let mut available = 0;

    kdbg!("Available memory areas:");
    for area in memory_map_tag
        .memory_areas()
        .filter(|a| a.typ() == multiboot2::MemoryAreaType::Available)
    {
        kdbg!(
            "  0x{:08X} - 0x{:08X} ({} bytes, {:?})",
            area.start_address(),
            area.end_address(),
            area.size(),
            area.typ(),
        );

        available += area.size();
    }

    kinfo!("Available memory: {} bytes", available);

    kinfo!(
        "Running on CPU: {:?} ({:?})",
        cpu::basic_cpuid(),
        cpu::advanced_cpuid()
    );

    kinfo!("Color test:");
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
