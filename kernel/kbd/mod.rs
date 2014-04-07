use core::option::{Option, Some, None};

static mut SHIFTED: bool = false;

fn is_A_Z(c: u8) -> bool {
    c >= 65 && c <= 90
}

pub unsafe fn parse_kmi_key(c: u8) -> Option<u8> {
    let c_ascii: Option<u8> = match c {
        /*
        0x2A => { SHIFTED = true; None},
        0x36 => { SHIFTED = true; None},
        0xAA => { SHIFTED = false; None },
        0xB6 => { SHIFTED = false; None },
        0x02 if SHIFTED => Some('!' as u8), // 1 pressed 
        0x03 if SHIFTED => Some('@' as u8), // 2 pressed 
        0x04 if SHIFTED => Some('#' as u8), // 3 pressed 
        0x05 if SHIFTED => Some('$' as u8), // 4 pressed 
        0x06 if SHIFTED => Some('%' as u8), // 5 pressed 
        0x07 if SHIFTED => Some('^' as u8), // 6 pressed 
        0x08 if SHIFTED => Some('&' as u8), // 7 pressed 
        0x09 if SHIFTED => Some('*' as u8), // 8 pressed 
        0x0A if SHIFTED => Some('(' as u8), // 9 pressed 
        0x0B if SHIFTED => Some(')' as u8), // 0 (zero) pressed 
        0x0C if SHIFTED => Some('-' as u8), // - pressed 
        0x27 if SHIFTED => Some(':' as u8),     //  ; pressed 
        0x28 if SHIFTED => Some('"' as u8), // ' (single quote) pressed 
        0x29 if SHIFTED => None, // TODO: We have no ~ character
        0x2B if SHIFTED => Some('|' as u8),     // \ pressed 
        0x33 if SHIFTED => Some('<' as u8), // , pressed 
        0x34 if SHIFTED => Some('>' as u8),     // . pressed 
        0x35 if SHIFTED => Some('?' as u8),     // / pressed 
        */
        0x01 => Some(27),     // escape pressed 
        0x02 => Some(49),     // 1 pressed 
        0x03 => Some(50),     // 2 pressed 
        0x04 => Some(51),     // 3 pressed 
        0x05 => Some(52),     // 4 pressed 
        0x06 => Some(53),     // 5 pressed 
        0x07 => Some(54),     // 6 pressed 
        0x08 => Some(55),     // 7 pressed 
        0x09 => Some(56),     // 8 pressed 
        0x0A => Some(57),     // 9 pressed 
        0x0B => Some(48),     // 0 (zero) pressed 
        0x0C => Some(45),     // - pressed 
        0x0D => None,         // #NAME?
        0x0E => Some(8),      //, backspace pressed 
        0x0F => Some(9),      // tab pressed 
        0x10 => Some(81),     // Q pressed 
        0x11 => Some(87),     // W pressed 
        0x12 => Some(69),     // E pressed 
        0x13 => Some(82),     // R pressed 
        0x14 => Some(84),     // T pressed 
        0x15 => Some(89),     // Y pressed 
        0x16 => Some(85),     // U pressed 
        0x17 => Some(73),     // I pressed 
        0x18 => Some(79),     // O pressed 
        0x19 => Some(80),     // P pressed 
        0x1A => Some(91),     // [ pressed 
        0x1B => Some(93),     // ] pressed 
        0x1C => Some(13),     // enter pressed 
        0x1D => None,         // TODO: control pressed
        0x1E => Some(65),     // A pressed 
        0x1F => Some(83),     // S pressed 
        0x20 => Some(68),     // D pressed 
        0x21 => Some(70),     // F pressed 
        0x22 => Some(71),     // G pressed 
        0x23 => Some(72),     // H pressed 
        0x24 => Some(74),     // J pressed 
        0x25 => Some(75),     // K pressed 
        0x26 => Some(76),     // L pressed 
        0x27 => Some(59),     //  ; pressed 
        0x28 => Some(39), // ' (single quote) pressed 
        0x29 => Some(96),     // ` (back tick) pressed 
        0x2B => Some(92),     // \ pressed 
        0x2C => Some(90),     // Z pressed 
        0x2D => Some(88),     // X pressed 
        0x2E => Some(67),     // C pressed 
        0x2F => Some(86),     // V pressed 
        0x30 => Some(66),     // B pressed 
        0x31 => Some(78),     // N pressed 
        0x32 => Some(77),     // M pressed 
        0x33            => Some(44), // , pressed 
        0x34 => Some(46),     // . pressed 
        0x39 => Some(32),     // space pressed 
        _ => None
    };
    
    match SHIFTED {
        false => { match c_ascii { // uppercase
            Some(lc_c) => {
                if is_A_Z(lc_c) {
                    Some(lc_c + 32)
                }
                else {
                    Some(lc_c)
                }
            }
            None => {
                None
            }
        }}
        true => c_ascii,
    }
}
