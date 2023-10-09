use emit::tempfile::*;
use std::env::{var,temp_dir};

fn main() {
    let editor = var("EDITOR").unwrap();
    let temp_file = TempFile::new(None, temp_dir()).unwrap();
    open_in_editor(&editor, &temp_file.get_path());
    println!("{:#?}", temp_file.read_to_str());
    let _ = temp_file.destroy(); 
}
