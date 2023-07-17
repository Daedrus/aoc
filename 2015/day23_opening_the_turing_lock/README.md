Not much of a fan of how verbose the `nom` parsing parts are. I took a look
at [pest](https://pest.rs/) but the separate grammar definitions look even
less maintainable. I should look at other people's solutions and see if I can
find anything cleaner.

The `Index` and `IndexMut` traits are useful for mapping the register names
to the actual struct fields. The `execute_instruction` implementation turns out
much cleaner when using them.
