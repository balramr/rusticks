VPATH=src:src/drivers
LD=ld
RC=rustc
RCFLAGS=-O --target i686-unknown-linux-gnu --crate-type lib --emit obj
NASM=nasm
QEMU=qemu-system-i386
ROBJ=main.o
SOURCES=src/main.rs
RUST_OBJ=build/main.o

all: build/rusticks.bin

$(RUST_OBJ): $(SOURCES)
	$(RC) $(RCFLAGS) -o $@ $<

build/boot.o: src/boot.asm
	$(NASM) -f elf -o $@ $<

build/rusticks.bin: $(RUST_OBJ) build/boot.o
	$(LD) -m elf_i386 -T src/linker.ld -o $@ $^	

run: build/rusticks.bin
	$(QEMU) -kernel $<

clean:
	rm build/*.o build/*.bin
