extern {
    pub fn outb(port: u16, out: u8);
    pub fn inb(port: u16) -> u8;
}