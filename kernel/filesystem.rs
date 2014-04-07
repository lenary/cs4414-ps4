// linked list of nodes (files or dirs)

// #[allow(unused_imports)];
// use core::*;
// use core::str::*;
// use core::option::{Some, Option, None};
use core::iter::Iterator;
// use kernel::*;
// use kernel::cstr::cstr;
// use super::super::platform::*;
// use kernel::memory::Allocator;

// pub struct Node {
//     parent: Option<Node>,
//     child: Option<Node>,
//     next: Option<Node>,
//     prev: Option<Node>,
//     data: cstr,
//     name: cstr
// }

enum Node {
    File,
    Dir,
    Dummy
}

pub struct File {
    data: cstr,
    name: cstr,
    next: Some<Node>,
    prev: Some<Node>,
    parent: None,
    child: None
}

pub struct Dir {
    data: cstr,
    name: cstr,
    next: Some<Node>,
    prev: Some<Node>,
    parent: None,
    child: Some<Node>
}

pub struct Dummy {
    data: cstr,
    name: cstr,
    next: Some<Node>,
    prev: None,
    parent: Some<Node>,
    child: None
}

impl Node {  

    fn is_file(&self: Node) -> bool {
        match (self) {
            data: 
            name: 
            next: Some<Node>,
            prev: Some<Node>,
            parent: None,
            child: None
        }
    }

    fn is_dir(&self: Node) -> bool {
        match (self.parent) {
            None =>
            // data: cstr,
            // name: cstr,
            // next: Some<Node>,
            // prev: Some<Node>,
            // parent: None,
            // child: Some<Node>
    }
    }

    fn is_dummy(&self: Node) -> bool {
        match (self.child) {
            None => return true;
            _ => return false;
        //     data: EMPTY,
        // name: EMPTY,
        // next: Some<Node>,
        // prev: None,
        // parent: Some<Node>,
        // child: None
        }
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

    fn new_dir(&self) -> Node {
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

pub struct List<Node> {

}

// enum List {
//     Cons (...), //next item in list
//     Nil //end of list
// }


impl List<Node> { 
    fn new() -> List<Node> { 
        let mut list = List { 
            //create new list with a dummy node
            Node::new_dummy() 
            }
    }
    
    fn insert(pointer: Node, data: cstr) {
        while (pointer.next != None) {
            pointer = pointer.next;
        }
        pointer = pointer.next;
        pointer.data = data;
        pointer.next = NULL;
    }

    fn find_str (pointer: Node, key: cstr) -> bool {
        pointer = pointer.next; //dummy node is first

        while (pointer != None) {
            if (pointer.data == key) {
                return true;
            }
            pointer = pointer.next; //look in next node
        }
        return false;  
    }

    fn delete(pointer: Node, data: cstr) {
        while (pointer.next != None && (pointer.next).data != data) {
            pointer = pointer.next;
        }
        if (pointer.next == None) {

            //HANDLE IF NOT IN LIST
        }

        let mut temp = Node {next: pointer.next};
        pointer.next = temp.next;

        //


    }



    //writes the given string to the given file
    pub unsafe fn write_file(file: File, string: cstr) {
        
        let mut file = file.name;
        file.value = string;
          
    }

    //returns the string stored in a given file
    pub unsafe fn read_file(&mut self) -> cstr {
            putstr(self.data);
            drawstr(self.data);
    }

    //creates a file with the given name in the given directory
    pub unsafe fn create_file(thisDir: Dir, thisName: cstr) {
        let mut newFile = File {
            parent: thisDir,
            name: thisName,
            next: 
        }
    }

    //deletes the file with the given name in the given directory
    pub unsafe fn delete_file(thisDir: Dir, thisName: cstr) {

    }

    //gets the file with the given name belonging to the specified directory
    pub unsafe fn get_file(thisDir: Dir, thisName: cstr) -> File {
       if (thisDir.name = )

    }


    //returns the list of files and directories contained in the specified directory
    pub unsafe fn list_directory(directory: Dir, ) {
        //for loop through contents of directory (files, directories)
        for (file : directory) {
            putstr(file.name);
            drawstr(file.name);
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
        for (dir in  )
        if (dir.is_empty() == true) {
            //then remove
        }

    }

    //gets the directory with the given name belonging to the specified parent
    pub unsafe fn get_directory(parent, name) {

        
    }

}



