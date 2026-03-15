use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn transfer(
    bank: &Arc<Vec<Mutex<i64>>>,
    from: usize,
    to: usize,
    amount: i64,
) -> Result<(), String> {
    thread::sleep(std::time::Duration::from_millis(1));
    let mut from_guard = bank[from].lock().unwrap();
    let mut to_guard = bank[to].lock().unwrap();

    if *from_guard < amount {
        Err("Insufficient funds".to_string())
    } else {
        *from_guard -= amount;
        *to_guard += amount;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let initial_accounts: Vec<Mutex<i64>> = vec![
        Mutex::new(10000),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
    ];

    assert_eq!(
        initial_accounts
            .iter()
            .map(|a| *a.lock().unwrap())
            .sum::<i64>(),
        10000
    );

    let bank = Arc::new(initial_accounts);
    let mut handles = vec![];
    let start = Instant::now();

    for _ in 0..8 {
        let bank = Arc::clone(&bank);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _ = transfer(
                    &bank,
                    rand::random_range(0..=4),
                    rand::random_range(0..=4),
                    rand::random_range(1..=3000),
                );
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Elapsed: {:?}", start.elapsed());
    println!(
        "Final balances: {:?}",
        bank.iter().map(|a| *a.lock().unwrap())
    );
    assert_eq!(bank.iter().map(|a| *a.lock().unwrap()).sum::<i64>(), 10000);

    Ok(())
}
