use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

const NUM_ACCOUNTS: usize = 10_000;
const THREADS: usize = 8;
const TRANSFERS_PER_THREAD: usize = 10_000;

fn transfer(bank: &[Mutex<i64>], from: usize, to: usize, amount: i64) {
    if from == to {
        return;
    }

    let (mut first, mut second) = if from < to {
        (bank[from].lock().unwrap(), bank[to].lock().unwrap())
    } else {
        (bank[to].lock().unwrap(), bank[from].lock().unwrap())
    };

    let (from_guard, to_guard) = if from < to {
        (&mut first, &mut second)
    } else {
        (&mut second, &mut first)
    };

    if **from_guard >= amount {
        **from_guard -= amount;
        **to_guard += amount;
    }
}

fn main() {
    let start = Instant::now();
    let mut accounts: Vec<Mutex<i64>> = Vec::with_capacity(NUM_ACCOUNTS);
    accounts.push(Mutex::new(1_000_000));
    for _ in 1..NUM_ACCOUNTS {
        accounts.push(Mutex::new(0));
    }
    let bank = Arc::new(accounts);

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

    let total: i64 = bank.iter().map(|a| *a.lock().unwrap()).sum();
    println!("Elapsed: {:?}", start.elapsed());
    println!("Total: {total}");
    assert_eq!(total, 1_000_000);
}
