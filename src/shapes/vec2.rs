extern crate num;

use self::num::Float;
use std::fmt::Display;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

impl<T: Float> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T: Float + Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_constructor() {
        let x = 1.0;
        let y = 2.0;

        let a: Vec2<f64> = Vec2::new(x, y);
        assert_eq!(a.x, x);
        assert_eq!(a.y, y);
    }

    #[test]
    fn vec2_add() {
        let a = Vec2::new(0.0, 5.0);
        let b = Vec2::new(10.0, 15.0);

        assert_eq!(Vec2 {x: 10.0, y: 20.0}, a + b);
    }

    #[test]
    fn vec2_display() {
        let a = Vec2::new(0.0, 10.0);

        assert_eq!(a.to_string(), "(0, 10)")
    }
}

