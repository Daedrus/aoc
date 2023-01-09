I would not be surprised if there is a way to reverse engineer the md5 algorithm
in order to mathematically find out exactly which data gives out a digest
that starts with a specific prefix. I'd be curious to read about this but not
enough to spend time on researching the problem so brute forcing is the way.

I didn't expect part 2 to be solvable without figuring out some sort of trick
but it wasn't that bad in the end, it just takes a minute longer than part 1.

Another thing I'd be interested in reading about is if there is a guarantee
that there always exists an input which results in a digest starting with a
specific prefix. To be on the safe side I assumed that there isn't so I added
that while loop which ends at usize::MAX. Note that the limit is artificial
since you could generate arbitrarily large numbers as strings and append them
to the secret key, one is not limited to integer data types for this problem.

---

In any case, time for the fun stuff, Rust. It's cool how even simple
implementations such as this one can still be of use as examples for the
language's concepts. In this case, it was the difference between &str and
String (which I considered as types for the parameters of the find_digest_..
function). [This](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
is a great explanation so in this case having the arguments as &str makes
sense since we're not interested in changing them.

But the way more interesting thing is Deref coercion which is explained
[here](https://doc.rust-lang.org/std/string/struct.String.html#deref).
You can see this in action with the secret_key variable being passed to
the find_digest_.. function.

So if a type T implements Deref<Target = OtherType> then passing arguments of
type &T to a function which takes parameters of type &OtherType will work since
T can be Derefed to OtherType.

---

Some good things to know are reflected in this line:
```
let data = secret_key.to_owned() + &number.to_string();
```

The way this works is by overloading the plus operator as described
[here](https://doc.rust-lang.org/rust-by-example/trait/ops.html). We can see
that String does this [here](https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26str%3E-for-String).

Since we're interested in creating a brand new string every time we have to
call either to_owned() or clone() on the secret_key in order to spawn a new
String from it (which well then call add() on &self). There is an explanation
[here](https://stackoverflow.com/questions/22264502/in-rust-what-is-the-difference-between-clone-and-to-owned)
about the differences between the two functions but I'll have to get back to
it since I don't fully understand it at the moment.

---

Another gotcha was that this does not work as expected:
```
        if md5::compute(data).starts_with(b"00000") {
            return Some(number.to_string());
	}
```

The "b" in front of "00000" was suggested by the compiler.

The Digest object returned by the md5::compute function Derefs into a &[u8]
slice when using the starts_with method. The b"00000" is a byte literal which
is a &[u8] slice containing the ASCII values of the character "0". So what
we're comparing is [0, 0, 0, 0, 0] with [48, 48, 48, 48, 48] which will fail.

Using the format! macro on the Digest object and the prefix &str fixes things.

One could also avoid the format! macro and pass a &[0,0,0,0,0] to the starts_with()
function but I think it looks cleaner with &str types.

