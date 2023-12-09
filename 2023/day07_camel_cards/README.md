This is an example of a problem where the difficulty comes from figuring out
how to match an input (in this case the poker hand strings) to a given type
(the category of the hand). I remember seeing a few of these before and the
hardest part is almost always to make sure that you've covered all possible
cases. It is very easy to miss one or two, have the example input pass and then
spend hours on the actual input trying to figure out where things go wrong.

I am not a big fan of this (since the puzzles themselves are not my main
interest, but rather the programming itself) so I have no qualms with browsing
reddit and seeing if other people have the same issues as me. That was the case
this time as well, I had missed one of the Joker cases. If I had more free time
I wouldn't mind spending it to try and figure out the issue myself, but since
that is not the case, I am fine with this approach.

---

On the actual coding side, I did learn some new things (or perhaps remembered
things that I had forgotten, not sure) so this was one of the most useful aoc
problems I've solved in a long while.

---

You can impose an order on the variants of an enum by doing:
```
#[derive(PartialEq, PartialOrd)]
enum Card {
```

Very useful in this case for both the category ranking and the individual card
ranking (when starting part 2, adding the Joker to the top of the enum solved
a lot of problems for free).

---

It was the first time I implemented the `From` trait from a tuple. Useful for
interpreting the `J` character differently in the two parts:

```
impl From<(char, bool)> for Card {
```

---

I really wanted to just call `sort` on the array of `Hand` so for that I had
to implement the `Ord`, `PartialEq` and `PartialOrd` traits for it. [This
article](https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/)
came in useful, together with some clippy hints.

---

Matching on array slices was something suggested by ChatGPT, I had not thought
of it. Very useful for deriving the hand categories from the sorted card
frequency patterns:
```
let category = match card_frequency_values[..] {
    [5] => ...
    [1, 4] => ...
    [2, 3] => ...
    [1, 1, 3] => ...
    [1, 2, 2] => ...
    [1, 1, 1, 2] => ...
    [1, 1, 1, 1, 1] => ...
```
