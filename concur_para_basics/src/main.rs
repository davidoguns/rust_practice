use rand::distr::uniform::{UniformDuration, UniformSampler};
use std::thread;
use std::time::Duration;

fn main() {
    let hndl = thread::spawn(|| {
        let mut seeded_rng: rand::rngs::ChaCha20Rng = rand::make_rng();
        let sleep_rng =
            UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();
        for i in 1..10 {
            thread::sleep(sleep_rng.sample(&mut seeded_rng));
            println!("Spawned thread = {i}");
        }
    });

    let mut seeded_rng: rand::rngs::ChaCha20Rng = rand::make_rng();
    let sleep_rng =
        UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();

    for i in 1..10 {
        thread::sleep(sleep_rng.sample(&mut seeded_rng));
        println!("Main thread    = {i}");
    }
    if let Err(e) = hndl.join() {
        eprintln!("Error waiting for handle: {e:?}");
    }
}
