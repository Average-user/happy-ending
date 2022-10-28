Implementation of the algorithm described in _Computer solution to the
17-point Erdős-Szekeres problem_, to show that any set of 17 points in
the plane in general position has a convex hexagon. See [Happy ending
problem
(Wikipedia)](https://en.wikipedia.org/wiki/Happy_ending_problem).

## Explanation of the problem.

Take an integer `k`. Does there exists another integer `n` such that
any set of `n` points in the plane in general position (no three
points in the same line) contains a convex k-agon (a subset of `k`
points which forms a convex polygon)? <br>
The answer is _yes_, as can be proven by an application of [Ramsey's
theorem](https://en.wikipedia.org/wiki/Ramsey's_theorem). Define `f(k)`
to be the least such `n`.

`f(4) = 5`: clearly a triangle with a single point inside is a
configuration in general position with `4` points and no convex
quadrilateral, so `f(4) >= 5`. Take any configuration of five points
in general position, and draw all possible edges between two
points. Since [`K_5`](https://en.wikipedia.org/wiki/Complete_graph) is
not a planar graph, there are two edges that cross. Their endpoints
form a convex quadrilateral, so `f(4) <= 5`.

Erdős and Szekeres proved that `f(k) >= 2^(k-2) + 1` for all `k >= 3`
and conjectured that equality holds always. `f(5) = 9` can be proved
by brute search rather effortlessly, and the mentioned paper
proves `f(6) = 17`. As time of writing, the conjecture is open
and no values of `f` are known for `k > 6`.
