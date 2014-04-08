use core::option::{Some, Option, None};
use kernel::cstr::cstr;

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
        nfs.current = nfs.head.child;
        nfs
    }

    pub unsafe fn link_dir(sibling: *inode) -> Option<*inode> {
        
    }
}

pub struct inode {
    name: Option<cstr>,
    parent: Option<*inode>,
    child: Option<*inode>,
    left: Option<*inode>,
    right: Option<*inode>,
    data: Option<cstr>,
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

    unsafe fn new_dir(name: cstr, link: *inode) -> inode {
        inode {
            parent: None,
            child: Some(~inode::new_dhead()),
            left: Some(link),
            right: None,
            name: None,
            data: None,
        }
    }

    pub fn is_dir(&self) {
        self.child.is_some()
    }

    pub fn is_dhead(&self) {
        self.left.is_none()
    }
    
    pub fn is_file(&self) {
        // Nothing will be files for 1st prototype
        self.data.is_some()
    }

    pub fn unlink() {
        // Does nothing yet
    }
}

static mut curr_dir: cstr = cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};

pub fn ls(dirname: cstr) {
}

pub fn cat(filename: cstr) {
}

pub fn cd(newdir: cstr) {
}

pub fn rm(filename: cstr) {
}

pub fn mkdir(dirname: cstr) {
}

pub fn pwd() {
}

pub fn mv(from: cstr, to: cstr) {
}

pub fn wr(filename: cstr, string: cstr) {
}

pub unsafe fn init() {
    filesystem = fs::new();
}
