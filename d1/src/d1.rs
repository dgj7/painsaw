pub struct Demo1 {
    pub showing_main_menu: bool,
}

impl Demo1 {
    pub(crate) fn new() -> Self {
        Self {
            showing_main_menu: false,
        }
    }
}
