use shapes::bounding_box::BoundingBox;
use Result;

pub trait BoundingTreeNode: BoundingBox {
    fn retrieve(&self, &Self::T) -> Vec<Self::T>;

    fn insert(&mut self, Self::T) -> Result<()>;
}