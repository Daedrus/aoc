use itertools::Itertools;
use log::info;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

// Given a set of packages, return all of the possible subsets which
// contain group_size packages and whose weights sum up to total_weight
fn get_sets_of_size_and_total_weight(
    packages: &HashSet<usize>,
    group_size: usize,
    total_weight: usize,
) -> Vec<HashSet<usize>> {
    packages
        .iter()
        .combinations(group_size)
        .filter(|c| c.iter().copied().sum::<usize>() == total_weight)
        .map(|v| v.into_iter().copied().collect::<HashSet<usize>>())
        .collect()
}

// Given a set of packages, check if the set can be divided into
// number_of_groups groups which are of size larger than group_size
// and whose weights sum up to total_weight
fn set_can_be_divided_into_groups_of_weight(
    packages: &HashSet<usize>,
    number_of_groups: usize,
    group_size: usize,
    total_weight: usize,
) -> bool {
    for i in group_size..(packages.len() - number_of_groups - 1) {
        let weights_in_group = get_sets_of_size_and_total_weight(packages, i, total_weight);
        if !weights_in_group.is_empty() {
            // If we're trying to divide into two groups and we found some
            // possible solutions for one group then we know for sure that
            // the other group fulfills the weight criteria so we can just
            // return here
            if number_of_groups == 2 {
                return true;
            } else {
                for weight_set in weights_in_group {
                    let remaining_packages: HashSet<usize> =
                        packages.difference(&weight_set).copied().collect();
                    if set_can_be_divided_into_groups_of_weight(
                        &remaining_packages,
                        number_of_groups - 1,
                        group_size,
                        total_weight,
                    ) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn separate_packages_into_groups(input: &mut impl BufRead, number_of_groups: usize) -> String {
    let packages: HashSet<usize> = input
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let group_weight: usize = packages.iter().sum::<usize>() / number_of_groups;
    let mut minimum_quantum_entanglement = usize::MAX;

    // Start by analyzing groups in increasing order of number of packages. We're only
    // interested in the first group having the fewest packages so we start with the
    // first group having one package, then two, then three, and so on, until we find
    // a group which leads to a solution (aka the remaining packages can be divided
    // into groups of the same weight)
    //
    // Since a group has at least one package then we can skip analyzing groups whose
    // size is so big that one of the other groups would be empty (aka reduce the
    // right part of the range). In other words, if we had 5 total packages and we
    // wanted to divide into 4 equal groups, then it makes no sense to analyze group 1
    // sets which contain 3,4 or 5 packages.
    for i in 1..(packages.len() - number_of_groups - 1) {
        let group_1_package_sets: Vec<HashSet<usize>> =
            get_sets_of_size_and_total_weight(&packages, i, group_weight);

        for group_1_package_set in group_1_package_sets {
            let remaining_packages: HashSet<usize> =
                packages.difference(&group_1_package_set).copied().collect();
            // If the remaining packages can be divided into groups of equal weight
            // then the group_1_package_set is a solution. We don't care about how
            // the other groups look like, just that they are of equal weight.
            //
            // If any of the other groups would be of shorter length than group 1
            // _and_ they would lead to a solution then we would have found it in
            // one of the early iterations here so there is no need to look for groups
            // that are shorter in length (therefore the i parameter).
            if set_can_be_divided_into_groups_of_weight(
                &remaining_packages,
                number_of_groups - 1,
                i,
                group_weight,
            ) {
                let quantum_entanglement: usize = group_1_package_set.iter().product();
                if minimum_quantum_entanglement > quantum_entanglement {
                    minimum_quantum_entanglement = quantum_entanglement;
                }
            }
        }

        // If we find a group of size X which is a valid solution then we don't analyze
        // groups of size X + 1 since the minimum solution has to be one of the groups
        // of size X. It's number of packages in the group that should be minimized,
        // the quantum entanglement criteria is there in case there are several solutions
        // with the same number of packages.
        if minimum_quantum_entanglement != usize::MAX {
            break;
        }
    }

    minimum_quantum_entanglement.to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    separate_packages_into_groups(input, 3)
}

fn part2(input: &mut impl BufRead) -> String {
    separate_packages_into_groups(input, 4)
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "11266889531");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "77387711");
    }
}
