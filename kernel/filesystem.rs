// linked list of nodes (files or dirs)

// #[allow(unused_imports)];
// use core::*;
// use core::str::*;
// use core::option::{Some, Option, None};
// use core::iter::Iterator;
// use kernel::*;
// use kernel::cstr::cstr;
// use super::super::platform::*;
// use kernel::memory::Allocator;

pub struct Node {
    parent: Option<~Node>,
    child: Option<~Node>,
    data: cstr, 
    next: Option<*mut Node> //if it has a next, 
    //only has "right arrow", should check if is_file
}

impl Node {
    pub fn new_dummy() -> Node {
        Node {data: None, next: Option<~File>};
    }

    pub fn new(parent: ) {

    }
}

// pub struct File {
//     filename: cstr,
//     value: cstr,
//     next: Option<~file>,
//     prev: Option<~file>
//     //parent: Option<Dir>
// }

impl File {
    fn new(name: cstr, content: cstr) -> File {
        let mut newFile = File {
            filename: name, 
            value: content, 
            next: Some<~File>, 
            prev: Some<File>
        }
    }
}

pub struct Dir {
    dir_name: cstr,
    parent: Option<~Node>, //could be null
    child: Option<~Node> 
}

impl Dir {
    
    fn new(name: cstr) -> Dir {
        let mut newDir = Dir {
        dir_name: name,
        parent: Option<~Node>, //could be null
        child: Some<~Node> //
        }
    }

    fn is_empty(&self: Dir) -> bool {
        

    }

}

pub struct List<~Node> {

// fn add(file: F, &mut self) {
//     match *self {
//         Node(_, ref mut next) => next.add(item),
//         Nil => *self = Node(item, ~Nil)

// impl DirEntry for List {
//    fn length(&self) -> int {
//       match self {
//         &Some(ref node) => { 1 + node.tail.length() }
//         &None => 0
//       }
//    }
// }

}

impl List<~Node> { 
    fn new() -> List<~Node> { 
        List { head: new_dummy() }
    }

    fn add(&mut self, item: T)
    {
        // let tail = self;
        // loop
        // {
        //     match *tail
        //     {
        //         Node(_, ~ref next) => tail = next,
        //         Nil => break
        //     }
        // }
        // *tail = Node(item, ~Nil);
    }

    pub unsafe fn is_file(&self) -> bool {
        match (self) {
            Some => {},
            None => {}
        }
    }

    pub unsafe fn is_dir(&self) -> bool {
        match (self) {
            Some => {},
            None => {}
        }
    }

    //writes the given string to the given file
    pub unsafe fn write_file(file: File, string: cstr) {
        match (file) {
            Some (ref file) => { file.value = string },
            None => { }
        }   
    }

    //returns the string stored in a given file
    pub unsafe fn read_file(&mut self) -> cstr {
        match(self) {
            // Some(ref mut current) => {
            //     current.value: },
            // None => { }

            // use putstr / drawstr            
        }
    }

    //creates a file with the given name in the given directory
    pub unsafe fn create_file(thisDir: Dir, thisName: cstr) {
        // File::new()
    }

    //deletes the file with the given name in the given directory
    pub unsafe fn delete_file(thisDir: Dir, thisName: cstr) {

    }

    //gets the file with the given name belonging to the specified directory
    pub unsafe fn get_file(thisDir: Dir, thisName: cstr) -> File {
       

    }


    //returns the list of files and directories contained in the specified directory
    pub unsafe fn list_directory(directory) {
        //for loop through contents of directory (files, directories)
        for (file : directory) {
            putstr(file.value);
            drawstr(file.value);
        }

        for (dir : directory) {
            putstr(dir.dir_name);
            drawstr(dir.dir_name);
        }

    }
    
    // creates a directory with the specified name under the given parent directory
    pub unsafe fn create_directory(parent, name) {

    }

    //deletes the given directory if and only if it is empty
    pub unsafe fn delete_directory(directory) {

        if (dir.is_empty() == true) {
            //then remove
        }

    }

    //gets the directory with the given name belonging to the specified parent
    pub unsafe fn get_directory(parent, name) {

        
    }


}



