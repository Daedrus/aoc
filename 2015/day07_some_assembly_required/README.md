I can't figure out how to model this problem so that the recursive solution I
had in mind doesn't cause the borrow checker to go bananas.

So the fundamental issue is that I have a data structure which is non
sliceable (a HashMap) and I am trying to modify a value in the data structure
which depends on other values in it. I cannot hold a reference to the value that
I am trying to modify while digging in the data structure for the values it
depends on.

In order for the recursive function to work I have to relinquish all borrows
of `wire_names_to_gates`. So I ended up deriving the Clone trait for Wire
and just cloning the input wires before calling the recursive function. Bleh.

I tried looking at other people's solutions but I couldn't find any that solves
this in a cleaner way. I used this kind of recursive pattern in the 2022 aoc
as well so I'd really like to figure out a way to avoid cloning everything.

I really don't like how cluttered the `compute_signal_value` function ended
up being.

---

A cool pattern that I learned was the fact that you can call `map()` on `IResult`
to convert from `IResult<&str, T>` to `IResult<&str, U>`. The `alt()` function
needs its arguments to be functions that return the same type but various parsers
return various types. For example:
```
separated_pair(
    tuple((
        alt((complete::digit1, complete::alpha1)),
        alt((tag(" AND "), tag(" OR "), tag(" LSHIFT "), tag(" RSHIFT "))),
        alt((complete::digit1, complete::alpha1)),
    )),
    tag(" -> "),
    complete::alpha1)
```
returns `IResult<&str, ((&str, &str, &str), &str)>` while
```
separated_pair(
    preceded(tag("NOT "), complete::alpha1),
    tag(" -> "),
    complete::alpha1)
```
returns `IResult<&str, (&str, &str)>`

Were I to try and use the above two parsers in an `alt()` function, it would
complain that the types are not the same. This is where `map()` comes in handy.

Of course this doesn't work on all parametrized types but at least `Option` and
`Result/IResult` seem to have this.

---

A bit of new syntax is that during a match you can ignore struct fields which
you're not interested in using `..` like this:
```
match self {
    Gate::PassThrough { wire_out, .. }
    | Gate::And { wire_out, .. }
    | Gate::LeftShift { wire_out, .. }
    | Gate::Not { wire_out, .. }
    | Gate::Or { wire_out, .. }
    | Gate::RightShift { wire_out, .. } => {
        wire_out.value = None;
    }
};
```

---

Since I used a function that generates random strings I needed a way to disable
the function when testing so that I can actually test for a specific string.
I am not sure if this is the canonical way of doing this, but it works:
```
fn generate_random_string() -> String {
    #[cfg(not(test))]
    {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>()
    }
    #[cfg(test)]
    {
        "TEST".to_string()
    }
}
```
