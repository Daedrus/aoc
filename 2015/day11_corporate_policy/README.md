Can't say I'm too happy with how this turned out, way too much dereferencing
for my taste, I think it just looks ugly. Could have probably fixed it with
some smart trait defintions for the primitive char type.

The ungodly amounts of calls to `unwrap()` that I am doing surely can't be a
good pattern, but I also don't see the need for proper error handling in the
aoc problems.

---

One situation which I encountered was that I wanted to for_each/fold/map
until a specific condition occured, at which point I wanted to break out
of said for_each/fold/map. It turns out that there exist "try" variants
of these functions which support breaking out using the `ControlFlow`
enum:
```
password.iter_mut().rev().try_for_each(|c| {
    if *c == 'z' {
        *c = 'a';
        ControlFlow::Continue(())
    } else {
        *c = std::char::from_u32(*c as u32 + 1).unwrap();
        ControlFlow::Break(c)
    }
});
```

---

The `TryFrom` trait was good for my `Password` type in cases where the
`&str` input was not 8 characters long. Didn't really have an use for this
in the solution, but I want to do it since I had never used the trait before.

`TryFrom` is used for type conversions that may fail. Had I simply implemented
the `Try` trait then I would have had no way of handling input that was not
8 characters long.

---

Implementing the `Display` trait for a type gives you the `to_string()` method
for free. This is sure to come in handy in the future.

