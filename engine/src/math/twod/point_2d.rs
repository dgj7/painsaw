use num_traits::Num;

pub struct Point2D<F: Num> {
    pub x: F,
    pub y: F,
}

impl<F: Num> Point2D<F> {
    pub fn new(x: F, y: F) -> Point2D<F> {
        Point2D { x, y }
    }

    pub fn origin() -> Point2D<F> {
        Point2D { x: F::zero(), y: F::zero() }
    }
}
