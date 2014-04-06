
/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
#[inline]
static UART0: u32 = 0x101f1000;

pub static UART0_INT:  u32 = 1 << 12;
pub static UART0_RXIM: u16 = 1 <<  4;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0183f/I18381.html */
pub static UART0_DR: *mut u32 = (UART0 + 0x00) as *mut u32;
pub static UART0_IMSC: *mut u16 = (UART0 + 0x038) as *mut u16;
