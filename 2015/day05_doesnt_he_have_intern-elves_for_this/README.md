In regards to the problem itself, I found the `tuple_windows` function from the
`Itertools` crate to be extremely useful. Part 2's solution turned out a bit
uglier than I wanted since I tried to do all checks in one string pass so I
chose a 3-character window which made the repeating pair checks a bit awkward.

---

It looks like `filter().count()` is a good way of figuring out how many elements
of a collection fulfill a specific predicate.

The surprising thing is that `filter()` works on &T as opposed to `map()` which
works on T. Since (as the documentation points out) many iterators iterate over
references, this means that the closure passed to `filter()` has a parameter of
type &&T. The solution to be able to call `unwrap()` on the `&&Result<String, Error>`
was to use `as_ref()`.

The definitions would probably help in understanding this so I'll paste them
here for future reference:
```
pub fn map<B, F>(self, f: F) -> Map<Self, F>
 where
     Self: Sized,
     F: FnMut(Self::Item) -> B,

pub fn filter<P>(self, predicate: P) -> Filter<Self, P>
 where
     Self: Sized,
     P: FnMut(&Self::Item) -> bool,
```
