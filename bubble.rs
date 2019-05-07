fn bubblesort(arr: &mut [i64]) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                let tmp: i64 = arr[j];
                arr[j] = arr[i];
                arr[i] = tmp;
            }
        }
    }
}

fn main() {
    let mut arr = [3, 17, 22, 1, 12, 1, 1, 3, 4, 19, 22, 50, 4];
    println!("Unsorted: {:?}", arr);
    bubblesort(&mut arr);
    println!("  Sorted: {:?}", arr);
}

