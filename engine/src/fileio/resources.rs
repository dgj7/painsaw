//!
//! access to resources that are external to the engine.
//!

use std::error::Error;

pub mod file;
pub mod memory;

///
/// any resource that exists outside the engine.
///
pub trait Resource {
    ///
    /// load all the bytes from the resource.
    ///
    fn bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>;
}
