use fastrand::Rng;

fn main() {
    let mut rng = Rng::new();

    for _ in 0..10000 {
        let value = rng.u32(0..=100);
        println!("{value}");
    }
}
