#[macro_use]

pub mod numerics;

fn main() {
    println!("Hello, world!");
    let x = vec![0.1,0.2,0.3,0.4];
    let y = vec![0.2,0.2,0.3,0.4];
    println!("{}", numerics::euclidean(&x, &y));
    println!("{}", numerics::dtw1d(&x, &y, 0));
    println!("{:?}", numerics::hamming(11));
}
