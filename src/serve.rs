use crate::pages::Page;
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct ServeManager {
    root_dir: PathBuf,
    pages_cache: Vec<Page>,
}

impl ServeManager {
    pub fn new(root_dir: String) -> Self {
        ServeManager {
            root_dir: PathBuf::from(root_dir),
            pages_cache: vec![],
        }
    }

    pub fn populate_from_root(&mut self) {
        let mut queue: VecDeque<PathBuf> = VecDeque::new();

        queue.push_back(PathBuf::from(&self.root_dir));

        while let Some(path) = queue.pop_front() {
            if path.is_dir() {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        queue.push_back(entry.path());
                    }
                }
            } else {
                if let Ok(page) = Page::new(path) {
                    self.pages_cache.push(page);
                } else {
                    println!("Warning: Could not load file into cache");
                }
            }
        }
    }
    pub fn serve(&mut self, resource: String) -> String {
        let wanted_path = resource.trim_start_matches("/");

        if let Some(page) = self
            .pages_cache
            .iter_mut()
            .find(|x| x.path.to_str() == Some(wanted_path))
        {
            return page.serve().unwrap_or_else(|_| "Error appended".into());
        }

        "No page found".into()
    }
}
