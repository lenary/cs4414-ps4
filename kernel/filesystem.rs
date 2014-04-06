//filesystem.rs

#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;


pub struct File { //leaf node
	val: cstr,
	filename: cstr,
	next: Option<~File>
}

pub struct Directory { //branch of tree
    dir_name: cstr,
}

type LinkedList = Option<~File>;


pub fn construct_list(n: int, x: cstr) -> LinkedList {
    match n {
        0 => { None }
        _ => { Some(~Node{val: x, tail: construct_list(n - 1, x + 1)}) }
    }
}


impl<T> LinkedList<Node>
{
    fn new(vector: &[T]) -> List<T> { Nil }

    fn add(&mut self, item: T)
    {
        let tail = self;
        loop
        {
            match *tail
            {
                Node(_, ~ref next) => tail = next,
                Nil => break
            }
        }
        *tail = Node(item, ~Nil);
    }
}

impl file {

		pub fn new() -> file {
        unsafe {
            file::new_file

            let (z, w) = heap.alloc(size);

        }
    }
        //writes the given string to the given file
        pub unsafe fn write_file(file: &self, cstr: s) {
            let new_file = file {val: cstr::news(DEFAULT_STRLEN)};

            

            }            
        }
        //returns the string stored in a given file
        pub unsafe fn read_file(file: &self) -> Option<cstr> {
            
            match f {
                None => {},
                Some => {}
            }

        }

        
        //creates a file with the given name in the given directory
        pub unsafe fn create_file(directory: d, filename: name) 

        //deletes the file with the given name in the given directory
        pub unsafe fn delete_file(directory, name) 
        
        //gets the file with the given name belonging to the specified directory
        pub unsafe fn get_file(directory, name) 


}

impl dir {
    pub unsafe fn list_directory(directory) //returns the list of files and directories contained in the specified directory
    pub unsafe fn create_directory(parent, name) // creates a directory with the specified name under the given parent directory
    pub unsafe fn delete_directory(directory) //deletes the given directory if and only if it is empty
    pub unsafe fn get_directory(parent, name) //gets the directory with the given name belonging to the specified parent

    pub unsafe fn is_file()
    pub unsafe fn is_dir()

}
