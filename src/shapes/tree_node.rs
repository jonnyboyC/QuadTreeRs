use Result;

pub trait TreeNode<T> {
    fn retrieve(&self, &T) -> Vec<T>;

    fn insert(&mut self, T) -> Result<()>;
}