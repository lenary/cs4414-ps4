use core::option::{Some, Option, None};
use kernel::cstr::Cstr;

/*
pub static mut filesystem: Option<fs> = None;
pub struct fs {
    head: ~inode,
    current: Option<*inode>,
}

impl fs {
    pub unsafe fn new() -> fs { 
        let mut nfs = fs {
            head: ~inode::new(),
            current: None,
        };
        nfs
    }

    pub unsafe fn link_dir(sibling: *inode) -> Option<*inode> {
        
    }
}

pub struct inode {
    name: Option<*mut Cstr>,
    parent: Option<*mut inode>,
    child: Option<*mut inode>,
    left: Option<*mut inode>,
    right: Option<*mut inode>,
    data: Option<*mut Cstr>,
}

impl inode {
    pub unsafe fn new() -> inode {
        inode {
            parent: None,
            child: None,
            left: None,
            right: None,
            name: None,
            data: None,
        }
    }

    unsafe fn new_dhead() -> inode {
        inode {
            parent: None,
            child: None,
            left: None,
            right: None,
            name: None,
            data: None,
        }
    }

    unsafe fn new_dir(name: Cstr, link: *inode) -> inode {
        inode {
            parent: None,
            child: Some(inode::new_dhead() as *u32),
            left: Some(link),
            right: None,
            name: None,
            data: None,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.child.is_some()
    }

    pub fn is_dhead(&self) -> bool {
        self.left.is_none()
    }
    
    pub fn is_file(&self) -> bool {
        // Nothing will be files for 1st prototype
        self.data.is_some()
    }

    pub fn unlink() {
        // Does nothing yet
    }
}

static mut curr_dir: Cstr = Cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};
*/
pub fn ls(dirname: Cstr) {
}

pub fn cat(filename: Cstr) {
}

pub fn cd(newdir: Cstr) {
}

pub fn rm(filename: Cstr) {
}

pub fn mkdir(dirname: Cstr) {
}

pub fn pwd() {
}

pub fn mv(from: Cstr, to: Cstr) {
}

pub fn wr(filename: Cstr, string: Cstr) {
}
