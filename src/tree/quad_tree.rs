use tree::quad_node::QuadNode;
use shapes::bounding_box::BoundingBox;
use shapes::bounding_box_split::BoundingBoxSplit;
use shapes::bounding_tree_node::BoundingTreeNode;
use Result;

pub struct QuadTree<B>
    where B: BoundingBoxSplit {
    root: Box<QuadNode<B>>
}

impl<B> QuadTree<B>
    where B: BoundingBoxSplit {
    pub fn new(bounds: B) -> QuadTree<B> {
        QuadTree { 
            root: Box::new(
                QuadNode::new(0, bounds)
            ) 
        }
    }
}

impl<B> BoundingBox for QuadTree<B> 
    where B: BoundingBoxSplit {
    type T = B::T;

    fn includes(&self, item: &Self::T) -> bool {
        self.root.includes(&item)
    }

    fn intersects(&self, item: &Self::T) -> bool {
        self.root.intersects(&item)
    }
}

impl<B> BoundingTreeNode for QuadTree<B> 
    where B: BoundingBoxSplit {
    
    fn insert(&mut self, item: Self::T) -> Result<()> {
        self.root.insert(item)
    }

    fn retrieve(&self, item: &Self::T) -> Vec<Self::T> {
        Vec::new()
    }
}