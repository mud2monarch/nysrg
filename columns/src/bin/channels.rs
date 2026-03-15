use std::sync::mpsc;
use std::time::Instant;

enum BankMsg {
    Transfer { from: usize, to: usize, amount: i64 },
}

fn main() -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<BankMsg>();
    let bank: Vec<i64> = vec![10000, 0, 0, 0, 0];
    assert_eq!(bank.iter().sum::<i64>(), 10000);

    let start = Instant::now();
    let actor = std::thread::spawn(move || {
        let mut bank = bank;
        while let Ok(msg) = rx.recv() {
            match msg {
                BankMsg::Transfer { from, to, amount } => {
                    if bank[from] >= amount {
                        bank[from] -= amount;
                        bank[to] += amount;
                    }
                }
            }
        }
        bank
    });

    for _ in 0..8 {
        let sender = tx.clone();
        std::thread::spawn(move || {
            for _ in 0..100 {
                std::thread::sleep(std::time::Duration::from_millis(1));
                sender
                    .send(BankMsg::Transfer {
                        from: rand::random_range(0..=4),
                        to: rand::random_range(0..=4),
                        amount: rand::random_range(1..=3000),
                    })
                    .unwrap();
            }
        });
    }

    drop(tx);
    let final_balances = actor.join().unwrap();

    println!("Elapsed: {:?}", start.elapsed());
    println!("{:?}", final_balances);
    assert_eq!(final_balances.iter().sum::<i64>(), 10000);
    Ok(())
}
