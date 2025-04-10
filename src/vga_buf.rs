use volatile::Volatile;

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

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

    fn newline(&mut self) {
        todo!()
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

pub fn print_something() {
    let c_code = ColorCode::new(Color::Black, Color::Yellow);
    let mut writer = Writer {
        col_position: 0,
        color_code: c_code,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello, ");
    writer.write_string("from PhrenOS");
}
