My first thought reading this was [Floyd-Warshall](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm).
But since this is a discrete matrix then the distances between the galaxies can
be computed using a simple [Taxicab/Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
Now, the crucial piece of information that makes this work is that "the
shortest path between two galaxies is allowed to pass through another galaxy".
Without this, the taxicab distance _does not_ work. I did not read that part
initially so I had dismissed this solution, instead looking into how to
implement [A\*](https://en.wikipedia.org/wiki/A*_search_algorithm).

I remember seeing on reddit the expression "Advent of reading comprehension"
and this is the perfect example of that. The constraint did eventually register
with me so I went back to taxicab but I had spent quite a bit of time to reach
that point.

---

Regarding the implementation itself, there's nothing special, this was just
another one of those mathy problems and I lucked out on part 2 being easy
purely because I didn't choose to go for a graph implementation in part 1. Meh.
