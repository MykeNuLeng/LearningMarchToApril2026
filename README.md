# Learning - March To May 2026
I'm hoping that a sense of accountability will be enough to make sure I stick with this, so that's why I'm posting this here.
## Overall learning goals for March through May
### iOS 
Build small but complete features, get comfortable with SwiftUI & UIKit.
### Rust 🦀
Learn about ownership, borrowing, lifetimes, and basic concurrency.
### ML maths ∬
Refresh on linear algebra, probability, and basic calculus
## Weekly tasks for March
### iOS development
Continue to progress through 100 days of Hacking with Swift
### Rust
#### Week 1 - Ownership & Moves
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
#### Week 2 - Borrowing & References
Refactor week 1 tool to:
- Avoid unnecessary moves.
- Use references.
- Implement functions that borrow instead of take ownership.
- Intentionally create borrow checker errors, then fix them.
Document:
- Why the error occurs.
- What rule is being enforced.
#### Week 3 - Lifetimes
Create: 
- A small struct holding references.
- Functions returning borrowed data.
Add explicit lifetime annotations.
Explain:
- What lifetime annotations actually mean.
- Why Rust can't infer some cases.
#### Week 4 - Into to Concurrency
Implement:
- Multi-threaded word counter using `Arc` + `Mutex`
Questions:
- Why is shared mutable state dangerous?
- What problem does `Mutex` solve?
- What is a data race formally?
### ML maths
#### Week 1 - Linear Algebra
Study: 
- Vectors
- Dot product
- Matrix multiplication
Implement in Python:
- Basic vector ops from scratch (don't use numpy aha)
#### Week 2 - Cost Function
Derive: 
$$
J(0) = \frac{1}{2m}\sum (h_{\theta}(x) - y)^2
$$
Explain:
- Why square the error?
- Why divide by m?
- What shape are these objects?
#### Week 3 - Gradient Descent
Derive update rule:
$$
\theta \colonequals \theta - \alpha \nabla J(\theta)
$$
Implement:
- Gradient descent loop manually.
#### Week 4 - Full implementation
Combine:
- Synthetic dataset
- Manual regression
- Plot loss decreasing

Rules:
- No ML frameworks
- Optional: NumbPy for convenience only after manual implementation
## Projects
### iOS
Profile Feature Module

Not a full app - a feature module structured as if it could drop into production.

Include:
- SwiftUI version
- UIKit version
- View model layer
- Networking layer
- Persistence layer
- Tests
- README explaining all architecture decisions

Deliverable
> A repository that proves I can design and ship a complete feature independently

### Rust
Concurrent Word Frequency CLI

Features: 
- Multi-threaded processing of large file
- Safe shared state
- No `unsafe`
- Error handling

Deliverable
> README explaining ownership, borrowing, lifetimes, and thread safety in my implementation

### ML maths
Linear Regression from Scratch

Features:
- Derivation in README
- Code implementing gradient descent
- Visualization of convergence
- Explanation of learning rate

Deliverable
> I can derive and explain gradient descent without Googling

