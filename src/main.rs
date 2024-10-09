use std::time::Instant;

use glory_rs::Glory;

fn main() {
    let mut glory = Glory::new();

    glory.insert(0, "Hello");
    glory.insert(0, "> ");
    glory.insert(0, "");
    glory.insert(4, " ");
    glory.insert(6, " ");
    glory.insert(8, " ");
    glory.insert(3, " ");
    glory.insert(0, "");

    // let mut tmp = Glory::from("Hello");
    // let time = Instant::now();

    // for _ in 0..1_000_000 {
    //     tmp.insert(4, "Hello");
    // }

    // println!("{:?}", time.elapsed());

    println!("'{glory}'")
}
