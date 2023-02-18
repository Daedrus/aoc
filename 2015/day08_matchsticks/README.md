Struggled with the parser a bit due to all of the character escaping. Also,
treating the double quotes from the start and end separately is a bit ugly,
but it works.

I also don't understand why I am allowed to mutably borrow the Vec produced
by the `delimited` parser when mapping the produced `IResult`:
```
    delimited(
        ...
    )(input)
    .map(|(s, mut v)| {
        // Manually add the beginning and end double quotes
        v.push(StringFragment {
            characters_of_code: 2,
            characters_in_memory: 0,
            characters_in_encoded: 6,
        });
        (s, v)
    })
```

---

The big thing which I only accidentally discovered (since it worked without
me realizing what I had done) was that I was able to call `map()` on a
parser function, _not_ on a parser function's result (as I have done so far):
```
    // This map() acts on the alpha1 parser
    alpha1.map(|s: &str| StringFragment {
        characters_of_code: s.len(),
        characters_in_memory: s.len(),
        characters_in_encoded: s.len(),
    }),

    // This map() acts on the IResult coming out of the delimited parser
    delimited(
        ...
    )(input)
    .map(|(s, mut v)| {
        ...
    })
```

I wanted to express \"convert the output of this function from (&str, &str) to
(&str, StringFragment)\" just as I had done before so I wrote the `map()`s
without thinking too much. I noticed that the arguments to `map()` were finicky
and it did force me to be explicit about the type in the above example but I
just chalked that up to the compiler being weird. Only to later realize that
these `map()`s were acting on the functions themselves, not their outputs.

I have no clue how and why this works, I tried looking at the `nom` code but
I can't make any sense of it at the moment.

---

A final small thing is that I don't know if it's better to place the `nom`
imports at the top of the file or add them in the function that uses `nom`
(like I have done so far).

