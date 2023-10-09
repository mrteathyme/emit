use std::fs::{File, remove_file};
use std::process::Command;
use std::io::{Read,Write};
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub fn open_in_editor(editor: &str, path: &str) {
    if let Err(err) =  Command::new(editor).arg(path).status() {
        panic!("Error when calling {}: {}", editor, err);
    }
}

#[derive(Debug)]
pub struct TempFile {
    filename: Rc<str>,
    root: Rc<str>,
    path: Rc<str>,
    file: File,
}

impl TempFile {
    pub fn new(name: Option<&str>, mut root_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let name: Rc<str> = match name {
            Some(name) => name.into(),
            None => {
                let random_string: Vec<u8> = thread_rng().sample_iter(&Alphanumeric)
                .take(20)
                .collect();
                String::from_utf8(random_string)?.into()
            } 
        };
        let root: Rc<str> = root_path.as_path().to_str().unwrap().into();
        root_path.push(name.to_string());
        let path: Rc<str> = root_path.as_path().to_str().unwrap().into();
        let file = File::create(root_path)?;
        Ok(Self {
            filename: name.into(),
            root,
            path,
            file,
        })
    }

    pub fn destroy(self) -> Result<(), Box<dyn Error>> {
        remove_file(self.path.to_string())?;
        Ok(())
    }

    pub fn read_to_str(&self) -> Result<Box<str>, Box<dyn Error>> {
        let mut file = File::open(&self.path.to_string())?;
        let mut data_buffer = String::new();
        file.read_to_string(&mut data_buffer)?;
        Ok(data_buffer.into())
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.file.write_all(buf)?;
        Ok(())
    }

    pub fn get_path(&self) -> Rc<str> {
        self.path.clone()
    }

    pub fn get_filename(&self) -> Rc<str> {
        self.filename.clone()
    }

    pub fn get_root(&self) -> Rc<str> {
        self.filename.clone()
    }
}
