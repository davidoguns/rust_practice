use rand::SeedableRng;
use rand::distr::uniform::{UniformDuration, UniformSampler};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

fn main() {
    let hndl = thread::spawn(|| {
        let micro_since_epoch = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
            //
            Ok(duration) => {
                // Get the total number of microseconds as a u128
                duration.as_micros()
            }
            Err(e) => {
                panic!("System clock moved backwards! Error: {}", e);
            }
        } as u64;
        let mut seeded_rng = rand::rngs::ChaCha20Rng::seed_from_u64(micro_since_epoch);
        let sleep_rng =
            UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();
        for i in 1..10 {
            
            thread::sleep(sleep_rng.sample(&mut seeded_rng));
            println!("Spawned thread = {i}");
        }
    });

    let micro_since_epoch = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
        //
        Ok(duration) => {
            // Get the total number of microseconds as a u128
            duration.as_micros()
        }
        Err(e) => {
            panic!("System clock moved backwards! Error: {}", e);
        }
    } as u64;
    let mut seeded_rng = rand::rngs::ChaCha20Rng::seed_from_u64(micro_since_epoch);
    let sleep_rng =
        UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();

    for i in 1..10 {
        thread::sleep(sleep_rng.sample(&mut seeded_rng));
        println!("Main thread    = {i}");
    }    if let Err(e) = hndl.join() {
        eprintln!("Error waiting for handle: {e:?}");
    }
}
