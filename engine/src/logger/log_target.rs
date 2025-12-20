#[derive(Debug, Clone)]
pub enum LogTarget {
    File { path: String },
    EngineConsole,
    StdOut,
}

impl LogTarget {
    pub(crate) fn print(&self, message: &String) {
        match self {
            LogTarget::File { path: _ }  => panic!("not implemented!"),
            LogTarget::EngineConsole => panic!("not implemented!"),
            LogTarget::StdOut => println!("{}", message),
        }
    }
}
