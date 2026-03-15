use std::{sync::mpsc, thread, time::Instant};

const NUM_ACCOUNTS: usize = 10_000;
const THREADS: usize = 8;
const TRANSFERS_PER_THREAD: usize = 10_000;

struct Transfer {
    from: usize,
    to: usize,
    amount: i64,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Transfer>();

    let mut bank = Vec::with_capacity(NUM_ACCOUNTS);
    bank.push(1_000_000i64);
    bank.resize(NUM_ACCOUNTS, 0);

    let start = Instant::now();

    let actor = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            if msg.from != msg.to && bank[msg.from] >= msg.amount {
                bank[msg.from] -= msg.amount;
                bank[msg.to] += msg.amount;
            }
        }
        bank
    });

    for _ in 0..THREADS {
        let tx = tx.clone();
        thread::spawn(move || {
            for _ in 0..TRANSFERS_PER_THREAD {
                tx.send(Transfer {
                    from: rand::random_range(0..NUM_ACCOUNTS),
                    to: rand::random_range(0..NUM_ACCOUNTS),
                    amount: rand::random_range(1..=100),
                })
                .unwrap();
            }
        });
    }
    drop(tx);

    let final_balances = actor.join().unwrap();
    let total: i64 = final_balances.iter().sum();
    println!("Elapsed: {:?}", start.elapsed());
    println!("Total: {total}");
    assert_eq!(total, 1_000_000);
}
