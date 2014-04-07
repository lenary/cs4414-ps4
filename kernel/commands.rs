use super::cstr;
use super::sgash;

static mut curr_dir: cstr::cstr = cstr::cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};

pub fn ls(dirname: cstr::cstr) {
}

pub fn cat(filename: cstr::cstr) {
}

pub fn cd(newdir: cstr::cstr) {
}

pub fn rm(filename: cstr::cstr) {
}

pub fn mkdir(dirname: cstr::cstr) {
}

pub fn pwd() {
}

pub fn mv(from: cstr::cstr, to: cstr::cstr) {
}

pub fn wr(filename: cstr::cstr, string: cstr::cstr) {
}