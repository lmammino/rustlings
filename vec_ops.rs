fn zip(v1: &Vec<i64>, v2: &Vec<i64>) -> Vec<i64> {
    let mut v3 = Vec::new();
    let len = if v1.len() > v2.len() { v1.len() } else { v2.len() };
    for i in 0..len {
        let v1val = v1.get(i);
        if v1val.is_some() {
            v3.push(*v1val.unwrap());
        }
        let v2val = v2.get(i);
        if v2val.is_some() {
            v3.push(*v2val.unwrap());
        }
    }

    return v3;
}

fn concat(v1: &Vec<i64>, v2: &Vec<i64>) -> Vec<i64> {
    let mut v3 = Vec::new();
    for i in v1.iter() {
        v3.push(*i);
    }
    for i in v2.iter() {
        v3.push(*i)
    }

    return v3;
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9];

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("zip(v1,v2): {:?}", zip(&v1, &v2));
    println!("zip(v2,v1): {:?}", zip(&v2, &v1));
    println!("concat(v1,v2): {:?}", concat(&v1, &v2));
    println!("concat(v2,v1): {:?}", concat(&v2, &v1));
}