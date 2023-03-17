Algorithm-wise, I assume that there is a smarter way to do this using graphs.
Not sure if it's possible to find the longest path in an undirected weighted
cyclic graph, I would have to check that.

So, in typical aoc fashion where the inputs are usually small enough for even
brute-force algorithms to work, I chose to go through all possible seating
arrangements and see which one works best.

---

The `circular_tuple_windows` function from the `itertools` crate is perfect
for this problem. By default the window size is 2, for window size larger
than that you need a type hint.

---

I think I figured out why nom wants the tuple function type annotated like
that. I had this issue in an earlier problem as well, I should go back and
fix that, I fixed it in a cleaner way now.

If we look at the tuple function signature:
```
pub fn tuple<I, O, E: ParseError<I>, List: Tuple<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
```

We see that the type E (used for errors) is constrained to error types which
implement the ParseError trait. Looking in `nom-7.1.3/src/error.rs` I can see
at least two different types implementing this trait:
```
impl<I> ParseError<I> for Error<I> {
impl<I> ParseError<I> for VerboseError<I> {
```

Since none of the other parser combinators explicitly restrict this type, the
compiler is left not knowing which type to choose, it could be an Error, it
could be a VerboseError (and possibly a few more). So we have to manually
declare which type to use. I went for Error, but VerboseError works just as
well.

Looking at the compiler error, it seems that it's the other parser combinators
which complain about not knowing the type and having the type known at the
tuple function call site automatically propagates the type to them. Thus,
annotating them with the error type instead of tuple also works.

There is definitely much more to dig into here and there are probably some
deeper lessons about how the compiler infers types but I am satisfied for now.
