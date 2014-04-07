/* kernel::sgash.rs */
#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None};
use core::mem::volatile_store;
use core::iter::Iterator;
use kernel::*;
use kernel::cstr::cstr;
use super::super::platform::*;
use super::super::platform::cpu::uart;
use kernel::memory::Allocator;

use commands;

static PROMPT: &'static str = &"sgash> ";
static UNRECOGNIZED: &'static str = &"Err: Unrecognized command\n";
static PROMPT_COLOR: u32 = 0xFFAF00;
static mut count: uint = 0;

static mut buffer: cstr = cstr {
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
        volatile_store(uart::UART0_DR, key);
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
        8 | 127 =>  {
            putchar('');
            putchar(' ');
            putchar('');
            buffer.delete_char();
            backspace();
        }
        _ =>  {
            if io::CURSOR_X < io::SCREEN_WIDTH-io::CURSOR_WIDTH {
                putchar(x as char);
                drawchar(x as char);
                buffer.add_u8(x);
            }
        }
    }
}

pub unsafe fn from_keyboard(x: u8) {
    match kbd::parse_kmi_key(x) {
        Some(y) => parsekey(y as char),
        None    => ()
    }
}

pub unsafe fn parse_buffer() {
    showstr(&"\n");
    buffer.map(putchar);
    buffer.map(drawchar);
    showstr(&"\n");
    let (command, args) = buffer.split(' ');
    if command.streq(&"echo") {
        args.map(drawchar);
        args.map(putchar);
        showstr(&"\n");
    }
    else if command.streq(&"cat") {
        commands::cat(args);
    }
    else if command.streq(&"cd") { 
        commands::cd(args);
    }
    else if command.streq(&"rm") { 
        commands::rm(args);
    }
    else if command.streq(&"ls") { 
        commands::ls(args);
    }
    else if command.streq(&"mkdir") {
        commands::mkdir(args);
    }
    else if command.streq(&"pwd") { 
        commands::pwd();
    }
    else if command.streq(&"wr") {
        let (filename, string) = args.split(' ');
        commands::wr(filename, string);
    }
    else if command.streq(&"mv") {
        let (from, to_) = args.split(' ');
        commands::mv(from, to_);
    }
    else {
        showstr(UNRECOGNIZED);
    }
    command.destroy();
    args.destroy();
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

