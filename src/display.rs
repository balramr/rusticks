use drivers::vga;

struct Position {
    x: u8,
    y: u8
}

static mut position: Position = Position {x: 0, y:0};
static mut foreground: vga::Color = vga::Color::Black;
static mut background: vga::Color = vga::Color::White;

pub fn init(fg: vga::Color, bg: vga::Color) {
    vga::init(bg);
    unsafe {
        position = Position {x: 0, y:0};
        foreground = fg;
        background = bg;
        vga::set_cursor(0, 0);
    }
}


pub fn print(text: &str) {
    for byte in text.bytes() {
        print_char(byte);
    }
}
    
pub fn print_char(c: u8) {
    unsafe {
        if c != b'\n' {
            vga::putchar(c, position.x, position.y, foreground, background);
            if position.x >= vga::X_WIDTH as u8 {
                position.x = 0;
                position.y += 1;
            } else {
                position.x += 1;
            }        
        } else {
            position.x = 0;
            position.y += 1;
        }
        if position.y >= vga::Y_HEIGHT as u8 && position.y == vga::X_WIDTH as u8 {
            position.x = 0;
            position.y = 0;
        }
        vga::set_cursor(position.x, position.y);
    }
}        

