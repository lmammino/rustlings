fn first(arr: &[i64]) -> &i64 {
    return &arr[0];
}

fn last(arr: &[i64]) -> &i64 {
    return &arr[arr.len() - 1];
}

fn first_two(arr: &[i64]) -> &[i64] {
    return &arr[..2];
}

fn last_two(arr: &[i64]) -> &[i64] {
    return &arr[arr.len()-2..];
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Array:     {:?}", arr);
    println!("First:     {:?}", first(&arr));
    println!("Last:      {:?}", last(&arr));
    println!("First two: {:?}", first_two(&arr));
    println!("Last two:  {:?}", last_two(&arr));
}