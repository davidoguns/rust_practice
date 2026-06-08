use rand::distr::uniform::{UniformDuration, UniformSampler};
use std::sync::atomic::AtomicU32;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};

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

    let atm_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for thr_idx in 1..=50 {
        let thrd_counter = Arc::clone(&atm_counter);
        handles.push(std::thread::spawn(move || {
            for try_idx in 1..=100 {
                match thrd_counter.try_lock() {
                    Ok(mut counter) => {
                        *counter += 1;
                        println!("Thread {thr_idx} succeeded after {try_idx} try_lock() attempts");
                        return; //ends thread::spawn()
                    },
                    Err(_) => (),
                }
                thread::sleep(Duration::from_millis(10));
            }
            println!("Try locks failed...using blocking lock");
            let mut counter = thrd_counter.lock().unwrap();
            *counter += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Arc::Mutex counter final value {}", atm_counter.lock().unwrap());

    let atm_counter_2 = Arc::new(AtomicU32::new(0));
    let atm_counter_1_ref = Arc::clone(&atm_counter_2);
    let atm_counter_2_ref = Arc::clone(&atm_counter_2);
    let atm_counter_3_ref = Arc::clone(&atm_counter_2);
    let mut handles = vec![];
    handles.push(std::thread::spawn(move || {
        atm_counter_1_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    handles.push(std::thread::spawn(move || {
        atm_counter_2_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    handles.push(std::thread::spawn(move || {
        atm_counter_3_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));
    for _ in 1..=20 {
        let atm_counter_ref = Arc::clone(&atm_counter_2);
        handles.push(std::thread::spawn(move || {
            atm_counter_ref.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Atomic counter final value {}", atm_counter_2.load(std::sync::atomic::Ordering::Acquire));
}

