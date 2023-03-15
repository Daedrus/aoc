This is one of the problem types where things become super easy when your
language of choice has a dedicated library for the problem. In this case,
I chose the `serde` library for JSON parsing. It made it easy to navigate
the tree and find the numbers.

---

`matches!` is a really nice macro that allows you to shorten things like
this, where you just want to see if an expression matches a specific pattern:
```
match value {
    Value::String(s) if s.eq("red") => true,
    _ => false
}
```

to the much more readable:
```
matches!(value, Value::String(s) if s.eq("red")),
```

The macro was suggested by clippy. Very cool.

---

So the problem involves summing up the numbers in a JSON file except those
numbers which are children of objects that fulfill a specific criteria. I
initially wanted to pass that criteria as a closure, but ran into a problem
which I couldn't really decipher but eventually led me to this:
https://github.com/rust-lang/rust/issues/43520

So I switched to a function pointer and things work as I expect them to.
