/* kernel::sgash.rs */
#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None};
use core::iter::Iterator;
use kernel::*;
use kernel::cstr::cstr;
use super::super::platform::*;
use kernel::memory::Allocator;

static PROMPT: &'static str = &"sgash> ";
static PROMPT_COLOR: u32 = 0xFFAF00;
static mut count: uint = 0;

static mut buffer: cstr = cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};

static mut history: cstr = cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};

/* Thanks to https://github.com/jvns/puddle/blob/master/src/stdio.rs */
pub fn putnum(x: uint, max: uint) {
    let mut i = max;
    if i == 0 {
        i = 1000000;
    }
    while(i > 0) {
        // Get the offset from each decimal point up to a certain value
        putchar(((((x / i) % 10) as u8) + ('0' as u8)) as char);
        i /= 10;
    }
    putchar(' ');
}

pub fn putkeycode(x: u8) {
    putnum(x as uint, 100);
}

pub fn putchar(key: char) {
    unsafe {
        /*
        * We need to include a blank asm call to prevent rustc
        * from optimizing this part out
        */
        asm!("");
        io::write_char(key, io::UART0);
    }
}

pub unsafe fn showstr(msg: &str) {
    putstr(msg);
    drawstr(msg);
}

pub fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
        putchar(*c as char);
    }
}

pub unsafe fn drawstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
        drawchar(*c as char);
    }
}

unsafe fn drawchar(x: char) {
    io::restore();
    if ((x as u8) as uint) == 10 || x == '\n' {
        io::CURSOR_Y += io::CURSOR_HEIGHT;
        io::CURSOR_X = 0u32;
    } else {
        if (((x as u8) as uint) >= 32 && ((x as u8) as uint) <= 125) {
            io::draw_char(x);
            io::CURSOR_X += io::CURSOR_WIDTH;
        }
    }
    io::backup();
    io::draw_cursor();
}


unsafe fn backspace() {
    io::restore();
    if (io::CURSOR_X >= io::CURSOR_WIDTH) {
        io::CURSOR_X -= io::CURSOR_WIDTH;
        io::draw_char(' ');
    }
    io::backup();
    io::draw_cursor();
}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard

    match x {
        13 =>  {
            parse_buffer();
            prompt();
            buffer.reset();
        }
        127 =>  {
            putchar('');
            putchar(' ');
            putchar('');
            buffer.delete_char();
            backspace();
        }
        _ =>  {
            buffer.add_u8(x);
            if io::CURSOR_X < io::SCREEN_WIDTH-io::CURSOR_WIDTH {
                putchar(x as char);
                drawchar(x as char);
            }
        }
    }
}

pub unsafe fn parse_buffer() {
    showstr(&"\n");
    buffer.map(putchar);
    buffer.map(drawchar);
    showstr(&"\n");
    if buffer.streq(&"history") {
        history.map(drawchar);
        history.map(putchar);
    }
    else if buffer.len() > 0 {
        let nhistory = history.join(buffer);
        history = nhistory;
        heap.free(history.p);
        history.add_char('\n');
    }
    let (command, args) = buffer.split(' ');
    if command.streq(&"echo") {
        args.map(drawchar);
        args.map(putchar);
        showstr(&"\n");
    }
}

pub unsafe fn screen() {
    putstr(&"\n                                                               ");
    putstr(&"\n                                                               ");
    putstr(&"\n                       7=..~$=..:7                             ");
    putstr(&"\n                  +$: =$$$+$$$?$$$+ ,7?                        ");
    putstr(&"\n                  $$$$$$$$$$$$$$$$$$Z$$                        ");
    putstr(&"\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    ");
    putstr(&"\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 ");
    putstr(&"\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                ");
    putstr(&"\n           Z$$$$$?                    .+ZZZZ$$                 ");
    putstr(&"\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            ");
    putstr(&"\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              ");
    putstr(&"\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           ");
    putstr(&"\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           ");
    putstr(&"\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            ");
    putstr(&"\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          ");
    putstr(&"\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          ");
    putstr(&"\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           ");
    putstr(&"\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          ");
    putstr(&"\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         ");
    putstr(&"\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           ");
    putstr(&"\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          ");
    putstr(&"\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          ");
    putstr(&"\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            ");
    putstr(&"\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           ");
    putstr(&"\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           ");
    putstr(&"\n        IZZZZZ:.                        . ,ZZZZZ$              ");
    putstr(&"\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             ");
    putstr(&"\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 ");
    putstr(&"\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                ");
    putstr(&"\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                ");
    putstr(&"\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    ");
    putstr(&"\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    ");
    putstr(&"\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        ");
    putstr(&"\n                       $7..I$7..I$,                            ");
    putstr(&"\n");
    putstr(&"\n _                     _     _                         _  ");
    putstr(&"\n| |                   (_)   | |                       | | ");
    putstr(&"\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | ");
    putstr(&"\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | ");
    putstr(&"\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | ");
    putstr(&"\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\n\n");
}

pub unsafe fn init() {
    buffer = cstr::new();
    history = cstr::news(512);
    screen();
    prompt();
}

pub unsafe fn prompt() {
    putstr(PROMPT);
    let prev_c = super::super::io::FG_COLOR;
    super::super::io::set_fg(PROMPT_COLOR);
    drawstr(PROMPT);
    super::super::io::set_fg(prev_c);
}

