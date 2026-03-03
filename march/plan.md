# March

I'm going to be splitting my time with 6 hours a week on iOS dev, 4 hours on Rust and 2 on ML.

## Weekly tasks for March

### iOS development

Continue to progress through 100 days of Hacking with Swift.

## Week 1 ✅

### Rust - Ownership & Moves ✅

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

### ML - Linear Algebra ✅

Study:

- Vectors
- Dot product
- Matrix multiplication

Implement in Python:

- Basic vector ops from scratch (don't use numpy aha)

## Week 2

### Rust - Borrowing & References

Refactor week 1 tool to:

- Avoid unnecessary moves.
- Use references.
- Implement functions that borrow instead of take ownership.
- Intentionally create borrow checker errors, then fix them.

Document:

- Why the error occurs.
- What rule is being enforced.

### ML - Cost Function

Derive:
$$
J(0) = \frac{1}{2m}\sum (h_{\theta}(x) - y)^2
$$
Explain:

- Why square the error?
- Why divide by m?
- What shape are these objects?

## Week 3

### Rust - Lifetimes

Create:

- A small struct holding references.
- Functions returning borrowed data.
- Add explicit lifetime annotations.

Explain:

- What lifetime annotations actually mean.
- Why Rust can't infer some cases.

### ML - Gradient Descent

Derive update rule:
$$
\theta \colonequals \theta - \alpha \nabla J(\theta)
$$
Implement:

- Gradient descent loop manually.

## Week 4

### Rust - Intro to Concurrency

Implement:

- Multi-threaded word counter using `Arc` + `Mutex`
Questions:
- Why is shared mutable state dangerous?
- What problem does `Mutex` solve?
- What is a data race formally?

### ML - Full implementation

Combine:

- Synthetic dataset
- Manual regression
- Plot loss decreasing

Rules:

- No ML frameworks
- Optional: `NumbPy` for convenience only after manual implementation
