use spin::Mutex;
use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Used to have 1 handler for output
lazy_static! {
    pub static ref OUTPUT: Mutex<Output> = Mutex::new(Output::new());
}

// Enum representing color of output
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// Enum representing color of background, foreground, and blinking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Visuals {
    foreground: Color,
    background: Color,
    blinking: bool
}

impl Visuals {
    // Creates new Visuals with given details
    pub fn new(foreground: Color, background: Color, blinking: bool) -> Visuals {
        Visuals { foreground, background, blinking }
    }

    // Returns the number of the visuals according to x86_64 convention
    fn get_color_code(&self) -> u8 {
        (self.blinking as u8) << 5 | (self.background as u8) << 4 | (self.foreground as u8)
    }

    // Returns visuals with black background, white foreground, and no blinking
    pub fn default() -> Visuals {
        Visuals { foreground: Color::White, background: Color::Black, blinking: false }
    }

}

// Represents character in the VGA buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Character (u16);

impl Character {
    // Creates character with given details
    fn new(char: char, visuals: Visuals) -> Character {
        Character((visuals.get_color_code() as u16) << 8 | (char as u16))
    }

    // Creates an empty character
    fn empty(visuals: Visuals) -> Character {
        Character::new(0 as char, visuals)
    }
}

// Buffer corresponding to the memory of the VGA output
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Structure handling output
pub struct Output {
    column: usize,
    row: usize,
    current_visuals: Visuals,
    bufffer: &'static mut Buffer
}

impl Output {
    // Clears the given row of the VGA buffer
    fn clear_row(&mut self, row: usize) {
        if row >= BUFFER_HEIGHT {
            panic!("Trying to clear buffer line out of range");
        }

        for col in 0..BUFFER_WIDTH {
            self.bufffer.chars[row][col].write(Character::empty(self.current_visuals));
        }
    }

    // Clears the VGA buffer
    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    // Moves the cursor to a new line for printing
    fn new_line(&mut self) {
        self.row += 1;

        // If it is the last row, move every other row up
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

    // Moves teh cursor to the right
    fn move_cursor(&mut self) {
        self.column += 1;
        if self.column >= BUFFER_WIDTH {
            self.new_line();
        }
    }

    // Writes given char at the current location of the cursor
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

    // Creates new Output handler struct
    pub fn new() -> Output {
        Output { 
            column: 0, 
            row: 0, 
            current_visuals: Visuals::default(), 
            bufffer:  unsafe { &mut *(0xb8000 as *mut Buffer) } 
        }
    }

    // Changes printing appearence
    pub fn change_visuals(&mut self, visuals: Visuals) {
        self.current_visuals = visuals;
    } 

    // prints a &str
    pub fn print(&mut self, text: &str) {
        for by in text.bytes() {
            match by {
                0x20..=0x7e | b'\n' => self.write_char(by as char),
                _ => self.write_char(0xfe as char),
            }
        }
    }

}

// Needed for writing formatted output
impl fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

// Print macro implemented in context
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::output_driver::_print(format_args!($($arg)*)));
}

// Println macro implemented in context
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Print method using static reference to OUTPUT
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    OUTPUT.lock().write_fmt(args).unwrap();
}

// Macro to clear VGA buffe
#[macro_export]
macro_rules! clear {
    () => {
        $crate::output_driver::_clear()
    };
}

// Cears the VGA buffer using the static reference to OUTPUT
#[doc(hidden)]
pub fn _clear() {
    OUTPUT.lock().clear();
}

#[test_case]
pub fn tets_println() {
    let string = "This is a test string";
    println!("{}", string);

    for (i, ch) in string.chars().enumerate() {
        let test_row = OUTPUT.lock().row - 1;
        let test_ch = OUTPUT.lock().bufffer.chars[test_row][i].read();
        assert_eq!(test_ch, Character::new(ch, OUTPUT.lock().current_visuals));
    }

}



