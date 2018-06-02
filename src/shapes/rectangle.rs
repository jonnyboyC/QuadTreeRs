extern crate num;

use {Result, QuadError};
use std::fmt;
use shapes::vec2::Vec2;
use shapes::bounding_box::BoundingBox;
use shapes::partition::Partition;


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
    type T = Self;

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

#[inline]
fn horizontal_between(
    bound_left: &Vec2<f64>, 
    bound_right: &Vec2<f64>,
    target_left: &Vec2<f64>,
    target_right: &Vec2<f64>) -> bool {
    debug_assert!(bound_left.x < bound_right.x);
    debug_assert!(target_left.x < target_right.x);

    bound_left.x < target_left.x && bound_right.x >= target_right.x
}

#[inline]
fn vertical_between(
    bound_bot: &Vec2<f64>, 
    bound_top: &Vec2<f64>, 
    target_bot: &Vec2<f64>, 
    target_top: &Vec2<f64>) -> bool {

    debug_assert!(bound_bot.y < bound_top.y);
    debug_assert!(target_bot.y < target_top.y);

    bound_bot.y < target_bot.y && bound_top.y >= target_top.y
}

impl Partition for Rectangle {
    fn split(&self) -> Vec<Rectangle> {
        vec![
            Rectangle::from_points(&self.p_mid, &self.p2),
            Rectangle::new(self.p1.x, self.p_mid.y, self.p_mid.x - self.p1.x, self.p_mid.y - self.p1.y).unwrap(),
            Rectangle::from_points(&self.p1, &self.p_mid),
            Rectangle::new(self.p_mid.x, self.p1.y, self.p_mid.x - self.p1.x, self.p_mid.y - self.p1.y).unwrap(),
        ]
    }

    fn get_partition(&self, item: &Rectangle) -> Option<usize> {
        if vertical_between(&self.p_mid, &self.p2, &item.p1, &item.p2) {
            if horizontal_between(&self.p_mid, &self.p2, &item.p1, &item.p2) {
                return Some(0)
            } else if horizontal_between(&self.p1, &self.p_mid, &item.p1, &item.p2) {
                return Some(1)
            }
        } else if vertical_between(&self.p1, &self.p_mid, &item.p1, &item.p2) {
            if horizontal_between(&self.p_mid, &self.p2, &item.p1, &item.p2) {
                return Some(3)
            } else if horizontal_between(&self.p1, &self.p_mid, &item.p1, &item.p2) {
                return Some(2)
            }
        }

        None
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectange({} x {})", self.p1, self.p2)
    }
}

mod tests {
    use super::*;

    #[test]
    fn constructor() {
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
    fn invalid_constructor() {
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
    fn dimension() {
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
    fn intersection() {
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(5.0, 5.0);

        let p3 = Vec2::new(2.0, 3.0);
        let p4 = Vec2::new(10.0, 4.0);

        let rec1 = Rectangle::from_points(&p1, &p2);
        let rec2 = Rectangle::from_points(&p1, &p2);
        let rec3 = Rectangle::from_points(&p3, &p4);

        let rec4 = Rectangle::from_points(&p2, &p4);
        let rec5 = Rectangle::from_points(&p1, &p3);

        assert!(rec1.intersects(&rec2));
        assert!(rec2.intersects(&rec1));

        assert!(rec1.intersects(&rec3));
        assert!(rec3.intersects(&rec1));
        
        assert!(!rec4.intersects(&rec5));
        assert!(!rec5.intersects(&rec4));
    }

    #[test]
    fn include() {
        let rec1 = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();
        let rec2 = Rectangle::new(1.0, 1.0, 5.0, 5.0).unwrap();

        let rec3 = Rectangle::new(4.0, 4.0, 5.0, 5.0).unwrap();

        assert!(rec1.includes(&rec2));
        assert!(!rec2.includes(&rec1));

        assert!(!rec2.includes(&rec3));
        assert!(!rec3.includes(&rec2));

        assert!(rec1.includes(&rec3));
        assert!(!rec3.includes(&rec1));
    }

    #[test]
    fn split() {
        let rec = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();

        let sub_recs = rec.split();
        assert_eq!(sub_recs[0], Rectangle::new(5.0, 5.0, 5.0, 5.0).unwrap());
        assert_eq!(sub_recs[1], Rectangle::new(0.0, 5.0, 5.0, 5.0).unwrap());
        assert_eq!(sub_recs[2], Rectangle::new(0.0, 0.0, 5.0, 5.0).unwrap());
        assert_eq!(sub_recs[3], Rectangle::new(5.0, 0.0, 5.0, 5.0).unwrap());
    }

    #[test]
    fn get_bounds() {
        let rec = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();
        let big_rec = Rectangle::new(-5.0, -5.0, 20.0, 20.0).unwrap();

        let offset_rec = Rectangle::new(6.0, 6.0, 4.0, 6.0).unwrap();

        assert_eq!(rec.get_partition(&big_rec), None);
        assert_eq!(rec.get_partition(&offset_rec), None);

    }
}