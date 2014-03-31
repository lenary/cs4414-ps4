/* kernel::cstr.rs */
#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;

pub static DEFAULT_STRLEN: uint = 256;

pub static mut count:uint = 0;

pub struct cstr {
    p: *mut u8,
    p_cstr_i: uint,
    size: uint
}

impl cstr {
    pub fn new() -> cstr {
        unsafe {
            cstr::news(DEFAULT_STRLEN)
        }
    }

    pub unsafe fn news(size: uint) -> cstr {
        // Sometimes this doesn't allocate enough memory and gets stuck...
        let (x, y) = heap.alloc(size);
        let this = cstr {
            p: x,
            p_cstr_i: 0,
            size: y
        };
        *(((this.p as uint)) as *mut char) = '\0';
        this
    }

    pub fn from_str(s: &str) -> cstr {
        let mut this = cstr::new();
        for c in slice::iter(as_bytes(s)) {
            this.add_u8(*c);
        };
        this
    }

    pub fn map_u8(&mut self, f: |u8|) {
        self.map(|c: char| {
            f(c as u8)
        });
    }
    
    pub fn map(&self, f: |char|) {
        let mut i = 0;
        while i < self.len() && self.get_char(i) != '\0' {
            f(self.get_char(i));
            i += 1;
        }
    }

    pub unsafe fn join(&self, other: cstr) -> cstr {
        let mut len = DEFAULT_STRLEN;
        while len < self.len() + other.len() {
            len += DEFAULT_STRLEN;
        }
        let mut new = cstr::news(len);
        self.map(|c| { new.add_char(c); } );
        other.map(|c| { new.add_char(c); } );
        new
    }

    pub unsafe fn trim(&mut self) -> cstr {
        let mut new = cstr::new();
        if self.len() == 0 || self.size > new.size {
            return new;
        }
        // Kill beginning whitespace.
        let mut beginwhite = true;
        let mut i = 0;
        while i < self.len() {
            let c = self.get_char(i);
            let iswhite = (c == (' ') || c == ('\n'));
            if !iswhite || !beginwhite {
                beginwhite = false;
                new.add_char(c);
            }
            i += 1;
        }
        // Kill trailing whitespace.
        i = self.len();
        let mut endwhite = true;
        while (endwhite && i > 0) {
            let c = self.get_char(i);
            let iswhite = (c == (' ') || c == ('\n'));
            if !iswhite {
                endwhite = false;
            }
            else {
                new.delete_char();
            }
            i -= 1;
        }
        return new;
    }

    pub unsafe fn clone(&mut self) -> cstr {
        let mut ind: uint = 0;
        let mut s = cstr::news(self.size);
        loop {
            if (self.len() == 0 || ind == self.len()) {
                return s;
            }
            s.add_u8(self.get_char(ind) as u8);
            ind += 1;
            if (ind == self.size) {
                return s;
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn len(&self) -> uint {
        self.p_cstr_i
    }

    // HELP THIS DOESN'T WORK THERE IS NO GARBAGE COLLECTION!!!
    // -- TODO: exchange_malloc, exchange_free
    #[allow(dead_code)]
    pub unsafe fn destroy(&self) {
        heap.free(self.p);
    }
    
    pub fn get_char(&self, idx: uint) -> char {
        if idx > self.len() || idx > self.size || self.p_cstr_i > self.size {
            return '\0';
        }
        unsafe {
            *(((self.p as uint) + idx) as *char)
        }
    }

    pub fn add_char(&mut self, c: char) -> bool {
        self.add_u8(c as u8)
    }

    pub fn add_u8(&mut self, x: u8) -> bool {
        if (self.p_cstr_i >= self.size || self.size > DEFAULT_STRLEN) {
            return false;
        }
        unsafe {
            *(((self.p as uint) + self.p_cstr_i) as *mut u8) = x;
            self.p_cstr_i += 1;
            *(((self.p as uint) + self.p_cstr_i) as *mut char) = '\0';
        }
        true
    }

    pub unsafe fn delete_char(&mut self) -> bool {
        if (self.p_cstr_i == 0) {
            return false;
        }
        self.p_cstr_i -= 1;
        *(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
        true
    }

    pub unsafe fn reset(&mut self) {
        self.p_cstr_i = 0;
        *(self.p as *mut char) = '\0';
    }

    pub unsafe fn eq(&self, other: &cstr) -> bool {
        // lengths not equal -> false
        if (self.len() != other.len()) {
            return false;
        }
        // both lengths are zero -> true
        if self.len() == 0 && other.len() == 0 {
            return true;
        }
        let mut i = 0;
        while i < self.len() {
            if (self.get_char(i) as u8) != (other.get_char(i) as u8) {
                return false;
            }
            i += 1;
        }
        true
    }

    pub unsafe fn streq(&self, other: &str) -> bool {
        let mut i = 0;
        for c in slice::iter(as_bytes(other)) {
            let selfc = self.get_char(i) as u8;
            if( *c != selfc) {
                return false;
            }
            i += 1;
        };
        return true;
    }

    pub unsafe fn split(&self, delim: char) -> (cstr, cstr) {
        let mut i = 0;
        let mut beg = cstr::new();
        let mut end = cstr::new();
        let mut found = false;
        while i < self.len() && self.get_char(i) != '\0' {
            if (!found && self.get_char(i) as u8 == delim as u8) {
                found = true;
            }
            else if (!found) {
                beg.add_char(self.get_char(i));
            }
            else {
                end.add_char(self.get_char(i));
            };
            i += 1;
        }
        (beg, end)
    }
}
