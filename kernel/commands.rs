use core::option::{Some, Option, None};
use kernel::cstr::Cstr;
use kernel::sgash::drawstr;

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

    pub unsafe fn is_dir(&self) -> bool {
        self.child.is_some()
    }

    pub unsafe fn is_dhead(&self) -> bool {
        self.left.is_none()
    }
    
    pub unsafe fn is_file(&self) -> bool {
        // Nothing will be files for 1st prototype
        self.data.is_some()
    }

    pub unsafe fn unlink() {
        // Does nothing yet
    }
}

static mut curr_dir: Cstr = Cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    size: 0,
};
*/
pub unsafe fn ls(dirname: Cstr) {
    drawstr(&". rusty\n");
}

pub unsafe fn cat(filename: Cstr) {
    if filename.streq(&"rusty") {
        drawstr(&"kernel\n");
    }
}

pub unsafe fn cd(newdir: Cstr) {
}

pub unsafe fn rm(filename: Cstr) {
}

pub unsafe fn mkdir(dirname: Cstr) {
}

pub unsafe fn pwd() {
    drawstr(&"/\n");
}

pub unsafe fn mv(from: Cstr, to: Cstr) {
}

pub unsafe fn wr(filename: Cstr, string: Cstr) {
}
