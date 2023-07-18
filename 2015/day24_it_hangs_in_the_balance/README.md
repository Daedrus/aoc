This probably has a better solution instead of just trying out the
combinations like I did. I did try to prune the search as much as I could
(see the comments in the solution) but it's still most likely suboptimal.
I really wish there existed reference pseudocode implementations with
the optimal solution.

I could have probably included the group 1 handling in the
`set_can_be_divided_into_groups_of_weight` function and return the quantum
entanglement from there somehow (aka treat all groups equally) but in the
end I decided that with this solution it is more obvious that group 1 is
special.

---

From a programming perspective `copied()` was new, it allows one to create
an iterator over T from an iterator over &T.
