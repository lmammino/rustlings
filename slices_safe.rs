fn main() {
    let arr = [1, 2, 3, 4, 5];
    let out_of_scope = &arr.get(5);
    println!("{:?}", out_of_scope);
}