use itertools::{iproduct, Itertools};
use log::info;
use nom::{bytes::complete::tag, character::complete, sequence::tuple};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Unit {
    hp: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone)]
struct Item {
    cost: usize,
    damage: i32,
    armor: i32,
}

#[derive(Debug, PartialEq)]
enum FightResult {
    PlayerWins,
    BossWins,
}

struct Shop {
    weapons: [Item; 5],
    armor: [Item; 6],
    rings: [Item; 8],
}

// The trick is to add (0,0,0) Items in order to emulate the lack
// of that particular item in a potential combination. That's why
// we have one such entry for armor (for combinations with no
// armor) and two entries for rings (for combinations with no rings
// or one ring)
const SHOP: Shop = Shop {
    weapons: [
        Item { cost:   8, damage: 4, armor: 0, },
        Item { cost:  10, damage: 5, armor: 0, },
        Item { cost:  25, damage: 6, armor: 0, },
        Item { cost:  40, damage: 7, armor: 0, },
        Item { cost:  74, damage: 8, armor: 0, },
    ],
    armor: [
        Item { cost:   0, damage: 0, armor: 0, },
        Item { cost:  13, damage: 0, armor: 1, },
        Item { cost:  31, damage: 0, armor: 2, },
        Item { cost:  53, damage: 0, armor: 3, },
        Item { cost:  75, damage: 0, armor: 4, },
        Item { cost: 102, damage: 0, armor: 5, },
    ],
    rings: [
        Item { cost:   0, damage: 0, armor: 0, },
        Item { cost:   0, damage: 0, armor: 0, },
        Item { cost:  25, damage: 1, armor: 0, },
        Item { cost:  50, damage: 2, armor: 0, },
        Item { cost: 100, damage: 3, armor: 0, },
        Item { cost:  20, damage: 0, armor: 1, },
        Item { cost:  40, damage: 0, armor: 2, },
        Item { cost:  80, damage: 0, armor: 3, },
    ],
};

impl Unit {
    fn equip(&mut self, (weapon, armor, ring1, ring2): &(&Item, &Item, &Item, &Item)) {
        self.damage += weapon.damage + ring1.damage + ring2.damage;
        self.armor += armor.armor + ring1.armor + ring2.armor;
    }

    fn heal_and_remove_equipment(&mut self) {
        self.hp = 100;
        self.damage = 0;
        self.armor = 0;
    }
}

fn parse_input(input: &mut impl BufRead) -> Unit {
    let boss_stats = input
        .lines()
        .take(3)
        .map(|line| line.unwrap())
        .collect::<String>();

    let (_, (_, hp, _, damage, _, armor)) = tuple::<_, _, nom::error::Error<_>, _>((
        tag("Hit Points: "),
        complete::i32,
        tag("Damage: "),
        complete::i32,
        tag("Armor: "),
        complete::i32,
    ))(boss_stats.as_str())
    .unwrap();

    Unit { hp, damage, armor }
}

// The entire loop could be removed and replaced with some simple division
// but I like the roleplaying aspect of this :)
fn fight(player: &Unit, boss: &Unit) -> FightResult {
    let mut player_hp = player.hp;
    let mut boss_hp = boss.hp;

    loop {
        boss_hp -= (player.damage - boss.armor).clamp(1, i32::MAX);
        if boss_hp <= 0 {
            return FightResult::PlayerWins;
        }
        player_hp -= (boss.damage - player.armor).clamp(1, i32::MAX);
        if player_hp <= 0 {
            return FightResult::BossWins;
        }
    }
}

// Given the boss defined in the input and the fight result
// get the cost of all combination of items which lead to
// that fight result with the input boss
fn cost_of_item_combinations_with_fight_result(
    input: &mut impl BufRead,
    fight_result: FightResult,
) -> Vec<usize> {
    let boss = parse_input(input);
    let mut player = Unit {
        hp: 100,
        damage: 0,
        armor: 0,
    };

    let item_combinations = iproduct!(&SHOP.weapons, &SHOP.armor, &SHOP.rings, &SHOP.rings);

    item_combinations
        .filter(|items| {
            // This could be done in a better way but I like the image of a
            // player fighting a respawning boss over and over and healing
            // and changing equipment between rounds :)
            player.equip(items);
            let result = fight(&player, &boss);
            player.heal_and_remove_equipment();

            result == fight_result
        })
        .map(|(weapon, armor, ring1, ring2)| weapon.cost + armor.cost + ring1.cost + ring2.cost)
        .collect_vec()
}

fn part1(input: &mut impl BufRead) -> String {
    cost_of_item_combinations_with_fight_result(input, FightResult::PlayerWins)
        .iter()
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    cost_of_item_combinations_with_fight_result(input, FightResult::BossWins)
        .iter()
        .max()
        .unwrap()
        .to_string()
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
    fn test_fight() {
        init();

        assert_eq!(
            fight(
                &Unit {
                    hp: 8,
                    damage: 5,
                    armor: 5
                },
                &Unit {
                    hp: 12,
                    damage: 7,
                    armor: 2
                }
            ),
            FightResult::PlayerWins
        );
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "78");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "148");
    }
}
