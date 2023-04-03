use crate::vga::{self, print, println, Color};

const BANNER: [[u8; 19]; 3] = [
    [
        b'.', b'_', b'_', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'.', b'_', b'_', b'.',
        b' ', b'_', b'_', b'.',
    ],
    [
        b'|', b' ', b' ', b'\\', b' ', b'_', b' ', b'.', b' ', b' ', b',', b'|', b' ', b' ', b'|',
        b'(', b'_', b'_', b' ',
    ],
    [
        b'|', b'_', b'_', b'/', b'(', b'/', b',', b' ', b'\\', b'/', b' ', b'|', b'_', b'_', b'|',
        b'.', b'_', b'_', b')',
    ],
];

pub fn print_banner() {
    unsafe {
        let colors = vga::WRITER.colors();

        vga::WRITER.set_colors((Color::Pink, Color::Black));

        println!();
        for line in BANNER.iter() {
            vga::WRITER.set_pos_x(31);

            for &byte in line.iter() {
                print!("{}", byte as char)
            }

            println!();
        }
        println!();

        vga::WRITER.set_colors(colors);
    };
}
