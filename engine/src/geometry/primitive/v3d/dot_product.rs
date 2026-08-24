use crate::geometry::primitive::v3d::Vertex3D;

// todo: finish/move documentation

impl Vertex3D {
    ///
    /// compute the dot product
    ///
    /// dot product: "how much" two vectors point in the same direction.
    /// positive: same direction
    /// zero: perpendicular
    /// negative: opposite directions
    ///
    /// computed as:
    /// absolute value of left
    /// x absolute value of right
    /// x cosine of the angle betwixt them
    ///
    pub fn dot_product(&mut self, multiplier: &Vertex3D) -> f32 {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }
    
    // todo: add new_dot_product() method that calls the above method
}


// todo: unit test
