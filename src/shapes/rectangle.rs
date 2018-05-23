extern crate num;

use {Result, QuadError};
use std::fmt;
use shapes::vec2::Vec2;
use shapes::bounding_box::BoundingBox;
use shapes::bounding_box_split::BoundingBoxSplit;


#[derive(Clone, Debug, PartialEq)]
pub struct Rectangle {
    pub p1: Vec2<f64>,
    pub p2: Vec2<f64>,
    p_mid: Vec2<f64>
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Result<Rectangle> {
        if width < 0.0 || height < 0.0 {
            Err(QuadError::BadParams)
        } else {
            Ok(Rectangle {
                p1: Vec2::<f64>::new(x, y),
                p2: Vec2::<f64>::new(x + width, y + height),
                p_mid: Vec2::<f64>::new(x + (width * 0.5), y + (height * 0.5)),
            })
        }
    }

    pub fn from_points(p1: &Vec2<f64>, p2: &Vec2<f64>) -> Rectangle{
        if is_ordered(&p1, &p2) {
            Rectangle {
                p1: p1.clone(),
                p2: p2.clone(),
                p_mid: Vec2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0)
            }
        } else { 
            Rectangle {
                p1: Vec2::new(f64::min(p1.x, p2.x), f64::min(p1.y, p2.y)),
                p2: Vec2::new(f64::max(p1.x, p2.x), f64::max(p1.y, p2.y)),
                p_mid: Vec2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0)
            }
        }

    }

    #[inline]
    pub fn width(&self) -> f64 {
        (self.p2.x - self.p1.x).abs()
    }

    #[inline]
    pub fn height(&self) -> f64 {
        (self.p2.y - self.p1.y).abs()
    }
}

#[inline]
fn is_ordered(p1: &Vec2<f64>, p2: &Vec2<f64>) -> bool {
    p1.x < p2.x && p1.y < p2.y
}

impl BoundingBox for Rectangle {
    type T = Rectangle;

    fn intersects(&self, bounds: &Rectangle) -> bool {
        if self.p1.x >= bounds.p2.x || self.p2.x <= bounds.p1.x {
            return false
        }

        if self.p1.y >= bounds.p2.y || self.p2.y <= bounds.p1.y {
            return false
        } 

        true
    }

    fn includes(&self, bounds: &Rectangle) -> bool {
        if self.p1.x >= bounds.p1.x || self.p2.x <= bounds.p2.x {
            return false
        }

        if self.p1.y >= bounds.p1.y || self.p2.y <= bounds.p2.y {
            return false
        }

        true
    }
}

impl BoundingBoxSplit for Rectangle {
    fn split(&self) -> Vec<Rectangle> {
        vec![
            Rectangle::from_points(&self.p_mid, &self.p2),
            Rectangle::new(self.p1.x, self.p_mid.y, self.p_mid.x - self.p1.x, self.p_mid.y - self.p1.y).unwrap(),
            Rectangle::from_points(&self.p1, &self.p_mid),
            Rectangle::new(self.p1.x, self.p_mid.y, self.p_mid.x - self.p1.x, self.p_mid.y - self.p1.y).unwrap(),
        ]
    }

    fn get_bounds(&self, item: &Rectangle) -> Result<usize> {
        if item.p1.y > self.p_mid.y && item.p2.y <= self.p2.y {
            if item.p1.x > self.p_mid.x && item.p2.x <= self.p2.x {
                return Ok(0)
            } else if item.p1.x > self.p1.x {
                return Ok(1)
            }
        } else if item.p1.y > self.p1.y {
            if item.p1.x > self.p_mid.x && item.p2.x <= self.p2.x {
                return Ok(4)
            } else if item.p1.x > self.p1.x {
                return Ok(3)
            }
        }

        Err(QuadError::OutOfBounds)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} x {}", self.p1, self.p2)
    }
}

mod tests {
    use super::*;

    #[test]
    fn rec_constructor() {
        let x = 1.0;
        let y = 2.0;

        let width = 20.0;
        let height = 10.0;

        let rec = Rectangle::new(x, y, width, height).unwrap();
        assert_eq!(rec.p1.x, x);
        assert_eq!(rec.p1.y, y);
        assert_eq!(rec.p2.x, x + width);
        assert_eq!(rec.p2.y, y + height);
    }

    #[test]
    fn rec_invalid_constructor() {
        let x = 0.0;
        let y = 0.0;

        let width = -10.0;
        let height = 5.0;

        let rec = Rectangle::new(x, y, width, height);
        assert!(rec.is_err());
        
        match rec {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn rec_dimension() {
        let x = 1.0;
        let y = 2.0;

        let width = 20.0;
        let height = 10.0;

        let rec = Rectangle::new(x, y, width, height).unwrap();
        assert_eq!(rec.width(), width);
        assert_eq!(rec.height(), height);
    }

    #[test]
    fn check_order() {
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(5.0, 5.0);

        let rec1 = Rectangle::from_points(&p1, &p2);
        let rec2 = Rectangle::from_points(&p2, &p1);

        assert_eq!(rec1, rec2)
    }

    #[test]
    fn check_intersection() {
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(5.0, 5.0);

        let p3 = Vec2::new(2.0, 3.0);
        let p4 = Vec2::new(10.0, 4.0);

        let rec1 = Rectangle::from_points(&p1, &p2);
        let rec2 = Rectangle::from_points(&p1, &p2);
        let rec3 = Rectangle::from_points(&p3, &p4);

        println!("rec1: {}, rec2: {}, rec3: {}", rec1, rec2, rec3);

        assert!(rec1.intersects(&rec2));
        assert!(rec1.intersects(&rec3));
    }

    #[test]
    fn check_include() {
        let rec1 = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();
        let rec2 = Rectangle::new(1.0, 1.0, 5.0, 5.0).unwrap();

        assert!(rec1.includes(&rec2))
    }
}