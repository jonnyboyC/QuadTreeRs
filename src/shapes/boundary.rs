extern crate num;

use shapes::vec2::Vec2;
use shapes::rectangle::Rectangle;
use std::fmt;


#[derive(Debug)]
pub struct Boundary {
    pub p1: Vec2<f64>,
    pub p2: Vec2<f64>,
    pub norm: Vec2<f64>,
    pub bounds: Rectangle
}

impl Boundary {
    pub fn new(p1: Vec2<f64>, p2: Vec2<f64>) -> Boundary {
        let norm : Vec2<f64> = Vec2::new(p2.x - p1.x, p2.y - p1.y);
        Boundary { 
            bounds: Rectangle::from_points(&p1, &p2),
            p1, 
            p2, 
            norm
        }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.p2.x - self.p1.x
    }
}

impl fmt::Display for Boundary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.p1, self.p2)
    }
}

