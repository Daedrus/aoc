This is the first aoc problem where I had no idea what it actually wanted me
to do. I am referring to this text in part 2:
_Turns out the shopkeeper is working with the boss, and can persuade you to buy
whatever items he wants. The other rules still apply, and he still only has one
of each item._

I had to look at other people's solutions and see what they were doing. And
that turns out to be... nothing? I still can't make sense of what the first
sentence implies, the item combinations are the same in both parts.

---

The thing to notice here is that `itertools`' `iproduct!` macro does all the
work. For a small input set such as the shop items defined by the problem
generating all of the possible combinations of items is feasible.

The trick of adding dud items to the shop makes the code easier to write.
Without that we would have had to call `iproduct!` three times:
```
iproduct!(&SHOP.weapons, &SHOP.armor);
iproduct!(&SHOP.weapons, &SHOP.armor, &SHOP.rings);
iproduct!(&SHOP.weapons, &SHOP.armor, &SHOP.rings, &SHOP.rings);
```

And then chain the resulting iterators.
