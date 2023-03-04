A classical aoc scenario where a suboptimal solution for part 1 will punish
you in part 2. I am way too addicted to the programming style shown in the
`look_and_say_slow` function and didn't consider that all of those functions
might have a significant runtime impact. `look_and_say_fast` is just a regular
O(n) traversal that works way faster. Bonus, it came directly from ChatGPT, I
only changed the name of the parameter to `input`.
