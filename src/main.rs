use serv::tempfile::*;
use std::env::temp_dir;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, env)]
    editor: Box<str>,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    TestEditor
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
           Some(Commands::TestEditor) => editor_test(&cli.editor), 
           None => println!("Successfully completed doing nothing!")
    }
     
}

fn editor_test(editor: &str) {
    let temp_file = TempFile::new(Some("test"), temp_dir()).unwrap();
    open_in_editor(editor, &temp_file.get_path());
    println!("Output of editor buffer was: {}", temp_file.read_to_str().unwrap());
    let _ = temp_file.destroy();
}
