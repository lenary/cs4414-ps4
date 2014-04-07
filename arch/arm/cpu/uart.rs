/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/BBABEGGE.html */
#[inline]
static UART0: u32 = 0x101f1000;

pub static UART0_INT:  u32 = 1 << 12;
pub static UART0_RXIM: u16 = 1 <<  4;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0183f/I18381.html */
pub static UART0_DR:  *mut char = (UART0 + 0x00) as *mut char;
pub static UART0_CR:   *mut u16 = (UART0 + 0x30) as *mut u16;
pub static UART0_IMSC: *mut u16 = (UART0 + 0x38) as *mut u16;