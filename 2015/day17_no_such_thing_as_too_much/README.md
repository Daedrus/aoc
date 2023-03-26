There were quite a few of these in 2022 where the solution involved careful
generation and navigation of the state space, along with aggressive pruning
strategies. Sometimes there exist smarter algorithms that don't involve
state space traversal so it's really interesting to look at how people solve
these problems.

I am not sure if there is a better way of generating all combinations of a
set which fulfill a specific criteria (in this case, they sum up to 150).

I also find that sorting the input data usually helps in some way, and this
was the case here as well, it allowed me to prune the branches where I had
X liters left, but none of the remaining containers had a capacity smaller
than X.

---

I would have liked to create a generator for the solutions instead of storing
them in a `Vec<Vec<u32>>` but I wasn't sure I would be able to create one
when recursion is involved. Perhaps I'll give it ago when generators make it
into stable Rust.

---

I am not sure if there is a better way to clone a slice than `clone_from_slice`.

---

For the second part `Itertools` `min_set_by_key` function was really useful.
