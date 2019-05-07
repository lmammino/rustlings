fn fact(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    return n * fact(n-1);
}

fn main() {
    println!("fact(17) is {}", fact(17));
}