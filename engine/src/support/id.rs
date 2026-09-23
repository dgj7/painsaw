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

impl IdentifierFactory {
    ///
    /// create a new id factory.
    ///
    pub fn new() -> IdentifierFactory {
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
