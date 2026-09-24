pub struct Demo2 {
    pub exit: bool,
}

impl Demo2 {
    pub(crate) fn new() -> Self {
        Self {
            exit: false,
        }
    }
}
