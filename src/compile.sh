rustc -O --target i686-unknown-linux-gnu --crate-type lib -o main.o --emit obj main.rs
ld -m elf_i386 -T linker.ld -o rusticks.bin boot.o main.o
