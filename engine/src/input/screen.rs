use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;
use crate::window::mswin::util::{get_client_rect_dim2d, get_window_rect_dim2d};
use crate::window::mswin::winapi::{get_client_rect, get_window_rect};

#[derive(Clone, Debug)]
pub struct ScreenState {
    /* dimensions */
    pub previous_client_dimensions: Dimension2D,
    pub current_client_dimensions: Dimension2D,
    pub previous_window_dimensions: Dimension2D,
    pub current_window_dimensions: Dimension2D,
    
    /* rects */
    pub previous_client_rect: Rectangle2D,
    pub current_client_rect: Rectangle2D,
    pub previous_window_rect: Rectangle2D,
    pub current_window_rect: Rectangle2D,
    
    /* locations */
    pub window_center: Vertex2D,
    pub client_center: Vertex2D,
}

impl ScreenState {
    pub fn new() -> Self {
        ScreenState {
            /* dimensions */
            previous_client_dimensions: Dimension2D::new(0.0, 0.0),
            current_client_dimensions: Dimension2D::new(0.0, 0.0),
            previous_window_dimensions: Dimension2D::new(0.0, 0.0),
            current_window_dimensions: Dimension2D::new(0.0, 0.0),
            
            /* rects */
            previous_client_rect: Rectangle2D { top_left: Vertex2D { x: 0.0, y: 0.0 }, bottom_right: Vertex2D { x: 0.0, y: 0.0 } },
            current_client_rect: Rectangle2D { top_left: Vertex2D { x: 0.0, y: 0.0 }, bottom_right: Vertex2D { x: 0.0, y: 0.0 } },
            previous_window_rect: Rectangle2D { top_left: Vertex2D { x: 0.0, y: 0.0 }, bottom_right: Vertex2D { x: 0.0, y: 0.0 } },
            current_window_rect: Rectangle2D { top_left: Vertex2D { x: 0.0, y: 0.0 }, bottom_right: Vertex2D { x: 0.0, y: 0.0 } },
            
            /* locations */
            window_center: Vertex2D::origin(),
            client_center: Vertex2D::origin(),
        }
    }

    #[cfg(target_os="windows")]
    pub fn from(hwnd: windows::Win32::Foundation::HWND) -> ScreenState {
        let mut screen = ScreenState::new();
        screen.update_mswin(hwnd);
        screen
    }

    // todo: this probably doesnt need to be called once per frame; more likely just once per update from the us (per win32 message)
    #[cfg(target_os="windows")]
    pub fn update_mswin(&mut self, hwnd: windows::Win32::Foundation::HWND) {
        /* get the various screen stats from win32 */
        let window_dimensions = get_window_rect_dim2d(hwnd);
        let window_rect = get_window_rect(hwnd);
        let client_dimensions = get_client_rect_dim2d(hwnd);
        let client_rect = get_client_rect(hwnd);

        /* make updates */
        self.update_client_dimensions(client_dimensions);
        self.update_window_dimensions(window_dimensions);
        self.update_client_rectangle(Rectangle2D::new(client_rect));
        self.update_window_rectangle(Rectangle2D::new(window_rect));
        self.update_screen_center();
    }

    fn update_client_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_client_dimensions.height = self.current_client_dimensions.height;
        self.previous_client_dimensions.width = self.current_client_dimensions.width;

        /* new info goes into current */
        self.current_client_dimensions.height = current.height;
        self.current_client_dimensions.width = current.width;
    }

    fn update_window_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_window_dimensions.height = self.current_window_dimensions.height;
        self.previous_window_dimensions.width = self.current_window_dimensions.width;

        /* new info goes into current */
        self.current_window_dimensions.height = current.height;
        self.current_window_dimensions.width = current.width;
    }

    fn update_client_rectangle(&mut self, current: Rectangle2D) {
        self.previous_client_rect.top_left.x = self.current_client_rect.top_left.x;
        self.previous_client_rect.top_left.y = self.current_client_rect.top_left.y;
        self.previous_client_rect.bottom_right.x = self.current_client_rect.bottom_right.x;
        self.previous_client_rect.bottom_right.y = self.current_client_rect.bottom_right.y;

        self.current_client_rect.top_left.x = current.top_left.x;
        self.current_client_rect.top_left.y = current.top_left.y;
        self.current_client_rect.bottom_right.x = current.bottom_right.x;
        self.current_client_rect.bottom_right.y = current.bottom_right.y;
    }

    fn update_window_rectangle(&mut self, current: Rectangle2D) {
        self.previous_window_rect.top_left.x = self.current_window_rect.top_left.x;
        self.previous_window_rect.top_left.y = self.current_window_rect.top_left.y;
        self.previous_window_rect.bottom_right.x = self.current_window_rect.bottom_right.x;
        self.previous_window_rect.bottom_right.y = self.current_window_rect.bottom_right.y;

        self.current_window_rect.top_left.x = current.top_left.x;
        self.current_window_rect.top_left.y = current.top_left.y;
        self.current_window_rect.bottom_right.x = current.bottom_right.x;
        self.current_window_rect.bottom_right.y = current.bottom_right.y;
    }

    fn update_screen_center(&mut self) {
        let wx = (self.current_window_rect.top_left.x + self.current_window_rect.bottom_right.x) / 2.0;
        let wy = (self.current_window_rect.top_left.y + self.current_window_rect.bottom_right.y) / 2.0;
        self.window_center.x = wx;
        self.window_center.y = wy;

        let cx = (self.current_client_rect.top_left.x + self.current_client_rect.bottom_right.x) / 2.0;
        let cy = (self.current_client_rect.top_left.y + self.current_client_rect.bottom_right.y) / 2.0;
        self.client_center.x = cx;
        self.client_center.y = cy;
    }
}
