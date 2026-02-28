# ML

This is my plan for learning ML in March 2026

## Weekly learning

### Week 1 - Linear Algebra

Study:

- Vectors
- Dot product
- Matrix multiplication
Implement in Python:
- Basic vector ops from scratch (don't use numpy aha)

### Week 2 - Cost Function

Derive:
$$
J(0) = \frac{1}{2m}\sum (h_{\theta}(x) - y)^2
$$
Explain:

- Why square the error?
- Why divide by m?
- What shape are these objects?

### Week 3 - Gradient Descent

Derive update rule:
$$
\theta \colonequals \theta - \alpha \nabla J(\theta)
$$
Implement:

- Gradient descent loop manually.

### Week 4 - Full implementation

Combine:

- Synthetic dataset
- Manual regression
- Plot loss decreasing

Rules:

- No ML frameworks
- Optional: `NumbPy` for convenience only after manual implementation

## Project

Linear Regression from Scratch

Features:

- Derivation in `README`
- Code implementing gradient descent
- Visualization of convergence
- Explanation of learning rate

Deliverable
> I can derive and explain gradient descent without Google

## Checkpoints

- Derive gradient descent update rule
- Explain why it converges
- Explain role of learning rate
