use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaBufferChar {
    ascii: u8,
    color_code: ColorCode,
}

impl VgaBufferChar {
    pub const fn new_ascii(ascii: u8, foreground: Color, background: Color) -> Self {
        Self {
            ascii,
            color_code: ColorCode::new(foreground, background),
        }
    }
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
const EMPTY_LINE: [VgaBufferChar; BUFFER_WIDTH] =
    [VgaBufferChar::new_ascii(b' ', Color::White, Color::Black); BUFFER_WIDTH];

pub struct VgaBufferWriter {
    pos_x: usize,
    pos_y: usize,
    foreground: Color,
    background: Color,
    vga_buffer: *mut VgaBufferChar,
    pending_newline: bool,
}

impl VgaBufferWriter {
    pub const fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            foreground: Color::White,
            background: Color::Black,
            vga_buffer: 0xb8000 as *mut VgaBufferChar,
            pending_newline: false,
        }
    }

    pub fn write_single(&mut self, byte: u8) -> &mut Self {
        match byte {
            b'\n' => self.fill_line().new_line(),
            byte => {
                if self.pending_newline {
                    self.fill_line().new_line();
                }

                self.write_single_at(
                    self.pos_x,
                    self.pos_y,
                    VgaBufferChar {
                        ascii: byte,
                        color_code: ColorCode::new(self.foreground, self.background),
                    },
                );

                self.pos_x += 1;

                if self.pos_x >= BUFFER_WIDTH {
                    self.pending_newline = true;
                }
            }
        };

        self
    }

    fn write_single_at(&mut self, x: usize, y: usize, c: VgaBufferChar) {
        assert!(x < BUFFER_WIDTH);
        assert!(y < BUFFER_HEIGHT);

        let base_idx = x + y * BUFFER_WIDTH;

        unsafe {
            self.vga_buffer.add(base_idx).write_volatile(c);
        }
    }

    pub fn write<S: AsRef<str>>(&mut self, s: S) -> &mut Self {
        let s = s.as_ref();

        for b in s.bytes() {
            match b {
                0x20..=0x7e | b'\n' => self.write_single(b),
                _ => self.write_single(0xFE),
            };
        }

        self
    }

    pub fn clear_screen(&mut self) {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let base_idx = y * BUFFER_WIDTH + x;

                unsafe {
                    self.vga_buffer.add(base_idx).write_volatile(VgaBufferChar {
                        ascii: b' ',
                        color_code: ColorCode::new(self.foreground, self.background),
                    });
                }
            }
        }
    }

    pub fn scroll_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            unsafe {
                let src = (self.vga_buffer.add(BUFFER_WIDTH * (row + 1)))
                    as *mut [VgaBufferChar; BUFFER_WIDTH];
                let dest =
                    (self.vga_buffer.add(BUFFER_WIDTH * row)) as *mut [VgaBufferChar; BUFFER_WIDTH];

                core::ptr::copy_nonoverlapping(src, dest, 1);
            }
        }

        unsafe {
            let dest = (self.vga_buffer.add(BUFFER_WIDTH * (BUFFER_HEIGHT - 1)))
                as *mut [VgaBufferChar; BUFFER_WIDTH];

            core::ptr::copy_nonoverlapping(&EMPTY_LINE, dest, 1);
        }
    }

    pub fn new_line(&mut self) {
        self.pending_newline = false;

        self.pos_x = 0;
        self.pos_y += 1;

        if self.pos_y >= BUFFER_HEIGHT {
            self.scroll_line();
            self.pos_y = BUFFER_HEIGHT - 1;
        }
    }

    pub fn fill_line(&mut self) -> &mut Self {
        for x in self.pos_x..BUFFER_WIDTH {
            self.write_single_at(
                x,
                self.pos_y,
                VgaBufferChar {
                    ascii: b' ',
                    color_code: ColorCode::new(self.foreground, self.background),
                },
            );
        }

        // self.new_line();

        self
    }

    pub fn write_char_slice(&mut self, slice: &[char]) -> &mut Self {
        for c in slice {
            self.write_single(*c as u8);
        }

        self
    }

    pub fn foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;
        self
    }

    pub fn background(&mut self, color: Color) -> &mut Self {
        self.background = color;
        self
    }
}

impl fmt::Write for VgaBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
