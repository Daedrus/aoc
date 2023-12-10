The code could use some cleanup but I don't have the energy for it today.

---

Part 2 took a long time to figure out, I kept thinking of solutions using
flood fill and/or doing smart things during the BFS traversal but none of
them led anywhere. I saw someone mentioning the [ray casting
algorithm](https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm)
on reddit so I went with that, and even that got tricky when defining what an
edge is. Figured it out eventually but it took quite a bit.

---

It looks like there were many ways to solve this, one of them being purely
mathematical, which I found quite cool this time:

https://en.wikipedia.org/wiki/Shoelace_formula
https://en.wikipedia.org/wiki/Pick%27s_theorem
