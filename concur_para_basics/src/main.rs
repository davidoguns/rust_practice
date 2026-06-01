use rand::distr::uniform::{UniformDuration, UniformSampler};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx_1, rx) = mpsc::channel();
    let tx_2 = tx_1.clone(); // have to explicitly clone sender

    let pure_producer_hndl = thread::spawn(move || {
        for s in vec![
            String::from("who"),
            String::from("dat"),
            String::from("there"),
        ] {
            match tx_1.send(s) {
                Ok(_) => (),
                Err(e) => panic!("Failure to send from pure sender: {e:?}"),
            }
        }
    });

    let hndl = thread::spawn(move || {
        let mut seeded_rng: rand::rngs::ChaCha20Rng = rand::make_rng();
        let sleep_rng =
            UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();
        for i in 1..10 {
            thread::sleep(sleep_rng.sample(&mut seeded_rng));
            println!("Spawned thread = {i}");
        }

        thread::sleep(Duration::from_secs(1));

        for i in 1..5 {
            let sleep_rng =
                UniformDuration::new(Duration::from_secs(1), Duration::from_secs(5)).unwrap();
            let delay_secs = sleep_rng.sample(&mut seeded_rng);
            println!("Delaying {} seconds...", delay_secs.as_secs());
            thread::sleep(delay_secs);
            match tx_2.send(format!("Sending = {i}").to_string()) {
                Ok(_) => (),
                Err(e) => { panic!("Error sending message: {e:?}") },
            }
        }
    });

    let mut seeded_rng: rand::rngs::ChaCha20Rng = rand::make_rng();
    let sleep_rng =
        UniformDuration::new(Duration::from_millis(1), Duration::from_millis(50)).unwrap();
    for i in 1..10 {
        thread::sleep(sleep_rng.sample(&mut seeded_rng));
        println!("Main thread    = {i}");
    }

    // How does rx know it's done? Likely because all "tx" borrows or clones have
    // been dropped
    for msg in rx {
        println!("Got message from spawned thread: {msg}");
    }

    // Not useful other than live watching delays or lack therof
    println!("Waiting for spawned thread");
    if let Err(e) = hndl.join() {
        eprintln!("Error waiting for handle: {e:?}");
    }
    println!("Waiting for pure producer thread");
    if let Err(e) = pure_producer_hndl.join() {
        eprintln!("Error waiting for producer handle: {e:?}");
    }
}

