use std::fmt;

#[derive(Debug)]
pub enum QuadError {
    BadParams,
    OutOfBounds,
}

impl fmt::Display for QuadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuadError::BadParams => write!(f, "Shits boned"),
            QuadError::OutOfBounds => write!(f, "Shits also boned")
        }
    }
}
