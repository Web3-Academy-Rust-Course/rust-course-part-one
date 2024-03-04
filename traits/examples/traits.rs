use std::f64::consts::PI;
use std::fmt::Debug;

// Trait defining common operations for shapes
trait Shape {
    // Hint: Something is missing here
    fn get_area(&self) -> f64;
    fn what_am_i(&self) -> ()
    where
        Self: Debug,
    {
        println!("I am geometrical body: {:?}", self)
    }
}

// Concrete implementation of a rectangle shape
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    // Implementation of get_area on Rectangle
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

// Concrete implementation of a circle shape
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    // Implementation of get_area on Circle
    fn get_area(&self) -> f64 {
        const PI_NUM: f64 = PI;
        PI_NUM * self.radius * self.radius
    }
}

fn main() {
    // Create instances of different shapes
    let rectangle = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    let circle = Circle { radius: 2.0 };
    // Call the
    rectangle.what_am_i();
    circle.what_am_i();
    // Call the get_area method through the trait interface
    println!("Rectangle Area: {}", rectangle.get_area());
    println!("Circle Area: {}", circle.get_area());
}
