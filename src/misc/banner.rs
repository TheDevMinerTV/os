use crate::vga::{self, print, println, Color};

const BANNER: [[u8; 19]; 5] = [
    [b' '; 19],
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
    [b' '; 19],
];

pub fn print_banner() {
    vga::restore_colors(|| {
        vga::set_colors((Color::Pink, Color::Black));

        for (y, line) in BANNER.iter().enumerate() {
            vga::set_coords(31, y);

            for &byte in line.iter() {
                print!("{}", byte as char)
            }

            println!();
        }
    });
}
