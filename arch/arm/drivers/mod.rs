/* driver::mod.rs */

use super::cpu::interrupt;
use super::cpu::uart;
use core::option::{Option, None};
use kernel;

pub fn init() {
    unsafe {
        kernel::int_table.map(|t| {
	        // See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0235c/index.html
            t.enable(interrupt::IRQ, keypress);
        });
    }
}

pub static mut keydown: Option<extern unsafe fn(char)> = None;
#[allow(dead_code)]
pub static mut read_char: Option<extern fn()->char> = None;

// this fires on any interrupt. we should only do UART0 stuff if it was a UART0 interrupt.

#[no_mangle]
pub unsafe fn keypress() {
	keydown.map(|f| {
		let x = *uart::UART0_DR as u8 as char;
		f(x)
	}
	);
    // Exception return instruction. [8]
    // TODO: better interrupt handler. r11 could change
    asm!("pop {r11, lr}
          subs pc, r14, #4"
        : /* out */
        : /* in */
        : "pc", "r11", "lr"  /* clobbers */) // pc = lr - 4
}
