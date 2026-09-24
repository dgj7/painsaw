use std::sync::{LazyLock, Mutex};

static ID_FACTORY: LazyLock<Mutex<IdentifierFactory>> = LazyLock::new(|| {
    Mutex::new(IdentifierFactory::new())
});

///
/// a type suitable to be a key.  intended to identify a unique object.
///
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Identifier {
    value: u128,
}

///
/// factory for generating the next identifier.
///
/// the first id it produces will always be 1.
///
pub struct IdentifierFactory {
    index: u128,
}

///
/// access the public static id factory.
///
pub fn next_id() -> Identifier {
    let mut idf = ID_FACTORY.lock().unwrap();
    idf.index += 1;
    idf.next()
}

impl IdentifierFactory {
    ///
    /// create a new id factory.
    ///
    fn new() -> IdentifierFactory {
        IdentifierFactory { index: 0, }
    }

    ///
    /// get the next identifier.
    ///
    pub fn next(&mut self) -> Identifier {
        self.index += 1;
        Identifier { value: self.index }
    }
}
