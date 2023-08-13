From an algorithmic perspective, were the input size a lot larger than it is,
it would have been better to store the "visited locations" as lines and just
run some line intersection checks after each instruction. However, for such a
small input size, storing each location in a HashSet works fine.

---

Not a big fan of how the code looks when using `ControlFlow` constructs, it
could probably be done better.
