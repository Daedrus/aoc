Not much to say here, this was perhaps the first mathy problem in 2015, there
are usually a few of these every year from what I've seen. As long as you know
about division and remainder, this should be easy.

My reading comprehension skills are problematic, as usual. I missed the fact
that multiple reindeer can be tied for the lead so my solution didn't match
the test in the problem's text (even though it did give the correct answer for
the inputs).

---

Clippy suggested that I replace this:
```
fn simulate_second(reindeer: &Vec<Reindeer>, second: u32) {
```
with this:
```
fn simulate_second(reindeer: &mut [Reindeer], second: u32) {
```

Which is interesting. I guess there could be multiple things which can provide
Reindeer slices so by chaging the signature like that, it could even work with
those. Not that that is applicable in this case, but cool nevertheless.
