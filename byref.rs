fn add_one(n: &mut i64) {
    *n = *n + 1
}

fn add_two(n: &mut i64) {
    *n = *n + 2;
}

fn main() {
    let mut n: i64 = 17;
    add_one(&mut n);
    add_two(&mut n);
    println!("17 + 1 + 2 is {}", n);
}