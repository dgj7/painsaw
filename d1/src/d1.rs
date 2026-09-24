#[derive(Clone)]
pub struct Demo1 {
    pub exit: bool,
    pub showing_main_menu: bool,
}

impl Demo1 {
    pub(crate) fn new() -> Self {
        Self {
            exit: false,
            showing_main_menu: false,
        }
    }
}
