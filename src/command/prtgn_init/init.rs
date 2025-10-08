// code for the 'new' sub-command
use crate::editor;

pub fn new_file(filename: String) {
    println!("Creating a new file...");

    editor::editor(filename);
}