use std::fmt;
use std::fmt::{Display, Formatter};
use tree::quad_node::QuadNode;
use shapes::bounding_box::BoundingBox;
use shapes::partition::Partition;
use shapes::tree_node::TreeNode;
use { Result, QuadError };

#[derive(Clone)]
pub struct QuadTree<B>
    where B: Partition {
    root: Box<QuadNode<B>>
}

impl<B> QuadTree<B>
    where B: Partition {
    pub fn new(bounds: B) -> QuadTree<B> {
        QuadTree { 
            root: Box::new(
                QuadNode::new(0, bounds)
            ) 
        }
    }
        
    fn insert(&mut self, item: B::T) -> Result<()> {
        if !self.includes(&item) {
            return Err(QuadError::OutOfBounds)
        }

        self.root.insert(item)
    }

    fn retrieve(&self, item: &B::T) -> Vec<&B::T> {
        self.root.retrieve(item)
    }
}

impl<B> BoundingBox for QuadTree<B> 
    where B: Partition {
    type T = B::T;

    fn includes(&self, item: &Self::T) -> bool {
        self.root.includes(&item)
    }

    fn intersects(&self, item: &Self::T) -> bool {
        self.root.intersects(&item)
    }
}

impl<B> Display for QuadTree<B>
    where B: Partition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "QuadTree()")
    }
}