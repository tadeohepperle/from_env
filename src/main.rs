use std::env;

pub fn main() {
    let e: Vec<String> = env::args().collect();
    dbg!(e);
}
