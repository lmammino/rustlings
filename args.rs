use std::env;

// run with `./rrun args 1 2 3 4 5`

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();
    println!("{:?}", args);
}