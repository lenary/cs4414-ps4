/* kernel::cstr.rs */
#![allow(unused_imports)]
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;

pub static DEFAULT_STRLEN: uint = 256;
pub static mut first: bool = true;

pub struct Cstr {
    p: *mut u8,
    p_cstr_i: uint,
    size: uint
}

impl Cstr {
    pub fn new() -> Cstr {
        unsafe {
            Cstr::news(DEFAULT_STRLEN)
        }
    }

    pub unsafe fn news(size: uint) -> Cstr {
        // Sometimes this doesn't allocate enough memory and gets stuck...
        if first { let (_, _) = heap.alloc(size); }
        first = false;
        let (x, y) = heap.alloc(size);
        let this = Cstr {
            p: x,
            p_cstr_i: 0,
            size: y
        };
        *(((this.p as uint)) as *mut char) = '\0';
        this
    }

    pub fn from_str(s: &str) -> Cstr {
        let mut this = Cstr::new();
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

    pub unsafe fn join(&self, other: Cstr) -> Cstr {
        let mut len = DEFAULT_STRLEN;
        while len < self.len() + other.len() {
            len += DEFAULT_STRLEN;
        }
        let mut new = Cstr::news(len);
        self.map(|c| { new.add_char(c); } );
        other.map(|c| { new.add_char(c); } );
        new
    }

    pub unsafe fn trim(&mut self) -> Cstr {
        let mut new = Cstr::new();
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

    pub unsafe fn clone(&mut self) -> Cstr {
        let mut ind: uint = 0;
        let mut s = Cstr::news(self.size);
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
    
    pub fn len(&self) -> uint {
        self.p_cstr_i
    }

    pub unsafe fn destroy(&self) {
        heap.free(self.p);
    }
    
    pub fn get_p(&self, idx: uint) -> Option<uint> {
        if idx > self.len() || idx > self.size || self.p_cstr_i > self.size {
            return None;
        }
        Some((self.p as uint) + idx)
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
        if (self.p_cstr_i + 1 >= self.size) {
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

    pub unsafe fn eq(&self, other: &Cstr) -> bool {
        if (self.len() != other.len()) { return false; }
        else {
            let mut x = 0;
            let mut selfp: uint = self.p as uint;
            let mut otherp: uint = other.p as uint;
            while x < self.len() {
                if (*(selfp as *char) != *(otherp as *char)) { return false; }
                selfp += 1;
                otherp += 1;
                x += 1;
            }
            true
        }
    }

    #[allow(dead_code)]
    pub unsafe fn split(&self, delim: char) -> (Cstr, Cstr) {
        let mut i = 0;
        let mut beg = Cstr::news(self.size);
        let mut end = Cstr::news(self.size);
        self.map(|c| {beg.add_char(c);});
        let mut found = false;
        while i < self.len() && !found {
            if !found && beg.get_char(i) as u8 == delim as u8 {
                found = true;
                match beg.get_p(i) {
                    Some(p) => *(p as *mut char) = '\0',
                    None => {},
                }
                beg.p_cstr_i = i;
            }
            i += 1;
        }
        while i < self.len() {
            end.add_char(self.get_char(i));
            i += 1;
        }
        (beg, end)
    }
}
