mod amort;

fn main() {
    println!("{:?}", amort::amortize(300000.0, 360, 5.0))
}
