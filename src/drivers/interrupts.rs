use super::util::outb;
use super::util::inb;

const IDT_NUM_ENTRIES: usize = 256;
const IDT_INTERRUPT_GATE: u8 = 0x8e;       // refer to Intel manual/osdev

const KERNEL_CODE_SEGMENT_OFFSET: u16 = 0x08;      // in GDT

const PIC1_CMD_PORT: u16 = 0x20;
const PIC1_DATA_PORT: u16 = 0x21;
const PIC2_CMD_PORT: u16 = 0xA0;
const PIC2_DATA_PORT: u16 = 0xA1;

const PIC1_OFFSET: u8 = 0x20;
const PIC2_OFFSET: u8 = 0x28;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

pub static KEYS: &'static [u8] = b"\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

extern {
    fn load_idt(idt_ptr: u64);
    fn keyboard_handler();
}


#[repr(C)]
#[derive(Clone, Copy)]
struct InterruptGate {
    handler_offset_low: u16,
    segment_selector: u16,
    unused: u8,
    int_type: u8,
    handler_offset_high: u16
}

// TODO: how to work aronud having a dummy function?
#[allow(unused_variables)]
fn dummy(a: u8) {}
static mut keyboard_fn: fn(u8) = dummy;
static mut idt: [InterruptGate; IDT_NUM_ENTRIES] = [InterruptGate { handler_offset_low: 0,  segment_selector: 0, unused: 0, int_type: 0, handler_offset_high: 0 }; IDT_NUM_ENTRIES];


pub fn init() {
    unsafe {
        //
        // Init PIc
        //
        
        // ICW1
        outb(PIC1_CMD_PORT, ICW1_INIT);
        outb(PIC2_CMD_PORT, ICW1_INIT);

        // ICW2, set interrupt offsets
        outb(PIC1_DATA_PORT, PIC1_OFFSET);
        outb(PIC2_DATA_PORT, PIC2_OFFSET);

        outb(PIC1_DATA_PORT, 0x00);
        outb(PIC2_DATA_PORT, 0x00);        // ditto

        // ICW4, 8086 mode
        outb(PIC1_DATA_PORT, ICW4_8086);
        outb(PIC2_DATA_PORT, ICW4_8086);

        // Disable all IRQ lines for now
        outb(PIC1_DATA_PORT, 0xff);
        outb(PIC2_DATA_PORT, 0xff);
        
        //
        // Init IDT
        //
        let handler_addr = keyboard_handler as u32;
        idt[0x21].handler_offset_low = (handler_addr & 0xffff) as u16;
        idt[0x21].segment_selector = KERNEL_CODE_SEGMENT_OFFSET;
        idt[0x21].unused = 0;
        idt[0x21].int_type = IDT_INTERRUPT_GATE;
        idt[0x21].handler_offset_high = ((handler_addr & 0xffff0000) >> 16) as u16;
        
        let idt_addr: u32 = &(idt[0]) as *const InterruptGate as u32;
        let mut idt_ptr: [u32; 2] = [0; 2];

        idt_ptr[0] = (64 * IDT_NUM_ENTRIES as u32) + ((idt_addr & 0xffff) << 16);
        idt_ptr[1] = idt_addr >> 16;
        
        load_idt(&(idt_ptr[0]) as *const u32 as u64);
        
        // Init keyboard
        outb(0x21 , 0xFD);
        
    }
}

pub fn set_keyboard_fn(keyboard_function: fn(u8)) {
    unsafe {
        keyboard_fn = keyboard_function;
    }
}

pub fn keypress_main() {
    unsafe {
        outb(PIC1_CMD_PORT, PIC1_OFFSET);
        
        let status: u8 = inb(KEYBOARD_STATUS_PORT);
        
        if (status & 0x01) != 0 {
            let keycode: u8 = inb(KEYBOARD_DATA_PORT);
            if keycode < 0 as u8 {
                return;
            }
            match KEYS.get(keycode as usize) {
                Some(c) => keyboard_fn(*c),
                None => {}
            }
        }
    }

}
