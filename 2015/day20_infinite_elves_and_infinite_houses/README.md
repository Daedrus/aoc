My least favourite type of aoc problem, where it's all about the math
manipulation and less about programming concepts. I really don't have much to
say about this but I do wonder if there is a better way to solve it (it is
quite slow). The `divisors` generator is optimized to only go up to sqrt of
the number so that saves some time, but it's nowhere near enough. My gut
feeling says that there is a way to start backwards from the input number and
reach the answer faster.

Another interesting test would have been to compare the time difference between
using a generator and doing it the old fashioned way where you'd create an
array/vector with all the divisors.
