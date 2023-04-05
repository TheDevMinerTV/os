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
    let colors = vga::colors();

    vga::set_colors((Color::Pink, Color::Black));

    println!();
    for (y, line) in BANNER.iter().enumerate() {
        vga::set_coords(31, y);

        for &byte in line.iter() {
            print!("{}", byte as char)
        }

        println!();
    }
    println!();

    vga::set_colors(colors);
}
