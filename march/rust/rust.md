# Rust
My plan for learning Rust in March 2026
I aim to spend about 4 hours per week on this.
## Weekly learning
### Week 1 - Ownership & Moves
Implement a CLI tool:
- Reads file
- Counts word frequency
- Outputs sorted results
Constraints:
- No cloning unless necessary
- Explain every move
Questions:
- When does a move occur?
- Why does `String` move but `i32` copy?
### Week 2 - Borrowing & References
Refactor week 1 tool to:
- Avoid unnecessary moves.
- Use references.
- Implement functions that borrow instead of take ownership.
- Intentionally create borrow checker errors, then fix them.
Document:
- Why the error occurs.
- What rule is being enforced.
### Week 3 - Lifetimes
Create: 
- A small struct holding references.
- Functions returning borrowed data.
Add explicit lifetime annotations.
Explain:
- What lifetime annotations actually mean.
- Why Rust can't infer some cases.
### Week 4 - Into to Concurrency
Implement:
- Multi-threaded word counter using `Arc` + `Mutex`
Questions:
- Why is shared mutable state dangerous?
- What problem does `Mutex` solve?
- What is a data race formally?
## Project
Concurrent Word Frequency CLI

Features: 
- Multi-threaded processing of large file
- Safe shared state
- No `unsafe`
- Error handling

Deliverable
> `README` explaining ownership, borrowing, lifetimes, and thread safety in my implementation
## Checkpoint
- Explain why borrow checker errors happen
- Explain difference between move, borrow, and clone
- Explain how `Arc<Mutex<T>>` ensures safety