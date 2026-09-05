pub struct Demo1 {
    pub display_menus: bool,
}

impl Demo1 {
    pub(crate) fn new() -> Self {
        Self { display_menus: false }
    }
}
