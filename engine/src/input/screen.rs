use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;
use crate::input::keyboard::kc::KeyChange;
use crate::input::keyboard::kii::KeyInputInfo;
use crate::input::keyboard::ks::KeyState;

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
    
    /* indicators */
    pub screen_resized: bool,
    pub focus: KeyState,
    
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
            
            /* indicators */
            screen_resized: false,
            focus: KeyState::new(KeyChange::Active {
                info: KeyInputInfo::handled(),
            }),
            
            /* locations */
            window_center: Vertex2D::origin(),
            client_center: Vertex2D::origin(),
        }
    }

    pub fn update_client_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_client_dimensions.height = self.current_client_dimensions.height;
        self.previous_client_dimensions.width = self.current_client_dimensions.width;

        /* new info goes into current */
        self.current_client_dimensions.height = current.height;
        self.current_client_dimensions.width = current.width;
    }

    pub fn update_window_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_window_dimensions.height = self.current_window_dimensions.height;
        self.previous_window_dimensions.width = self.current_window_dimensions.width;

        /* new info goes into current */
        self.current_window_dimensions.height = current.height;
        self.current_window_dimensions.width = current.width;
    }

    pub fn update_client_rectangle(&mut self, current: Rectangle2D) {
        self.previous_client_rect.top_left.x = self.current_client_rect.top_left.x;
        self.previous_client_rect.top_left.y = self.current_client_rect.top_left.y;
        self.previous_client_rect.bottom_right.x = self.current_client_rect.bottom_right.x;
        self.previous_client_rect.bottom_right.y = self.current_client_rect.bottom_right.y;

        self.current_client_rect.top_left.x = current.top_left.x;
        self.current_client_rect.top_left.y = current.top_left.y;
        self.current_client_rect.bottom_right.x = current.bottom_right.x;
        self.current_client_rect.bottom_right.y = current.bottom_right.y;
    }

    pub fn update_window_rectangle(&mut self, current: Rectangle2D) {
        self.previous_window_rect.top_left.x = self.current_window_rect.top_left.x;
        self.previous_window_rect.top_left.y = self.current_window_rect.top_left.y;
        self.previous_window_rect.bottom_right.x = self.current_window_rect.bottom_right.x;
        self.previous_window_rect.bottom_right.y = self.current_window_rect.bottom_right.y;

        self.current_window_rect.top_left.x = current.top_left.x;
        self.current_window_rect.top_left.y = current.top_left.y;
        self.current_window_rect.bottom_right.x = current.bottom_right.x;
        self.current_window_rect.bottom_right.y = current.bottom_right.y;
    }

    pub fn update_screen_center(&mut self) {
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
