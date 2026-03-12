use std::fs;
use std::io;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub struct Page {
    content: String,
    pub path: PathBuf,
    last_edit_date: SystemTime,
}

impl Page {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let last_edit_date: SystemTime = fs::metadata(&path)?.modified()?;
        let content = fs::read_to_string(&path)?;

        Ok(Self {
            content,
            path: PathBuf::from(path),
            last_edit_date,
        })
    }
    pub fn update_from_file(&mut self) -> io::Result<()> {
        let last_edit_date = fs::metadata(&self.path)?.modified()?;
        let content = fs::read_to_string(&self.path)?;

        self.content = content;
        self.last_edit_date = last_edit_date;

        Ok(())
    }

    pub fn serve(&mut self) -> io::Result<String> {
        if self.is_page_obsolete() {
            self.update_from_file()?;
        }
        Ok(self.content.clone())
    }

    pub fn is_page_obsolete(&self) -> bool {
        let modified = fs::metadata(&self.path).and_then(|t| t.modified());

        match modified {
            Ok(t) => t != self.last_edit_date,
            Err(_) => true,
        }
    }
}
