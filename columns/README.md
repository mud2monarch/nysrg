# Columns

A potentially-parallel bank. Using this to learn three concepts: shared mutable state with Mutex, model actor concurrency communicating via channels, and maybe tokio async.

## Model 1 — Shared state with Mutex

- [x] Set up basic data structures (Bank, Account)
- [x] Simplify to `Arc<Mutex<Vec<f64>>>` (balances only)
- [x] Write `transfer(bank, from, to, amount)` — lock, check funds, subtract/add
- [ ] Spawn 8 threads doing 100 transfers each
- [ ] Assert total money is conserved after all threads join
- [ ] Experiment: try locking individual accounts separately, discover deadlocks