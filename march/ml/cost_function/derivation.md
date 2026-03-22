# Derivations

## Cost Function

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

### Why square the error?

If you don't then the sum of the differences between the expected and actual outcome will come to 0 - That is the definition of a linear average. Comparison to MSD in stats

### Why divide by n?

Otherwise it wouldn't be an average, and larger datasets would be needlessly penalized. Comparison to MSD in stats

### What shape are these objects?

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

## Gradient Descent

We want to find the value of $\theta$ where cost is lowest - so we'll want to find a point on the curve where the gradient is 0 (this is assuming it has a local minima - which is fair in this instance considering it is a form of quadratic with a positive a $\theta^2$)

First step will be to apply $\frac{d}{d\theta}$ to both side of our cost function.

$$
\frac{dJ(\theta)}{d\theta} = \frac{1}{2n}\displaystyle\sum_{i=1}^n ((h_{\theta}(x_{i}) - y_{i}).2x_{i})
$$

$$
 => \frac{1}{n}\displaystyle\sum_{i=1}^n (h_{\theta}(x_{i}) - y_{i}).x_{i}
$$

From here you can find a value of $\frac{dJ(\theta)}{d\theta}$ by choosing a particular value of $\theta$, and taking the dot product of the error against the vector $x$. From there you would use another value of $\theta$ choose the lower value, and continue to move in that direction, until the value of $\frac{dJ(\theta)}{d\theta}$ begins to increase again. From there, you know the minima is between those last two values of $\theta$
