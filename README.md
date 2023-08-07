My Rust solutions for [Advent of Code](https://adventofcode.com/).

I started doing these in 2022 to learn Rust, decided afterwards to go back
and try to do all of them all the way back from 2015.

The solutions are organized in workspaces, one workspace per year.

To run a solution and see its answers, go to a solutions folder and:
```
~/aoc/2015/day01_not_quite_lisp ❯ RUST_LOG=info cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/anfa/aoc/target/debug/day01_not_quite_lisp`
[2023-08-07T18:57:09Z INFO  day01_not_quite_lisp] Part 1 answer: 138
[2023-08-07T18:57:09Z INFO  day01_not_quite_lisp] Part 2 answer: 1771
```

Some solutions have debug printouts, you can see those by changing the
`RUST_LOG` flag:
```
~/aoc/2015/day01_not_quite_lisp ❯ RUST_LOG=debug cargo run
```

To check if a solution passes the tests:
```
~/aoc/2015/day01_not_quite_lisp ❯ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (/home/anfa/aoc/target/debug/deps/day01_not_quite_lisp-462e43bd5ba4d516)

running 3 tests
test tests::part1_tests ... ok
test tests::part2_tests ... ok
test tests::check_answers ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Or, from the root folder:
```
~/aoc ❯ cargo test --bin day01_not_quite_lisp
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (/home/anfa/aoc/target/debug/deps/day01_not_quite_lisp-462e43bd5ba4d516)

running 3 tests
test tests::part1_tests ... ok
test tests::part2_tests ... ok
test tests::check_answers ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

I try to write a README in each solution, describing what I have learned and
my thoughts on the particular problem. Take that stuff with a grain of salt, I
am still learning so I am bound to make mistakes.
