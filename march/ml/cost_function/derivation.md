# Derivation of the cost function

Derive:

$$
J(\theta) = \frac{1}{2m}\sum (h_{\theta}(x) - y)^2
$$

Explain:

- Why square the error?
- Why divide by m?
- What shape are these objects?

J of theta is the mean of the square difference between expected result and actual result, so starting out by adding together and dividing by the number. J of theta is essentially the ml equivalent of the MSD in stats.

$$
J(\theta) = \frac{(h_{\theta}(x_1) - y_1)^2 + (h_{\theta}(x_2) - y_2)^2 + ... + (h_{\theta}(x_n) - y_n)^2}{n}
$$

From there it's easy to simplify this into a sum

$$
J(\theta) = \frac{1}{n}\displaystyle\sum_{n=1}^n (h_{\theta}(x_{n}) - y_{n})^2
$$

Which through an addition of the constant 0.5 (added because apparently computer scientists are appalling at calculus) brings us to:

$$
J(\theta) = \frac{1}{2n}\displaystyle\sum_{n=1}^n (h_{\theta}(x_{n}) - y_{n})^2
$$

## Why square the error?

If you don't then the sum of the differences between the expected and actual outcome will come to 0 - That is the definition of a linear average. Comparison to MSD in stats

## Why divide by n?

Otherwise it wouldn't be an average, and larger datasets would be needlessly penalized. Comparison to MSD in stats

## What shape are these objects?

$$
J(\theta) - scalar
$$

$$
n - scalar
$$

$$
h_{\theta} - vector
$$

$$
x_{n} - vector
$$

$$
y_{i} - scalar
$$

$$
(h_{\theta}(x_{i}) - y_{i}) - \mathbf{1}\cdot\mathbf{n} - (vector)
$$