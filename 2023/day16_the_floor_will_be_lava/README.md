I had initially implemented this with a recursive DFS and it worked fine until
I ran `cargo test`. It looks like the stack size for the thread spawned by
`cargo test` is smaller than the stack size of the main thread which is why
`cargo run` was working. [This](https://stackoverflow.com/a/42960702) post
has more details about the issue. I converted to a queue-based BFS and now
everything works well. I still think it is strange that I got a stack overflow,
I was using `Box` everywhere but that didn't fix the issue.

---

`cargo fmt` really formats this in an ugly way, the match statements look
awful. I could rework that part and use some offsets instead, but meh, I like
it when it's explicit like this, even if overly verbose.

---

One small trick to the problem was to understand that coming in from the same
direction more than once in a node is a stop condition since the beam will
just follow the same path again.
