fn main() {
    let fib = [1, 1, 2, 3, 5, 8, 13];
    println!("The first {} numbers of the fibonacci sequence are:", fib.len());
    for i in 0..fib.len() {
        println!("fib({}) = {}", i, fib[i]);
    }
}