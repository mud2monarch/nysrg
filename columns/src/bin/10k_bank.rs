use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

const NUM_ACCOUNTS: usize = 10_000;
const THREADS: usize = 8;
const TRANSFERS_PER_THREAD: usize = 10_000;

fn transfer(bank: &Arc<Mutex<Vec<i64>>>, from: usize, to: usize, amount: i64) {
    let mut accounts = bank.lock().unwrap();
    if from != to && accounts[from] >= amount {
        accounts[from] -= amount;
        accounts[to] += amount;
    }
}

fn main() {
    let start = Instant::now();
    let mut initial: Vec<i64> = vec![0; NUM_ACCOUNTS];
    initial[0] = 1_000_000;
    let bank = Arc::new(Mutex::new(initial));

    let handles: Vec<_> = (0..THREADS)
        .map(|_| {
            let bank = Arc::clone(&bank);
            thread::spawn(move || {
                for _ in 0..TRANSFERS_PER_THREAD {
                    transfer(
                        &bank,
                        rand::random_range(0..NUM_ACCOUNTS),
                        rand::random_range(0..NUM_ACCOUNTS),
                        rand::random_range(1..=100),
                    );
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let total: i64 = bank.lock().unwrap().iter().sum();
    println!("Elapsed: {:?}", start.elapsed());
    println!("Total: {total}");
    assert_eq!(total, 1_000_000);
}
