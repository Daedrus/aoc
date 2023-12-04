I wonder if there is a way to make the `nom` `tuple`s clearer by removing all
of those `multispace1`s and their associated `_` in the let binding. Handling
all that whitespace really clutters the code.

---

I am not sure which is better, `fold` or `map` + `sum`. I eventually settled
for `map` + `sum` in part 1 since I feel like it is more readable but that is
very subjective. I would be curious how they compare performance wise, I wonder
if the compiler is smart enough to generate the same code for both approaches.
I should dig into this...

---

The really cool part about the slice in part 2 is that it automatically wraps
at the end so if `scratchcard.index + 1 + matching_numbers` exceeds the size of
`scratchcard_instances` then the range will only go to the last element and not
throw an error. Neat.
