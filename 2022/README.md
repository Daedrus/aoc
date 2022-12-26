This is a dump of all the code that I wrote for Advent of Code 2022. It is unprocessed,
aka in the same form it was after I submitted my answers. I've made no attempt at
cleaning it up, although I am well aware of how much a cleanup is needed. On the other
hand I find it interesting as a snapshot of my current skills when it comes to
"programming against a timer in a language I am not that familiar with when programming
hasn't even been my main job for many years".

2022 was the first year I made an attempt at Advent of Code. The stars aligned and I
realized I had enough time on a daily basis to allocate to it + I wanted to pick up
Rust again. The target was to finish each problem the day it got posted which meant
that outside the weekends, I would work on the problems after getting home from work.
Day 15's Part 2 was the only problem that I couldn't manage in the self-imposed time
limit. Day 16 and Day 24 were skipped entirely mostly due to lack of time but also
since optimization problems by default would take me much longer to grok since I
never work with them IRL. Day 19 was such an optimization problem and I just decided
to brute-force it and of course the state space got too big and I had to go to reddit
for some tips on how to prune it.

That being said, a few stars are most likely undeserved, day 19, 20 and 25 all had me
go to reddit and read the solutions thread for tips. This is my final result:

          --------Part 1--------   --------Part 2--------
    Day       Time   Rank  Score       Time   Rank  Score
     25   15:29:23  10633      0          -      -      -
     24          -      -      0          -      -      0
     23   11:01:09   8538      0   11:03:46   8212      0
     22   14:55:40  11036      0   18:20:52   6345      0
     21   14:00:57  15248      0   18:01:11  13676      0
     20   15:13:05  11354      0   17:13:17  11282      0
     19   17:49:07   7867      0   17:55:49   6743      0
     18   03:51:07   7431      0   04:59:30   5841      0
     17   06:09:42   6283      0   13:11:09   6308      0
     16          -      -      0          -      -      0
     15   13:28:06  21885      0       >24h  24970      0
     14   13:33:13  24069      0   13:47:31  22776      0
     13   15:25:02  26179      0   15:44:01  25128      0
     12   14:28:57  26995      0   14:53:17  26374      0
     11   07:38:29  25013      0   12:40:07  27241      0
     10   02:18:03  13987      0   02:42:55  12408      0
      9   15:07:08  46478      0   18:57:24  41921      0
      8   14:04:02  55037      0   14:29:11  48287      0
      7   16:08:56  54107      0   16:20:33  52149      0
      6   00:38:24  15843      0   00:40:39  15017      0
      5   15:28:34  73610      0   15:29:19  70969      0
      4   03:37:14  29441      0   03:39:58  28114      0
      3   03:31:51  28415      0   04:20:34  29217      0
      2   04:06:59  41969      0   04:14:08  38004      0
      1   05:55:41  45946      0   06:05:33  43763      0

Other thoughts:

- I dislike contests as these since they unwillingly promote messy code and ugly hacks.
Maintainability, extensibility, ease-of-testing, documentation and other such IRL
things all take a step back to make room for a quick working solution. I'd love to see
a bit more focus on handling malformed inputs, test coverage, passing linter checks, etc.
- I also wish that there existed input that tested the limits of one's solution. Space
and time limits would be interesting but also large input files or even streams, to
make sure that the data processing is done in an efficient way (I am a bit disappointed
at the include\_str! solution that many people use in Rust and that I eventually adopted).
- It is a great way to learn a new language since it inevitably exposes you to all of
a language's basic building blocks. Not only that, but reading other people's code is a
great way to learn new coding constructs / patterns in your language of choice. I saw a
lot of cool Rust stuff by looking at other people's solutions.
- The most common bugs that I had were off-by-one errors and bugs related to reading
comprehension.
- The memes on r/adventofcode were (for me) a big reason for why this was so fun.
- I had so much fun doing this that I will probably start all the way back from 2015 and
try and solve everything without the pressure of time, focusing on finding the best Rust
constructs for various situations and taking my sweet time to learn any new algorithms
that I need (especially optimization and graph related ones).
- Really looking forward to next year's AoC, I'll see what other people are doing when
it comes to automatic submission and input handling and set up something similar.
