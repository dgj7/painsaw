use crate::window::context::RendererContext;

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
    fn execute(&self, context: &mut RendererContext);

    ///
    /// undo the previous operation.
    ///
    fn undo(&self, _context: &mut RendererContext) {
        panic!("`undo` not supported!");
    }
}
