Very cheeky part 2. I basically winged it after seeing the example and
thinking that "surely that's the least common multiple". I looked at the input
and saw that the paths cycle (aka the end node cycles back to the start node)
and then I just prayged that the cycles are "clean". It worked, but solution
would definitely break on an input that's not this constrained.

---

Being able to pass functions as arguments leads to some really nice code since
you can code what an end node is through a:
```
is_end_node: fn(&str) -> bool,
```
And use that as a break condition for the traversal.

---

Imported a new crate called `num` for the `lcm` implementation. I had not
pulled in a new crate in a very long time.

---

To have an iterator go back to the start when it reaches the end one can use
the `cycle` function. I am surprised I have not used this before. It made it
very easy to cycle through the instructions endlessly.
