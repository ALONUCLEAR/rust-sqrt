## Rust Sqrt

Using <a href="https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method" target="_blank">Heron's method</a> to calculate a sqrt of an f32.

In basic terms, for a given number $S$,
we'll start with an initial guess $x_{0}$
<br /> We'll use the following formula to get another guess, and as we do it over more and more iterations, the approximation will approach the actual sqrt.

The formula is
$x_{n+1} = (x_{n} + \frac{S}{x_{n}})\times\frac{1}{2}$
