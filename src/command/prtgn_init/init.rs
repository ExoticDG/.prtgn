
// code for the 'new' sub-command
pub mod text_editor;
pub mod wav_converter;

pub fn init(filename: String, wav: bool) {
    println!("Creating a new file...");
    println!("Wav flag is: {}", wav);

    let mut filename_prt = filename;

    if wav == true {
        if !filename_prt.ends_with(".wav") {
       filename_prt.push_str(".wav");
      }
      wav_converter::wav_to_prtgn(filename_prt);

    }
    else {
        if !filename_prt.ends_with(".prtgn") {
       filename_prt.push_str(".prtgn");
        text_editor::editor(filename_prt);
      }
    }


}