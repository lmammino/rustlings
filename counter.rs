struct Counter {
    value: i64,
}

impl Counter {
    fn increase(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut c = Counter {value: 0};
    c.increase();
    c.increase();
    c.increase();
    println!("{}", c.value);
}