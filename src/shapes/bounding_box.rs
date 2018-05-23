pub trait BoundingBox: Sized {
    type T: BoundingBox + Clone;

    fn intersects(&self, &Self::T) -> bool;

    fn includes(&self, &Self::T) -> bool;
}