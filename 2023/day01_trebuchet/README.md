Not really happy with the fact that I traverse each input string 20 times in
part2 but the alternative would have been to implement logic similar to the one
needed for a table-driven lexer so that I can find the first matching digit
(no matter in which form) in one pass for each string. Then instead of 20 passes
I would only need 2 per input string, at the expense of some extra memory for
the transition table. But, ain't nobody got time for that.

---

I wish I would have figured out how to create an array of
[Pattern](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html) so that
I could skip the "clever" index logic in the `find_digit` function. I remember
struggling with trait objects before, I still haven't grokked that concept.
