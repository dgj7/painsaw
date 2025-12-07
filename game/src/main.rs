use window::create_window;
use window::model::window_request::{WindowDimensions, WindowRequest};

fn main() {
    let wr = WindowRequest::new(WindowDimensions::Fullscreen, "painsaw");
    match create_window(&wr) {
        Ok(win) => {
            win.begin_display();
        },
        Err(_e) => {
            // todo: logger here
        }
    }
}
