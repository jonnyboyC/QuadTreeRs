extern crate quad_tree;
extern crate rand;

use rand::StdRng;
use rand::Rng;
use quad_tree::shapes::rectangle::Rectangle;
use quad_tree::shapes::boundary::Boundary;
use quad_tree::shapes::bounding_box::BoundingBox;
use quad_tree::shapes::vec2::Vec2;

fn main() {

    let pt = Vec2::<f64>::new(0.0, 0.0);
    println!("Point {}", pt);

    let mut rng = StdRng::new().unwrap();

    for _ in 1..100 {
        let width = (rng.next_f64() * 10.0) - 5.0;
        let height = (rng.next_f64() * 10.0) - 5.0;
        let x = rng.next_f64() * 10.0;
        let y = rng.next_f64() * 10.0;

        let rec = Rectangle::new(x, y, width, height);

        match rec {
            Ok(rec) => println!("Good rectangle produced {}", rec),
            Err(e) => println!("x: {}, y: {}, width: {}, height: {} is not valid", x, y, width, e)
        }
    }

    for _ in 1..100 {
        let p1 = Vec2::new(rng.next_f64(), rng.next_f64());
        let p2 = Vec2::new(rng.next_f64(), rng.next_f64());

        let bound = Boundary::new(p1, p2);
        println!("Test boundary {}", bound);
    }


    let rec1 = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();
    let rec2 = Rectangle::new(-2.0, -2.0, 8.0, 8.0).unwrap();

    match rec1.intersects(&rec2) {
        true => println!("{} insecting {}", rec1, rec2),
        false => println!("Ohh no")
    }

}


