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
    parent: Option<Node>,
    child: Option<Node>,
    next: Option<Node>,
    prev: Option<Node>,
    data: cstr,
    name: cstr
}

impl Node {  

    fn is_file(&self: Node) {
        data: 
        name: 
        next: Some<Node>,
        prev: Some<Node>,
        parent: None,
        child: None
    }

    fn is_dir(&self: Node) {
        data:
        name: 
        next: Some<Node>,
        prev: Some<Node>,
        parent: None,
        child: Some<Node>

    }

    fn is_dummy(&self: Node) {
        data: EMPTY,
        name: EMPTY,
        next: Some<Node>,
        prev: None,
        parent: Some<Node>,
        child: None
    }

    fn new_dummy() -> Node {
        let mut newDummy = Node {
            data: EMPTY, 
            name: EMPTY, 
            next: Some<Node>,
            prev: Some<Node>,
            parent: Some<Node>,
            child: None
        }
        newDummy;
    }
// impl File {
    fn new_file(name: cstr, data: cstr) -> Node {
        let mut newFile = File {
            filename: name, 
            value: content, 
            next: Some<~File>, 
            prev: Some<File>
        }
        newFile;
    }

    fn new_dir() -> Node {
            let mut newDir = Dir {
            dir_name: name,
            parent: None,
            child: Some<Node> // some dummy
        }
        newDir;
    }
}


impl Dir {
    
    fn new(name: cstr) -> Dir {
        let mut newDir = Dir {
        dir_name: name,
        parent: Option<~Node>, //could be null
        child: Some<Node> //
        }
    }

    fn is_empty(&self: Node) -> bool {
        match (ref self.child) {
            Some => return true;
            _ => return false;
        }


    }

}

// pub struct List<~Node> {


// }

// impl List<~Node> { 
//     fn new() -> List<~Node> { 
//         List { head: new_dummy() }
//     }

//     fn add(&mut self, item: T)
//     {
//         // let tail = self;
//         // loop
//         // {
//         //     match *tail
//         //     {
//         //         Node(_, ~ref next) => tail = next,
//         //         Nil => break
//         //     }
//         // }
//         // *tail = Node(item, ~Nil);
//     }

//     pub unsafe fn is_file(&self) -> bool {
//         match (self) {
//             Some => {},
//             None => {}
//         }
//     }

//     pub unsafe fn is_dir(&self) -> bool {
//         match (self) {
//             Some => {},
//             None => {}
//         }
//     }

//     //writes the given string to the given file
//     pub unsafe fn write_file(file: File, string: cstr) {
//         match (file) {
//             Some (ref file) => { file.value = string },
//             None => { }
//         }   
//     }

//     //returns the string stored in a given file
//     pub unsafe fn read_file(&mut self) -> cstr {
//         match(self) {
//             // Some(ref mut current) => {
//             //     current.value: },
//             // None => { }

//             // use putstr / drawstr            
//         }
//     }

//     //creates a file with the given name in the given directory
//     pub unsafe fn create_file(thisDir: Dir, thisName: cstr) {
//         // File::new()
//     }

//     //deletes the file with the given name in the given directory
//     pub unsafe fn delete_file(thisDir: Dir, thisName: cstr) {

//     }

//     //gets the file with the given name belonging to the specified directory
//     pub unsafe fn get_file(thisDir: Dir, thisName: cstr) -> File {
       

//     }


//     //returns the list of files and directories contained in the specified directory
//     pub unsafe fn list_directory(directory) {
//         //for loop through contents of directory (files, directories)
//         for (file : directory) {
//             putstr(file.value);
//             drawstr(file.value);
//         }

//         for (dir : directory) {
//             putstr(dir.dir_name);
//             drawstr(dir.dir_name);
//         }

//     }
    
//     // creates a directory with the specified name under the given parent directory
//     pub unsafe fn create_directory(parent, name) {

//     }

//     //deletes the given directory if and only if it is empty
//     pub unsafe fn delete_directory(directory) {

//         if (dir.is_empty() == true) {
//             //then remove
//         }

//     }

//     //gets the directory with the given name belonging to the specified parent
//     pub unsafe fn get_directory(parent, name) {

        
//     }


}



