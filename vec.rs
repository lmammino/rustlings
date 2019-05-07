fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector:        {:?}", v);
    println!("Vector.get(2): {:?}", v.get(2));
    println!("Vector.get(3): {:?}", v.get(3));
}