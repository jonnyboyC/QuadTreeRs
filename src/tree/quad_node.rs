use shapes::bounding_box_split::BoundingBoxSplit;
use shapes::bounding_tree_node::BoundingTreeNode;
use shapes::bounding_box::BoundingBox;
use Result;

const MAX_ITEMS: usize = 20;

pub struct QuadNode<B> 
    where B: BoundingBoxSplit {
    level: u16,
    bounds: B,
    items: Vec<B::T>,
    nodes: Option<Vec<QuadNode<B>>>
}

impl<B> QuadNode<B> 
    where B: BoundingBoxSplit {
    pub fn new(level: u16, bounds: B) -> QuadNode<B> {
        QuadNode { 
            level, 
            bounds, 
            items: Vec::new(), 
            nodes: None 
        }
    }

    fn split(&mut self) {
        let new_bounds = self.bounds.split();

        let mut new_nodes = Vec::new();
        let mut remaining_items = Vec::new();

        for bound in new_bounds {
            new_nodes.push(QuadNode::new(self.level + 1, bound));
        }

        // TODO don't be dumb
        for item in &mut self.items {
            match self.bounds.get_bounds(&item) {
                Ok(index) => new_nodes[index].items.push(item.clone()),
                Err(_) => remaining_items.push(item.clone()) 
            }
        }

        self.items = remaining_items;
        self.nodes = Some(new_nodes);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<B> BoundingBox for QuadNode<B> 
    where B: BoundingBoxSplit {
    type T = B::T;

    fn includes(&self, item: &Self::T) -> bool {
        true
    }

    fn intersects(&self, item: &Self::T) -> bool {
        true
    }
}

impl<B> BoundingTreeNode for QuadNode<B> 
    where B: BoundingBoxSplit {

    fn insert(&mut self, item: Self::T) -> Result<()> {
        match self.nodes {
            Some(ref mut nodes) => {
                match self.bounds.get_bounds(&item) {
                    Ok(index) => nodes[index].insert(item),
                    Err(e) => return Err(e)
                }
            },
            None => {
                self.items.push(item);
                if self.items.len() > MAX_ITEMS {
                    self.split()
                }
                Ok(())
            }
        }
    }

    fn retrieve(&self, item: &Self::T) -> Vec<Self::T> {
        Vec::new()
    }
}