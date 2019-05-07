use std::f64::consts;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle {
            radius: radius
        }
    }

    fn area(&self) -> f64 {
        consts::PI * &self.radius * &self.radius
    }

    fn perimeter(&self) -> f64 {
        consts::PI * 2.0 * &self.radius
    }
}

fn main() {
    let c = Circle::new(4.0);
    println!("{:?}\n  area: {}\n  perimeter: {}", c, c.area(), c.perimeter());
}