use core::fmt::{self, Write};

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_position: 0,
        color_code: ColorCode::new(Color::Black, Color::Yellow),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Copy, Debug)]
#[repr(transparent)] // ensure has u8 data layout
pub struct ColorCode(u8);

impl ColorCode {
    fn new(background: Color, foreground: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Copy, Clone)]
#[repr(C)] //  struct’s fields are laid out exactly like in a C struct 
pub struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
/* ensure that it has the same memory layout as its single field.*/
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}

pub struct Writer {
    col_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col_position >= BUF_WIDTH {
                    self.newline();
                }

                let row = BUF_HEIGHT - 1;
                let col = self.col_position;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });

                self.col_position += 1;
            }
        }
    }

    // move every character one line up (the top line gets deleted)
    // start at the beginning of the last line again.
    fn newline(&mut self) {
        for row in 1..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }

        self.clear_row(BUF_HEIGHT - 1);
        self.col_position = 0;
    }

    // overwrite all of its characters with a space character.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUF_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ascii or newline
                0x20..0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buf::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn t_simple() {
    println!("testing works");
}

#[test_case]
fn t_print_many() {
    for _ in 0..200 {
        println!("test writing many");
    }
}

#[test_case]
fn t_char_appear() {
    let s = "This is a basic line!";
    println!("{}", s);

    for (i, c) in s.chars().enumerate() {
        // Println appends a newline,
        // thus the string should appear on line BUFFER_HEIGHT - 2
        let s_char = WRITER.lock().buffer.chars[BUF_HEIGHT - 2][i].read();
        assert_eq!(char::from(s_char.ascii_char), c);
    }
}
