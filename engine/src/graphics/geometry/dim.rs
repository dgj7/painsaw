use num_traits::Num;

#[derive(Clone,Debug)]
pub struct Dimension2D<F: Num> {
    pub height: F,
    pub width: F,
}

impl<F: Num> Dimension2D<F> {
    pub fn new(height: F, width: F) -> Dimension2D<F> {
        Dimension2D { height, width }
    }

    pub fn is_equal(&self, other: &Dimension2D<F>) -> bool {
        self.height == other.height
            && self.width == other.width
    }

    pub fn is_zero(&self) -> bool {
        self.width == F::zero() && self.height == F::zero()
    }
}
