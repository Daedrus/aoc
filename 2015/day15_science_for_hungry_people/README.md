I wonder if there is a better solution for this than trying out all permutations
of ingredient amounts that sum up to 100. I'd expect that there are some
mathematical ways of maximizing the function(s) which can model this problem.

---

Once I decided on the permutations, the first thing I thought of was that I'd
really like to find something in Itertools which automatically generates them
for me. Unfortunately, I couldn't find anything that would help me generate
all permutations of X numbers that sum up to S (in this case I would have needed
X=2, S=100 for the example input and X=4, S=100 for the real input).

So I started looking into generators only to find out that [they are not yet
supported](https://doc.rust-lang.org/stable/unstable-book/language-features/generators.html)
in stable Rust :(. After a bit more digging I found a crate called `generator`
which somehow implements support for generators using stable Rust. I decided to
try it out and for my particular use-case it worked well. A generator for all
pairs of positive numbers that sum up to 100 looks like this:
```
Gn::new_scoped(|mut s| {
    for i in 0..=100 {
        let j = 100 - i;
        s.yield_([i, j]);
    }
    done!();
})
```

The type of the above construct is iterable which means that you can apply the
usual filter / fold / map functions on it.

I am not too happy of having to define two generators (one for two positive
integers, one for four), but it's the best I can come up with at this moment.
I would love to see a generator which accepts the number of integers as a
parameter as well.

---

Initially I had the two generators duplicated in part1() and part2() but that
was way too much duplication (I already consider them to be "duplicate enough"
since they only differ by the number of integers/for loops). I couldn't figure
out how to create a function which returns a generator (got some type mismatch
issue which I couldn't figure out) so I went for macros, which is something I
haven't used in a long time.

This cleaned things up a bit but the following construction is still super
ugly:

```
if ingredients.len() == 2 {
    permutations2!(teaspoons)
    ...
} else if ingredients.len() == 4 {
    permutations4!(teaspoons)
    ...
} else {
    unreachable!()
}
```

Had I known how to create a macro (or directly a generator) which is not hard
coded for size then the above would just be:
```
permutationsX!(ingredients.len(), teaspoons)
```

There has to be a way, I'll come back to this in the future.

---

One final small thing was that I used `filter_map` for the first time and it's
quite cool. I don't know why it messes with my mind a bit, even as I am
writing this I had to go back and check its definition. I think it's because I
see it as `map the input to an Option and then filter based on that Option` so
it feels more like it should be called `map_filter` or `map_filter_map`.

