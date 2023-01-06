I'll put in random thoughts every now and then in README files such as this,
as a way to reflect on what I have learned and to write down any outstanding
questions that I might have.

I wanted to have two entry points/functions, one for each part. They should
accept both strings (so that I can write quick tests, if possible) and files as
input. The solution I found for this was to have the functions accept types
which implement the BufRead trait. What I don't like about this is that I have
to wrap strings in a Cursor before passing them to the part functions. It looks
fugly, I wish there was a cleaner way I could handle both strings and files and
have access to the lines() method.

I wanted a way to enable/disable println!s since I found myself constantly
commenting and uncommenting them when I was writing code for aoc 2022. I saw
some macro-based solutions for this online but they felt hacky so I just chose
a logging crate which seemed lightweight (env\_logger). I am not particularly
fond of the init() function it made me write and use in the tests mod though.
I might change the info!s back to println!s since right now `cargo run` doesn't
show anything which might be confusing for someone just wanting to run the
project to see what happens.

Here are the commands that I am currently using to run this, I might have to
create a makefile for these at some point.
* `RUST_LOG=info cargo run` shows the info messages with the results for part
1 and part 2
* `RUST_LOG=debug cargo run` shows both the debug and info messages
* `cargo test` runs all tests
* `cargo test part1_tests` runs the part1 tests
* `RUST_LOG=debug cargo test part1_tests -- --nocapture` runs the part1 tests and shows the debug messages

I experimented a bit with the [bench] feature on rust nightly but decided
against it after a while, I'd like to keep these solutions on stable. I'll
definitely use the feature if it ever makes it into stable. I looked at some
other crates for benchmarking but they seem like too much of a hassle and I'd
like to keep things clean.

Finally, this piece of code:
```
        .map(|line| {
            let line = line.unwrap();
            let mut gift = line.split('x').map(|d| d.parse::<usize>().unwrap());
```

was originally:
```
        .map(|line| {
            let mut gift = line.unwrap().split('x').map(|d| d.parse::<usize>().unwrap());
```

but that didn't work, compilation fails with:

```
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:18:28
   |
18 |             let mut gift = line.unwrap().split('x').map(|d| d.parse::<usize>().unwrap());
   |                            ^^^^^^^^^^^^^                                                - temporary value is freed at the end of this statement
   |                            |
   |                            creates a temporary which is freed while still in use
19 |             Gift {
20 |                 length: gift.next().unwrap(),
   |                         ----------- borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
error: could not compile `day02_i_was_told_there_would_be_no_math` due to previous error
```

I still haven't wrapped my head around why I had to do this, I'll append an
explanation to this README once I figure it out. I blindly followed the
compiler instructions during aoc 2022, it's not a good habit.
