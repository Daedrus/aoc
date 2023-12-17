I initially wanted to spend some time cleaning this solution up but I decided
not to since I already spent too much time just getting part 2 to work. I'll
leave it as is as a testament to my current Rust level and aoc problem solving
skills.

For some reason I thought that cycles were _always_ present instead of starting
after a specific offset so I spent a long time figuring out why my calculations
were not working.

---

One thing I really need to start working on is the use of `ndarray`. I think
[this](https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/) is
extremely helpful, discovered it recently.

---

I also really dislike parsing the input matrix using the rewind method, there
has to be a better way of doing it.
