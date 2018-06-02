use std::fmt::Display;

pub trait BoundingBox: Sized + Clone + Display {
    type T: BoundingBox;

    fn intersects(&self, &Self::T) -> bool;

    fn includes(&self, &Self::T) -> bool;
}