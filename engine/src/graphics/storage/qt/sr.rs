use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::qt::attrib::align::Alignment;
use crate::graphics::storage::qt::attrib::sizing::Sizing;

///
/// user request for how a 2d screen element should be sized and positioned.
///
pub struct SizingRequest {
    pub vertical_sizing: Sizing,
    pub horizontal_sizing: Sizing,

    pub vertical_alignment: Alignment,
    pub horizontal_alignment: Alignment,
}

impl SizingRequest {
    pub(super) fn client_to_origin(&self, client: &Dimension2D) -> Vertex2D {
        let width = self.horizontal_sizing.from_client_to_dimension(client.width);
        let height = self.vertical_sizing.from_client_to_dimension(client.height);
        let x = match self.horizontal_alignment {
            Alignment::Minimum => 0.0,
            Alignment::Center => (client.width / 2.0) - (width / 2.0),
            Alignment::Maximum => client.width - width,
        };
        let y = match self.vertical_alignment {
            Alignment::Minimum => 0.0,
            Alignment::Center => (client.height / 2.0) - (height / 2.0),
            Alignment::Maximum => client.height - height,
        };
        Vertex2D { x, y }
    }

    pub(super) fn client_to_antipode(&self, client: &Dimension2D) -> Vertex2D {
        let width = self.horizontal_sizing.from_client_to_dimension(client.width);
        let height = self.vertical_sizing.from_client_to_dimension(client.height);
        let x = match self.horizontal_alignment {
            Alignment::Minimum => width,
            Alignment::Center => (client.width / 2.0) + (width / 2.0),
            Alignment::Maximum => client.width,
        };
        let y = match self.vertical_alignment {
            Alignment::Minimum => client.height - height,
            Alignment::Center => (client.height / 2.0) + (height / 2.0),
            Alignment::Maximum => client.height,
        };
        Vertex2D { x, y }
    }
}

impl Default for SizingRequest {
    fn default() -> Self {
        SizingRequest {
            vertical_sizing: Sizing::RemainingSpace {},
            horizontal_sizing: Sizing::RemainingSpace {},
            vertical_alignment: Alignment::Center,
            horizontal_alignment: Alignment::Center,
        }
    }
}

pub struct SizingRequestBuilder {
    the_vertical_sizing: Option<Sizing>,
    the_horizontal_sizing: Option<Sizing>,
    the_vertical_alignment: Option<Alignment>,
    the_horizontal_alignment: Option<Alignment>,
}

impl SizingRequestBuilder {
    pub fn new() -> SizingRequestBuilder {
        SizingRequestBuilder {
            the_vertical_sizing: None,
            the_horizontal_sizing: None,
            the_vertical_alignment: None,
            the_horizontal_alignment: None,
        }
    }

    pub fn with_vertical_sizing(mut self, vertical: Sizing) -> SizingRequestBuilder {
        self.the_vertical_sizing = Some(vertical);
        self
    }

    pub fn with_horizontal_sizing(mut self, horizontal: Sizing) -> SizingRequestBuilder {
        self.the_horizontal_sizing = Some(horizontal);
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: Alignment) -> SizingRequestBuilder {
        self.the_vertical_alignment = Some(alignment);
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: Alignment) -> SizingRequestBuilder {
        self.the_horizontal_alignment = Some(alignment);
        self
    }
    
    pub fn build(self) -> SizingRequest {
        SizingRequest {
            vertical_sizing: self.the_vertical_sizing.unwrap_or_else(|| Sizing::RemainingSpace {}),
            horizontal_sizing: self.the_horizontal_sizing.unwrap_or_else(|| Sizing::RemainingSpace {}),
            horizontal_alignment: self.the_horizontal_alignment.unwrap_or_else(|| Alignment::Center),
            vertical_alignment: self.the_vertical_alignment.unwrap_or_else(|| Alignment::Center),
        }
    }
}
