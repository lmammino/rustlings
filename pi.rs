use std::f64::consts;

fn circle_area(radius: f64) -> f64 {
    consts::PI * radius * radius
}

fn main() {
    println!("The area of a circle with radius = 17 is {}", circle_area(17.0));
}