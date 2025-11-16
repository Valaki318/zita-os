use volatile::Volatile;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//#[repr(u8)]
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FullColor {
    foreground: Color,
    background: Color,
    blinking: bool
}

impl FullColor {
    pub fn new(foreground: Color, background: Color, blinking: bool) -> FullColor {
        FullColor { foreground, background, blinking }
    }

    fn get_color_code(&self) -> u8 {
        (self.blinking as u8) << 5 | (self.background as u8) << 4 | (self.foreground as u8)
    }

    pub fn default() -> FullColor {
        FullColor { foreground: Color::White, background: Color::Black, blinking: false }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Character (u16);

impl Character {
    fn new(char: char, visuals: FullColor) -> Character {
        Character((visuals.get_color_code() as u16) << 8 | (char as u16))
    }

    fn empty() -> Character {
        Character(0)
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Output {
    column: usize,
    row: usize,
    current_visuals: FullColor,
    bufffer: &'static mut Buffer
}

impl Output {
    fn clear_row(&mut self, row: usize) {
        if row >= BUFFER_HEIGHT {
            panic!("Trying to clear buffer line out of range");
        }

        for col in 0..BUFFER_WIDTH {
            self.bufffer.chars[row][col].write(Character::empty());
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn new_line(&mut self) {
        self.row += 1;
        if self.row >= BUFFER_HEIGHT {
            self.row = BUFFER_HEIGHT -1;
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let copy_char = self.bufffer.chars[row][col].read();
                    self.bufffer.chars[row-1][col].write(copy_char);
                }
            }
            self.clear_row(self.row);
        }
        self.column = 0;
    }

    fn move_cursor(&mut self) {
        self.column += 1;
        if self.column >= BUFFER_WIDTH {
            self.new_line();
        }
    }

    fn write_char(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            ch => {
                let next_char = Character::new(ch, self.current_visuals);
                self.bufffer.chars[self.row][self.column].write(next_char);
                self.move_cursor();
            }
        }
    }

    pub fn new() -> Output {
        Output { 
            column: 0, 
            row: 0, 
            current_visuals: FullColor::default(), 
            bufffer:  unsafe { &mut *(0xb8000 as *mut Buffer) } 
        }
    }

    pub fn change_visuals(&mut self, visuals: FullColor) {
        self.current_visuals = visuals;
    } 

    pub fn print(&mut self, text: &str) {
        for by in text.bytes() {
            match by {
                0x20..=0x7e | b'\n' => self.write_char(by as char),
                _ => self.write_char(0xfe as char),
            }
        }
    }

    pub fn println(&mut self, text: &str) {
        self.print(text);
        self.print("\n");
    }
}

impl fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}



