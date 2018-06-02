use std::fmt;
use std::fmt::{Display, Formatter};
use shapes::partition::Partition;
use shapes::tree_node::TreeNode;
use shapes::bounding_box::BoundingBox;
use Result;

const MAX_ITEMS: usize = 20;

#[derive(Clone)]
pub struct QuadNode<B> 
    where B: Partition {
    level: u16,
    bounds: B,
    items: Vec<B::T>,
    nodes: Option<Vec<QuadNode<B>>>
}

impl<B> QuadNode<B> 
    where B: Partition {
    pub fn new(level: u16, bounds: B) -> QuadNode<B> {
        QuadNode { 
            level, 
            bounds, 
            items: Vec::new(), 
            nodes: None 
        }
    }

    fn size(&self) -> usize {
        match self.nodes {
            Some(ref nodes) => {
                nodes.iter().fold(self.items.len(),
                    |sum, node| sum + node.items.len()
                )
            }
            None => return self.items.len()
        }
    }

    fn split(&mut self) {
        println!("split");

        let mut children: Vec<QuadNode<B>> = self.bounds.split()
            .into_iter()
            .map(|b| QuadNode::new(self.level + 1, b))
            .collect();

        let mut remaining = Vec::new();
        for item in &self.items {
            match self.bounds.get_partition(&item) {
                Some(index) => {
                    children[index].items.push(item.clone())
                },
                None => remaining.push(item.clone()) 
            }
        }

        self.nodes = Some(children);
        self.items = remaining;
    }

    pub fn clear(&mut self) {
        self.items.clear();
        if let Some(ref mut nodes) = self.nodes {
            for mut node in nodes {
                node.clear();
            }
        }
    }

    fn retrieve_all(&self, accumulator: &mut Vec<B::T>) {
        if self.nodes.is_some() {
            for node in self.nodes.unwrap() {
                node.retrieve_all(accumulator)
            }
        }

        accumulator.extend(self.items.iter().map(|x| x.clone()))
    }

    fn retrieve(&self, item: &B::T, accumulator: &mut Vec<B::T>) {
        match self.nodes {
            Some(ref mut nodes) => {
                match self.bounds.get_partition(&item) {
                    Some(index) => nodes[index].retrieve(item, accumulator),
                    None => {
                        for node in nodes {
                            node.retrieve_all(accumulator);
                        }
                    }
                }
            },
            None => ()
        }
 
        accumulator.extend(self.items.iter().map(|x| x.clone()));
    }
}

impl<B> BoundingBox for QuadNode<B> 
    where B: Partition {
    type T = B::T;

    fn includes(&self, item: &Self::T) -> bool {
        self.bounds.includes(item)
    }

    fn intersects(&self, item: &Self::T) -> bool {
        self.bounds.intersects(item)
    }
}

impl<B> TreeNode<B::T> for QuadNode<B> 
    where B: Partition {
    fn insert(&mut self, item: B::T) -> Result<()> {
        let size = self.size();

        match self.nodes {
            Some(ref mut nodes) => {
                match self.bounds.get_partition(&item) {
                    Some(index) => {
                        println!("insert into child {}\n self has {} items all has {} items", nodes[index].bounds, nodes[index].items.len(), size);
                        nodes[index].insert(item)?
                    },
                    None => {
                        println!("inserting into self {} has {} items", self.bounds, self.items.len());
                        self.items.push(item)
                    }
                }
            },
            None => {
                println!("inserting into self {} has {} items all has {} items", self.bounds, self.items.len(), size);
                self.items.push(item);
                if self.items.len() > MAX_ITEMS {
                    self.split()
                };
            }
        }

        Ok(())
    }

    fn retrieve(&self, item: &B::T) -> Vec<B::T> {
        let mut accumulator = Vec::new();
        self.retrieve(item, &mut accumulator);

        accumulator
    }
}

impl<B> Display for QuadNode<B>
    where B: Partition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "QuadNode()")
    }
}