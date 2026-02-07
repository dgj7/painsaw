///
/// build-in command types for
///
pub trait Command {
    ///
    /// describe the command.
    ///
    fn describe(&self) -> String;

    ///
    /// execute the command on the current context.
    ///
    // todo: use rendering context here
    fn execute(&self);

    ///
    /// undo the previous operation.
    ///
    // todo: use rendering context here
    fn undo(&self) {
        panic!("`undo` not supported!");
    }
}
