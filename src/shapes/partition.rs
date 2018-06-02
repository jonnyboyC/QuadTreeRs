use shapes::bounding_box::BoundingBox;

pub trait Partition: BoundingBox {
    fn split(&self) -> Vec<Self>;

    fn get_partition(&self, &Self::T) -> Option<usize>;
}