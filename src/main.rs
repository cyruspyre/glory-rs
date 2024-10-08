use std::time::Instant;

use glory_rs::Glory;
// An unknown error happening when there it has to go reverse after a series of forward
fn main() {
    let mut glory = Glory::from("Hello");

    glory.insert(0, "> ");
    glory.insert(6, " ");
    glory.insert(3, " ");
    glory.insert(6, " ");
    glory.insert(5, " ");
    glory.insert(0, "");
    // glory.insert(0, "");

    // let mut tmp = Glory::from("Hello");
    // let time = Instant::now();

    // for _ in 0..1_000_000 {
    //     tmp.insert(4, "Hello");
    // }

    // println!("{:?}", time.elapsed());

    println!("'{glory}'")
}
