All right, so the tactic here is to keep two 2d arrays, one to represent the
board state in the previous iteration and one to modify during the current
iteration. At the beginning of each iteration, you swap between the two. In
this way you avoid allocating new memory over and over in every iteration
whenever you want to change the board state (since current board state depends
on previous board state).

---

The biggest thing I struggled with here was how to represent a 2d array. I
experimented with arrays and had trouble figuring out how to dynamically
allocate them both on the stack _and_ on the heap. I also didn't want to
use `Vec` since that type is meant for data that can grow and I just wanted
to allocate a fixed-sized array once, where the size would be decided at
runtime.

I eventually gave up and used the `ndarray` crate and `Box`ed the provided
2d arrays so that they're on the heap (assuming that we want the program to
handle large grids, having that much data on the stack feels like a poor
choice).

Variable length arrays _might_ be supported at some point as mentioned
[here](https://doc.rust-lang.org/beta/unstable-book/language-features/unsized-locals.html#variable-length-arrays)

My worry with `ndarray` is that it seems to be poorly maintained but for my
aoc needs it should suffice since I doubt I will ever need advanced
functionality.

---

The swapping part was actually kind of neat and to my surprise it even worked
on `Box`es. I can't fully tell _why_ it works on `Box`es though, I can't find
anything in the documentation. In any case, it was as simple as doing this:
```
std::mem::swap(&mut self.lights, &mut self.old_lights);
```

---

In order to figure out the size of the board (or `Grid` as I call it in the
implementation) I wanted to peek at the first line in the file and get its
size (and then just assume that all the following lines are of the same size).
There is a `peekable` iterator available but it seems that that still consumes
the element when you call `peek()`. So I ended up just adding another trait
constraint to the input and made it have to support `Seek`. In that way, I
could rewind the input after getting the length of the first line. There has to
be a better way to do this.

