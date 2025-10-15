
// code for the 'new' sub-command
use crate::editor;

pub fn init(filename: String) {
    println!("Creating a new file...");

    let mut filename_prt = filename;

    if !filename_prt.ends_with(".prtgn") {
       filename_prt.push_str(".prtgn");
    }

    editor::editor(filename_prt);
}