use generator::{done, Gn};
use log::{debug, info};
use nom::{bytes::complete::tag, character::complete, sequence::tuple};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

// Macro that defines a generator that generates all permutations
// of two positive integers whose sum is $sum
macro_rules! permutations2 {
    ($sum: ident) => {{
        Gn::new_scoped(move |mut s| {
            for i in 0..=$sum {
                let j = $sum - i;
                s.yield_([i, j]);
            }
            done!();
        })
    }};
}

// Macro that defines a generator that generates all permutations
// of four positive integers whose sum is $sum
macro_rules! permutations4 {
    ($sum: ident) => {{
        Gn::new_scoped(move |mut s| {
            for i in 0..=$sum {
                for j in 0..=$sum - i {
                    for k in 0..=$sum - i - j {
                        let l = $sum - i - j - k;
                        s.yield_([i, j, k, l]);
                    }
                }
            }
            done!();
        })
    }};
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_input(input: &mut impl BufRead) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (_, (_, _, capacity, _, durability, _, flavor, _, texture, _, calories)) =
                tuple::<_, _, nom::error::Error<_>, _>((
                    complete::alpha1,
                    tag(": capacity "),
                    complete::i32,
                    tag(", durability "),
                    complete::i32,
                    tag(", flavor "),
                    complete::i32,
                    tag(", texture "),
                    complete::i32,
                    tag(", calories "),
                    complete::i32,
                ))(line.as_str())
                .unwrap();

            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect()
}

fn calculate_score_and_calories(ingredients: &[Ingredient], amounts: &[i32]) -> Option<(i32, i32)> {
    // The function works for any number of ingredients but the ingredients
    // and amounts slices have to be of the same size
    if ingredients.len() != amounts.len() {
        return None;
    }

    // Taking the example input, this would be:
    //
    // ingredients:
    // [
    //  Ingredient {capacity -1, durability -2, flavor  6, texture  3, calories 8}
    //  Ingredient {capacity  2, durability  3, flavor -2, texture -1, calories 3}
    // ]
    //
    // amounts:
    // [
    //  44,
    //  56
    // ]

    let (mut capacity, mut durability, mut flavor, mut texture, calories) = ingredients
        .iter()
        .zip(amounts.iter())
        // ingredients.zip(amounts):
        // [
        //  (Ingredient {capacity -1, durability -2, flavor  6, texture  3, calories 8}, 44)
        //  (Ingredient {capacity  2, durability  3, flavor -2, texture -1, calories 3}, 56)
        // ]
        //
        .map(|(ingredient, amount)| {
            (
                ingredient.capacity * amount,
                ingredient.durability * amount,
                ingredient.flavor * amount,
                ingredient.texture * amount,
                ingredient.calories * amount,
            )
        })
        // ingredients.zip(amounts).map:
        // [
        //  (-44, -88,  264, 132, 352),
        //  (112, 168, -112, -56, 168)
        // ]
        //
        .fold((0, 0, 0, 0, 0), |acc, score| {
            (
                acc.0 + score.0,
                acc.1 + score.1,
                acc.2 + score.2,
                acc.3 + score.3,
                acc.4 + score.4,
            )
        });
        // ingredients.zip(amounts).map.fold:
        // [
        //  (68, 80, 152, 76, 520)
        // ]

    capacity = capacity.clamp(0, i32::MAX);
    durability = durability.clamp(0, i32::MAX);
    flavor = flavor.clamp(0, i32::MAX);
    texture = texture.clamp(0, i32::MAX);

    Some((capacity * durability * flavor * texture, calories))
}

fn part1(input: &mut impl BufRead, teaspoons: i32) -> String {
    let ingredients = parse_input(input);

    debug!("{:?}", ingredients);

    if ingredients.len() == 2 {
        permutations2!(teaspoons)
            .map(|amounts| {
                let (score, _) = calculate_score_and_calories(&ingredients, &amounts).unwrap();
                score
            })
            .max()
            .unwrap()
            .to_string()
    } else if ingredients.len() == 4 {
        permutations4!(teaspoons)
            .map(|amounts| {
                let (score, _) = calculate_score_and_calories(&ingredients, &amounts).unwrap();
                score
            })
            .max()
            .unwrap()
            .to_string()
    } else {
        unreachable!()
    }
}

fn part2(input: &mut impl BufRead, teaspoons: i32) -> String {
    let ingredients = parse_input(input);

    if ingredients.len() == 2 {
        permutations2!(teaspoons)
            .map(|amounts| calculate_score_and_calories(&ingredients, &amounts).unwrap())
            .filter_map(|(score, calories)| if calories == 500 { Some(score) } else { None })
            .max()
            .unwrap()
            .to_string()
    } else if ingredients.len() == 4 {
        permutations4!(teaspoons)
            .map(|amounts| calculate_score_and_calories(&ingredients, &amounts).unwrap())
            .filter_map(|(score, calories)| if calories == 500 { Some(score) } else { None })
            .max()
            .unwrap()
            .to_string()
    } else {
        unreachable!()
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader, 100));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader, 100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 100), "62842880");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader, 100), "57600000");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 100), "13882464");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 100), "11171160");
    }
}
