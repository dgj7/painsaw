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
}
