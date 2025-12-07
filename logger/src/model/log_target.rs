
#[derive(Debug, Clone)]
pub enum LogTarget {
    File { path: String },
    EngineConsole,
    StdOut,
}

impl LogTarget {
    pub(crate) fn print<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        let message = message_provider();
        match self {
            LogTarget::File { path: _ }  => panic!("not implemented!"),
            LogTarget::EngineConsole => panic!("not implemented!"),
            LogTarget::StdOut => println!("{}", message),
        }
    }
}
