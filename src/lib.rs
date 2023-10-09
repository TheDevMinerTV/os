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
use multiboot2::BootInformation;
use strum::IntoEnumIterator;
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
    let boot_info_ptr = (mb_addr as *const u8).cast();
    let boot_info = unsafe { BootInformation::load(boot_info_ptr).unwrap() };

    let memory_map = boot_info.memory_map_tag().unwrap();
    {
        let mut available = 0;

        kdbg!("Available memory areas:");
        for area in memory_map
            .memory_areas()
            .iter()
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
        let elf_sections = boot_info.elf_sections().unwrap();

        kdbg!("ELF sections:");
        for section in elf_sections.clone() {
            kdbg!(
                "  0x{:08X} - 0x{:08X} ({} bytes, {:?})",
                section.start_address(),
                section.end_address(),
                section.size(),
                section.flags(),
            );
        }

        let kernel_start = elf_sections
            .clone()
            .map(|s| s.start_address())
            .min()
            .unwrap();
        let kernel_end = elf_sections.map(|s| s.end_address()).max().unwrap();

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
            (mb_start, mb_end),
        )
    };

    {
        let basic = cpu::cpuid::Basic::read();
        let extended = cpu::cpuid::Extended::read();

        kdbg!("CPU Information:");
        kdbg!("  Manufacturer: {}", basic.manufacturer);

        if let Some(brand) = extended.brand {
            kdbg!("  Brand: {}", brand);
        }

        if let Some(vendor) = extended.vendor {
            kdbg!("  Vendor: {}", vendor);
        }

        if let Some(info) = basic.basic_info {
            kdbg!("  Info:");
            kdbg!("    Type: {:?}", info.type_);
            kdbg!(
                "    Family: {:01X}h ({:02X}h)",
                info.family,
                info.extended_family
            );
            kdbg!(
                "    Model: {:01X}h ({:01X}h)",
                info.model,
                info.extended_model
            );
            kdbg!("    Stepping: {}", info.stepping);
        }

        print!("[DBG]    Capabilities: ");
        if let Some(info) = basic.info_and_bits {
            for (bit, ..) in info.edx.iter_names() {
                print!("{} ", bit);
            }
            for (bit, ..) in info.ecx.iter_names() {
                print!("{} ", bit);
            }
        }
        if let Some(info) = extended.info_and_bits {
            for (bit, ..) in info.edx.iter_names() {
                print!("{} ", bit);
            }
            for (bit, ..) in info.ecx.iter_names() {
                print!("{} ", bit);
            }
        }
        println!();

        if let Some(svm_revision) = extended.svm_revision {
            kdbg!("  SVM Revision: {}", svm_revision);
        }
    }

    let frame_allocator =
        mem::AreaFrameAllocator::new(kernel, multiboot, memory_map.memory_areas());
    kdbg!("Initialized frame allocator");

    vga::restore_colors(|| {
        print!("[INFO] Color test: ");
        for bg in vga::Color::iter() {
            let fg = bg.inverse();

            vga::set_colors((fg, bg));
            print!(" {:?} + {:?} ", fg, bg);
        }
    });
    println!();

    todo!("do things");
}
