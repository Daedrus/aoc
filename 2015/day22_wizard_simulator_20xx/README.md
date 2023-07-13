I would be curious to see how DFS compares to BFS here, I had a hunch that the
state space pruning would go better if I went for BFS but I don't have the
numbers to back it up. Not exploring states which lead to a higher mana cost
than the current minimum is key to this.

The choice of using a `HashMap` for storing the effects doubled the time it
takes to find the solution but I liked it more from a design perspective. I had
initially used three `u32`s in the `State` struct for storing the duration of
the three possible effects.

---

The `retain` function on a `HashMap` for removing keys whose values fulfill a
specific predicate is really useful.
