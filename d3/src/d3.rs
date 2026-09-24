pub struct Demo3 {
    pub exit: bool
}

impl Demo3 {
    pub(crate) fn new() -> Self {
        Self {
            exit: false
        }
    }
}
