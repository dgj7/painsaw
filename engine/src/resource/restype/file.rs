use crate::resource::Resource;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

///
/// a resource that comes from a file.
///
pub struct FileResource {
    path: PathBuf,
}

impl FileResource {
    ///
    /// initialize from str.
    ///
    pub fn from_path_str(path: &str) -> FileResource {
        Self::from_path(PathBuf::from(path))
    }

    ///
    /// initialize from path buf.
    ///
    pub fn from_path(path: PathBuf) -> FileResource {
        FileResource {
            path,
        }
    }
}

impl Resource for FileResource {
    ///
    /// read a file, to eof, with automagic retries.
    ///
    fn bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(&self.path)?)
    }
}
