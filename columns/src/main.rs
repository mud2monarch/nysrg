use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn transfer(
    bank: &Arc<Mutex<Vec<i64>>>,
    from: usize,
    to: usize,
    amount: i64,
) -> Result<(), String> {
    let mut accounts = bank.lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(1));
    if accounts[from] < amount {
        Err("Insufficient funds".to_string())
    } else {
        accounts[from] -= amount;
        accounts[to] += amount;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let initial_accounts: Vec<i64> = vec![10000, 0, 0, 0, 0];
    assert_eq!(initial_accounts.iter().sum::<i64>(), 10000);

    let bank = Arc::new(Mutex::new(initial_accounts));
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
    println!("Final balances: {:?}", bank.lock().unwrap());
    assert_eq!(bank.lock().unwrap().iter().sum::<i64>(), 10000);

    Ok(())
}
