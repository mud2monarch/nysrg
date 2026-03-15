# Columns

A potentially-parallel bank. Using this to learn three concepts: shared mutable state with Mutex, model actor concurrency communicating via channels, and maybe tokio async.

## Model 1 - Shared state over entire bank/accounts list

Lock entire "bank" whenever `transfer`. Data structure is `Arc<Mutex<Vec<i64>>>`.

Did this at `5198ef5`.

```bash
Elapsed: 1.027906792s
Final balances: [148, 4370, 1202, 3841, 439]
```

## Model 2 - Shared state over accounts

Provision locks/mutex over each individual account. Data structure is `Arc<Vec<Mutex<i64>>>`.

Did this but ran into deadlocks at `546e72f`.

Ensure deterministic locking to prevent deadlocks at `26965af`. 

```bash
Elapsed: 127.138917ms
Final balances: Map { iter: Iter([Mutex { data: 5054, poisoned: false, .. }, Mutex { data: 207, poisoned: false, .. }, Mutex { data: 1076, poisoned: false, .. }, Mutex { data: 1236, poisoned: false, .. }, Mutex { data: 2427, poisoned: false, .. }]) }
```
 ## Model 3 - Actor model, communicate via channels