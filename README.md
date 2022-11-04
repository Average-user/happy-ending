Implementation of the algorithm described in _Computer solution to the
17-point Erdős-Szekeres problem_, to show that any set of 17 points in
the plane in general position has a convex hexagon. See [Happy ending
problem (Wikipedia)](https://en.wikipedia.org/wiki/Happy_ending_problem).

## Explanation of the problem.

Take an integer `k`. Does there exists another integer `n` such that
any set of `n` points in the plane in general position (no three
points in the same line) contains a convex k-agon (a subset of `k`
points which forms a convex polygon)? <br> The answer is _yes_, as can
be proven by an application of
[Ramsey's theorem](https://en.wikipedia.org/wiki/Ramsey's_theorem). Define
`f(k)` to be the least such `n`.

`f(4) = 5`: clearly a triangle with a single point inside is a
configuration in general position with `4` points and no convex
quadrilateral, so `f(4) >= 5`. Take any configuration of five points
in general position, and draw all possible edges between two
points. Since [`K_5`](https://en.wikipedia.org/wiki/Complete_graph) is
not a planar graph, there are two edges that cross. Their endpoints
form a convex quadrilateral, so `f(4) <= 5`.

Erdős and Szekeres proved that `f(k) >= 2^(k-2) + 1` for all `k >= 3`
and conjectured that equality holds always. `f(5) = 9` can be proved
by brute search rather effortlessly, and the mentioned paper proves
`f(6) = 17`. As time of writing, the conjecture is open and no values
of `f` are known for `k > 6`.

## Results.

(See section 4 of the paper to understand this)

To derive a contradiction from each of the `446` signatures in
`omega*` (which we call simply omega) which start by `1` we do not
perform the `U_13` check, but rather perform `one-`, `two-` and
`three-bit-check`.

The number of assignments that survived the `one-`, `two-` and
`three-bit-check` respectively are `63181`, `18` and `0` respectively,
thus establishing the desired result. A table of all the information
can be found in [results.txt](https://github.com/Average-user/happy-ending/blob/main/results.txt),
where each entry is of the form `idx
1bc 2bc 3bc t`. `idx` is the signature's index, which is just the
decimal representation of the binary number obtained by replacing each
`-1` in the signature by a `0` (recall that a signature is a vector of
`1`s and `-1`s). `1bc` is the number of partial assignments
initialized with the given signature that survived the
`one-bit-check`. Similarly for `2bc` and `3bc`. Finally `t` is the
time in hours that took to derive a contradiction from the particular
signature. There were only five signatures for which it was necessary
to run the `three-bit-check` to derive a contradiction:

``` text
 820242    33  3  0   0.58
 825179    27  3  0   0.55
 983040  4554  6  0  16.86
 983055  7664  3  0  18.39
1015809    39  3  0   0.70
```
So the first entry specifies that for the signature with index
`820242`, there were `33` partial assignments that survived after
the `one-bit-check`, `3` after the `two-bit-check` and none survived
after the `three-bit-check`.
