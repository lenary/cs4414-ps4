/* driver::mod.rs */

use super::cpu::{interrupt, uart, kmi};
use core::mem::volatile_load;
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
pub static mut uart0_rec: Option<unsafe fn(char)> = None;

// A pointer to a rust function for dealing with KMI0 Data (PS/2 Keyboard)
pub static mut kmi0_rec:  Option<unsafe fn(u8)> = None;

// this fires on all Interrupts; so we need to check which interrupt triggered it,
#[no_mangle]
pub unsafe fn handle_irq() {
    let pic_status = volatile_load(interrupt::PIC_INT_STATUS);

    if ((pic_status & uart::UART0_INT) != 0) {
        uart0_rec.map(|f| {
            let x = volatile_load(uart::UART0_DR as *char);
            f(x)
        });
    }
    else if ((pic_status & interrupt::SIC_INT) != 0) {
        let sic_status = volatile_load(interrupt::SIC_INT_STATUS);

        if ((sic_status & kmi::KMI0_INT) != 0) {
            kmi0_rec.map(|f| {
                let x = volatile_load(kmi::KMI0_DR);
                f(x);
            });
        }
    }

    // Exception return instruction. [8]
    // TODO: better interrupt handler. r11 could change
    asm!("pop {r11, lr}
          subs pc, r14, #4"
        : /* out */
        : /* in */
        : "pc" "lr" "r11"  /* clobbers */) // pc = lr - 4
}
