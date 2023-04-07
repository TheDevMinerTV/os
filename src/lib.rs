#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod cpu;
mod mem;
mod misc;
mod vga;

use crate::misc::banner;
use core::arch::asm;
use core::panic::PanicInfo;

use misc::klog::{kdbg, kinfo};
use vga::{print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();

    vga::set_colors((vga::Color::White, vga::Color::Red));

    println!("================================================================================");
    println!("KERNEL PANIC:");
    println!("{}", info);
    println!("================================================================================");
    println!("Clearing interrupts and halting CPU...");

    unsafe { asm!("hlt") };

    println!("CPU halted, this should never happen!");

    loop {}
}

#[no_mangle]
pub extern "C" fn _rust_main(mb_magic: usize, mb_addr: usize) {
    vga::clear_screen();
    banner::print_banner();

    if mb_magic != 0x36d76289 {
        panic!("Invalid multiboot magic, 0x{:08X} != 0x36d76289", mb_magic);
    }

    kinfo!("Received multiboot2 information at: {:?}", mb_addr);
    let boot_info = unsafe { multiboot2::load(mb_addr).unwrap() };

    let memory_map = boot_info.memory_map_tag().unwrap();
    {
        let mut available = 0;

        kdbg!("Available memory areas:");
        for area in memory_map
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
    }

    let (kernel, multiboot) = {
        let elf_sections = boot_info.elf_sections_tag().unwrap();

        kdbg!("ELF sections:");
        for section in elf_sections.sections() {
            kdbg!(
                "  0x{:08X} - 0x{:08X} ({} bytes, {:?})",
                section.start_address(),
                section.end_address(),
                section.size(),
                section.flags(),
            );
        }

        let kernel_start = elf_sections
            .sections()
            .map(|s| s.start_address())
            .min()
            .unwrap();
        let kernel_end = elf_sections
            .sections()
            .map(|s| s.end_address())
            .max()
            .unwrap();

        let mb_start = mb_addr;
        let mb_end = mb_addr + boot_info.total_size();

        kinfo!(
            "Kernel loaded at 0x{:08X} - 0x{:08X} ({} bytes)",
            kernel_start,
            kernel_end,
            kernel_end - kernel_start
        );
        kinfo!(
            "Multiboot2 information at 0x{:08X} - 0x{:08X} ({} bytes)",
            mb_start,
            mb_end,
            mb_end - mb_start
        );

        (
            (kernel_start as usize, kernel_end as usize),
            (mb_start as usize, mb_end as usize),
        )
    };

    kdbg!("Running on CPU: {:?}", cpu::cpuid::Basic::read());
    kdbg!("  {:?}", cpu::cpuid::Extended::read());

    let mut frame_allocator =
        mem::AreaFrameAllocator::new(kernel, multiboot, memory_map.memory_areas());
    kdbg!("Initialized frame allocator");

    print!("[INFO] Color test: ");
    for row in 0..16 {
        let bg: vga::Color = row.into();
        let fg = bg.inverse();

        vga::set_colors((fg, bg));
        print!(" {:?} + {:?} ", fg, bg);
    }
    vga::set_colors((vga::Color::White, vga::Color::Black));
    println!();

    todo!("do things");
}
