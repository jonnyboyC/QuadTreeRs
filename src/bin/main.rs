extern crate quad_tree;
extern crate rand;

use rand::{ StdRng, Rng, FromEntropy};
use quad_tree::Result;
use quad_tree::shapes::rectangle::Rectangle;
use quad_tree::shapes::boundary::Boundary;
use quad_tree::shapes::bounding_box::BoundingBox;
use quad_tree::shapes::tree_node::TreeNode;
use quad_tree::tree::quad_tree::QuadTree;
use quad_tree::shapes::vec2::Vec2;

fn main() {

    let pt = Vec2::<f64>::new(0.0, 0.0);
    println!("Point {}", pt);

    let mut rng = StdRng::from_entropy();
    // let a: f64 = rng.gen();
    // println!("{}", a);
    // test_rec(&mut rng);
    // test_vec(&mut rng);

    let mut qt = QuadTree::new(
        Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap()
    );

    for _ in 1..4000 {
        let insert_rec = rand_rec(&mut rng, 10.0);
        match insert_rec {
            Ok(rec) => {
                println!("Inserting {}", rec);
                match qt.insert(rec.clone()) {
                    Ok(_) => println!("Insert success!"), 
                    Err(_) => println!("could not insert rect {}", rec) 
                }
            }
            Err(e) => println!("fuck {}", e)
        };
    }

    let rec1 = Rectangle::new(0.0, 0.0, 10.0, 10.0).unwrap();
    let rec2 = Rectangle::new(-2.0, -2.0, 8.0, 8.0).unwrap();


    if rec1.intersects(&rec2) {
        println!("{} insecting {}", rec1, rec2)
    } else {
        println!("Ohh no")
    }
}

fn test_vec(rng: &mut StdRng) {
    for _ in 1..100 {
        let p1 = Vec2::new(rng.gen(), rng.gen());
        let p2 = Vec2::new(rng.gen(), rng.gen());

        let bound = Boundary::new(p1, p2);
        println!("Test boundary {}", bound);
    }
}

fn rand_rec(rng: &mut StdRng, size: f64) -> Result<Rectangle> {
    let w: f64 = rng.gen();
    let h: f64 = rng.gen();
    let x1: f64 = rng.gen();
    let y1: f64 = rng.gen();

    let width = w * (size / 8.0);
    let height = h * (size / 8.0);
    let x = x1 * size;
    let y = y1 * size;

    Rectangle::new(x, y, width, height)
}

fn test_rec(rng: &mut StdRng) {
    for _ in 1..100 {
        let rec = rand_rec(rng, 10.0);

        match rec {
            Ok(rec) => println!("Good rectangle produced {}", rec),
            Err(e) => println!("Couldn't produce rectangle {}", e)
        }
    }
}


