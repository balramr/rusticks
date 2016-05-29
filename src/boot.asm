; Declare constants used for creating a multiboot header.
MBALIGN     equ  1<<0
MEMINFO     equ  1<<1
FLAGS       equ  MBALIGN | MEMINFO
MAGIC       equ  0x1BADB002
CHECKSUM    equ -(MAGIC + FLAGS)

section .multiboot
align 4
	dd MAGIC
	dd FLAGS
	dd CHECKSUM

section .bootstrap_stack, nobits
align 4
stack_bottom:
resb 16384
stack_top:

; Define _start and other functions
section .text
global _start
global outb
global inb
global keyboard_handler
global load_idt
extern keyboard_handler_main
extern main

_start:
	cli
	call main
.hang:
    hlt
    jmp .hang
outb:
    mov edx, [esp + 4]
    mov al, [esp + 8]
    out dx, al      ; output al to port dx
    ret
inb:
    mov edx, [esp + 4]
    in al, dx       ; read port at dx into al
    ret

keyboard_handler:
    call keyboard_handler_main
	iretd

load_idt:
    mov edx, [esp + 4]
    lidt [edx]
    sti
    ret
