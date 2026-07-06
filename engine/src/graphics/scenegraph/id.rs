use std::sync::atomic::AtomicUsize;

static NEXT : AtomicUsize = AtomicUsize::new(0);

///
/// A node identifier.
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(usize);

impl NodeId {
    ///
    /// generate a new node id, with a unique internal value.
    ///
    pub fn new() -> NodeId {
        NodeId { 0: NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) }
    }
}
