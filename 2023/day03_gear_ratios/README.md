This isn't as clean as I would want it to be, mostly because I am trying to
do everything in one pass through the array. But then that collides with my
approach of the two parts being completely independent (including the parsing)
so it's not like I am actually showing off that efficiency gain. Nevertheless,
adding some internal state to the `EngineSchematic` struct and having the
`analyze_schematic` function change that state (for example, to store the
part numbers and the part number neighbour list for each gear) would allow one
to do the parsing + traversal in one go and then just have the `part1` and
`part2` functions read that internal state.

---

I copy/pasted the parsing from a previous solution and did the same silly
"rewind" method to get the size of the grid so that I could parse grids
of different sizes and not have to hardcode the size. The `analyze_neighbours`
function is also copy/pasted and adapted from the same solution.

---

This code pattern is great for expressing "I want to change the value of
this key if it already exists in the map; if it doesn't exist then create the
key and assign it this value":
```
gears_to_part_numbers
    .entry(*gear)
    .and_modify(|part_numbers| part_numbers.push(part_number))
    .or_insert_with(|| vec![part_number]);
```
