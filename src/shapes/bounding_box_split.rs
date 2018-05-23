use shapes::bounding_box::BoundingBox;
use Result;

pub trait BoundingBoxSplit: BoundingBox {
    fn split(&self) -> Vec<Self>;

    fn get_bounds(&self, &Self::T) -> Result<usize>;
}