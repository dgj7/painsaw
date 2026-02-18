use crate::support::resource::Resource;
use std::error::Error;

///
/// a resource that comes from memory.
///
pub struct MemoryResource {
    bytes: Vec<u8>,
}

impl MemoryResource {
    pub fn from_array(bytes: Vec<u8>) -> MemoryResource {
        MemoryResource { bytes }
    }
}

impl Resource for MemoryResource {
    ///
    /// get all bytes from memory.
    ///
    fn bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // todo: clone here?
        Ok(self.bytes.clone())
    }
}
