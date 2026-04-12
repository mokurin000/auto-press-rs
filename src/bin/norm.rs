use auto_press_rs::rng::NormalInRange;
use fastrand::Rng;

fn main() {
    let mut rng = Rng::new();

    for _ in 0..10000 {
        let value = rng.norm_rand(0..=100);
        println!("{value}");
    }
}
