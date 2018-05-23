
#![crate_type = "lib"]
#![crate_name = "quad_tree"]

pub mod shapes;
pub mod tree;

mod error;
pub use error::QuadError;

pub type Result<T> = std::result::Result<T, QuadError>;

#[cfg(test)]
mod tests {
    #[test]
    fn not() {
        assert!(true);
    }
}