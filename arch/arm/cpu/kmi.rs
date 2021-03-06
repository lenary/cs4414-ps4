
/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/CFHHIEIF.html */
#[inline]
static KMI0: u32 = 0x10006000;

pub static KMI0_INT: u32 = 1 << 3; // on secondary interrupt controller

pub static KMI0_ENABLE: u8 = 1 << 2;
pub static KMI0_RXIM:   u8 = 1 << 4;

/* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0143c/i1008380.html */
pub static KMI0_CR: *mut u8 = (KMI0 + 0x00) as *mut u8;
pub static KMI0_DR:     *u8 = (KMI0 + 0x08) as     *u8;
