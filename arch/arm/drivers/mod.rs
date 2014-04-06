/* driver::mod.rs */

use super::cpu::{interrupt, uart, kmi};
use core::option::{Option, None};
use kernel;

pub fn init() {
    unsafe {
        kernel::int_table.map(|t| {
	        // See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0235c/index.html
            t.enable(interrupt::IRQ, handle_irq);
        });
    }
}

// A pointer to a rust function for dealing with UART0 Data
pub static mut uart0_rec: Option<extern unsafe fn(char)> = None;

// A pointer to a rust function for dealing with KMI0 Data (PS/2 Keyboard)
pub static mut kmi0_rec:  Option<extern unsafe fn(char)> = None;

// this fires on all Interrupts; so we need to check which interrupt triggered it,
#[no_mangle]
pub unsafe fn handle_irq() {

    // break here hopefully

    if ((*interrupt::PIC_INT_STATUS & uart::UART0_INT) > 0) {
        // UART0 interrupt!
        uart0_rec.map(|f| {
    		let x = *uart::UART0_DR as u8 as char;

            *interrupt::PIC_INT_ENCLEAR = uart::UART0_INT;
    		f(x)
        });
    }

/*    if ((*interrupt::PIC_INT_STATUS & interrupt::SIC_INT) > 0) {

        kmi0_rec.map(|f| {
            let x = *kmi::KMI0_DATA as u8 as char;

            *interrupt::PIC_INT_ENCLEAR = interrupt::SIC_INT;
            f(x);
        });
    }*/

    // Exception return instruction. [8]
    // TODO: better interrupt handler. r11 could change
    asm!("pop {r11, lr}
          subs pc, r14, #4"
        : /* out */
        : /* in */
        : "pc", "r11", "lr"  /* clobbers */) // pc = lr - 4
}
