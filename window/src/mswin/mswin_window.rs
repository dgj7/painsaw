use crate::model::window::Window;
use crate::model::window_request::WindowRequest;

pub struct MsWinWindow {
    // todo
}

impl Window for MsWinWindow {
    fn begin_display(&self) {
        println!("TODO: begin display of mswin-window");
    }
}

impl MsWinWindow {
    pub fn new(_request : &WindowRequest) -> Box<dyn Window> {
        Box::new(MsWinWindow {})
    }
}
