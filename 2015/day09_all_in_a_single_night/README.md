This is the travelling salesman problem but since the input is so small I
didn't bother to implement anything smart so I just check all of the location
permutations and see which one is best. Classical aoc approach, I have to say.

---

I wanted to drop the separate `nom` parsing function (`parse_distance`) and
just inline it in the `parse_input` function but I got a lot of strange
type-related issues which I couldn't figure out at all. Eventually I asked
ChatGPT to fix my code and while it didn't succeed, it did manage to spit
out the correct type annotations for the `alpha1` functions.

It seems that I am not the only person to run into these:
https://users.rust-lang.org/t/lost-in-generic-inference-and-lifetimes-with-nom-and-its-only-a-6-statement-function/68120/2

I do not understand these issues at all yet, I hope `rustlings` covers them
at some point.

---

I heard you liked to `to_string` so I made sure to force you to `to_string`
everywhere since the `to_string` pattern is so nice. I am referring mainly to
the `to_string` calls in the HashMap `.get` calls.

Naturally, someone before me has already had this issue:
https://users.rust-lang.org/t/hashmap-with-tuple-keys/12711/9

All options in there seem ugly, except OP's suggestion to implement
`impl Borrow<(&X, &Y)> for (X,Y)` which I am currently unable to due to
extreme noobness.
