use crate::errors::ReturnCodes;
use crate::errors::ReturnCodes::NotFound;
use crate::pages::Resource;
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// ServeManager is the struct managing all pages and serving them
/// # ServeManager :
/// - root_dir : Path of the based resource directory
/// - resource_cache : a Vec of Pages that were found in the resource directory
pub struct ServeManager {
    root_dir: PathBuf,
    resource_cache: Vec<Resource>,
}

impl ServeManager {
    /// Creates a ServeManager based on the root_dir
    pub fn new(root_dir: String) -> Self {
        ServeManager {
            root_dir: PathBuf::from(root_dir),
            resource_cache: vec![],
        }
    }

    /// Populate the pages_cache by performing a BFS on the root_dir
    /// It is called once on start, and at each time a page not found error is encountered
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
                if let Ok(page) = Resource::new(path) {
                    self.resource_cache.push(page);
                } else {
                    println!("Warning: Could not load file into cache");
                }
            }
        }
    }

    /// Serves the resource requested, and updates it if it's not found (maybe the resource was created since the start of the server)
    pub fn serve(&mut self, resource: String) -> String {
        println!("Serving: {resource}");
        let wanted_path = resource.trim_start_matches("/");
        let mut content = String::from("No page found");
        let mut return_code = ReturnCodes::NotFound;

        for attempts in 0..2 {
            if let Some(page) = self
                .resource_cache
                .iter_mut()
                .find(|x| x.path.strip_prefix(&self.root_dir).ok() == Some(wanted_path.as_ref()))
            {
                println!("Page found, serving");
                if let Some(page_content) = page.serve().ok() {
                    content = page_content;
                    return_code = ReturnCodes::Success;
                } else {
                    content = "Error happened".into();
                    return_code = ReturnCodes::BackendError;
                };
            }

            if attempts == 0 {
                self.populate_from_root();
            }
        }

        let len = content.len();
        let content_type = Self::resolve_filetype(resource);
        let headers = format!("Content-Length: {len}\r\nContent-Type: {content_type}");
        let request = format!("HTTP/1.1 {return_code} OK\r\n{headers}\r\n\r\n{content}");

        request
    }

    /// Resolves the HTTP content type based on the extension of the on-disk file
    pub fn resolve_filetype(resource: String) -> String {
        match resource.as_str().split(".").last() {
            Some("html") => "text/html".into(),
            Some("css") => "text/css".into(),
            _ => "".into(),
        }
    }
}
