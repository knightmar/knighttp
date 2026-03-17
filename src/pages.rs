use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents a resource on disk
/// # Page struct :
/// - content: stores the text content of the file
/// - path : the path of the file on disk
/// - last_edit_date : time at which the file was last edited (for caching)
pub struct Resource {
    content: String,
    pub path: PathBuf,
    last_edit_date: SystemTime,
}

impl Resource {
    /// Creates a Page, using the path give, and retrieves the file metadata to populate the last_edit_date and content
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let last_edit_date: SystemTime = fs::metadata(&path)?.modified()?;
        let content = fs::read_to_string(&path)?;

        Ok(Self {
            content,
            path,
            last_edit_date,
        })
    }

    /// Updates the content and last_edit_date of this Resource
    pub fn update_from_file(&mut self) -> io::Result<()> {
        let last_edit_date = fs::metadata(&self.path)?.modified()?;
        let content = fs::read_to_string(&self.path)?;

        self.content = content;
        self.last_edit_date = last_edit_date;

        Ok(())
    }

    /// Serves the resource and updates it if it's obsolete before serving it
    pub fn serve(&mut self, verbose: bool) -> io::Result<String> {
        if self.is_page_obsolete() {
            if verbose {
                println!("Page is obsolete, reloading cache");
            }
            self.update_from_file()?;
        }
        Ok(self.content.clone())
    }

    /// Checks if the resource is obsolete, based on the last_time_modified
    pub fn is_page_obsolete(&self) -> bool {
        let modified = fs::metadata(&self.path).and_then(|t| t.modified());

        match modified {
            Ok(t) => t != self.last_edit_date,
            Err(_) => true,
        }
    }
}
