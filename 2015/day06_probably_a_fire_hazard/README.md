The main thing I wanted to try out in this solution was parsing using `nom`.
I used regexes exclusively during aoc 2022 since relearning nom would have
cost me some time and my goal was to finish the problems as fast as possible.
I say "relearning" since I used nom in my z80 emulator a while ago. Looking
at it again now it seems that they moved away from the macro-based approach
they were using 5 or 6 years ago and instead encourage their crate users to
use regular functions. I have to say it looks a lot tidier now, it reads a
lot like regular language and the `tuple` function makes it easy to extract
the results of a chain of parsers. I'll most likely stop using regexes
entirely and switch to nom for all aoc problems, parser combinators are just
great.


Connected to the above, converting from a type A to a type B can be neatly
implemented using [the From and Into traits](https://doc.rust-lang.org/stable/rust-by-example/conversion/from_into.html).
This makes for easy conversion from strings in the aoc input files to
whatever datatype I'll need in my solutions. In this case I had Instruction
and LightAction as datatypes and obtaining those from strings is as easy
as writing the code for these:
```
impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
    ...
    }
}

impl From<&str> for LightAction {
    fn from(input: &str) -> Self {
    ...
    }
}
```
And then the conversion is simply:
```
let instruction: Instruction = line.unwrap().as_str().into();

// action here is a previously obtained &str
Instruction {
    light_action: action.into()
    ...
}
```


Another useful trait for my two-valued LightState enum was Not. Since it has
two values which are opposite, implementing the Not trait allowed me to write:
```
// grid[i][j] is of type LightState
grid[i][j] = !grid[i][j];
```
This is Rust's way of handling [operator overloading](https://doc.rust-lang.org/rust-by-example/trait/ops.html).


I wish there was a clean way to iterate through each element of a 2d array, I
tried these two but they both seem like overkill (syntactically):
```
// Gets the number of lights that are turned on
grid.iter().flat_map(|r| r.iter()).filter(|&&l| l == LightState::On).count();

// Sums all the elements in the 2d array
grid.iter().fold(0, |sum, r| sum + r.iter().sum::<usize>());
```
I would have liked something like this:
```
grid.iter_all_elems().filter(|&&l| l == LightState::On).count();
grid.iter_all_elems().sum();
```
I saw that some of the specialized crates for handling multi-dimensional arrays
do offer methods for this but that approach involves using an entire crate (and
specialized datatype) to get access to an iterator. That's just another type of
poison.


A neat thing which I also added a comment about in the part2 function is that
declaring a 2d array with elements of type usize on the stack causes an
overflow after a specific array size. I didn't really test to see where the
threshold is but it was funny to see.
