It helped to know that this was similar to [Floyd's Triangle](https://en.wikipedia.org/wiki/Floyd%27s_triangle)
and that each diagonal ended with a [triangular number](https://en.wikipedia.org/wiki/Triangular_number)
; or, in other words, the numbers on the first row are triangular.

The next trick is to realize that given any row and column, you can get
the diagonal index the corresponding number is on with the formula
`row + column - 1`.

We know that each diagonal ends in a triangular number which follows the
formula `n * (n + 1) / 2` where `n` is the diagonal index.

From there I just empirically deduced that the code index at (row, column) is
`n * (n + 1) / 2 - row + 1`.

The rest is just applying the formula given in the problem text over and over.

---

Once again, I really don't like these mathy problems, there is very little to
learn programming wise. I guess this time the f64 and u64 conversions were
interesting, but still.
