use log::info;
use nom::{bytes::complete::tag, character::complete, sequence::tuple};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    mana: i32,
    spent_mana: i32,
}

#[derive(Debug, Clone)]
struct Boss {
    hp: i32,
    damage: i32,
}

impl From<&str> for Boss {
    fn from(input: &str) -> Self {
        let (_, (_, hp, _, damage)) = tuple::<_, _, nom::error::Error<_>, _>((
            tag("Hit Points: "),
            complete::i32,
            tag("Damage: "),
            complete::i32,
        ))(input)
        .unwrap();

        Boss { hp, damage }
    }
}

#[derive(Debug, Clone)]
enum Turn {
    PlayerTurn,
    BossTurn,
}

#[derive(Debug, Clone)]
struct State {
    player: Player,
    boss: Boss,
    // This HashMap makes the solution slower but I like it since it is more
    // extensible. Not that aoc problems have that as a goal... just personal
    // preference, I don't like hardcoding things if the effort of doing
    // otherwise is not too big.
    effects: HashMap<Effect, Duration>,
    turn: Turn,
}

impl State {
    fn handle_effects(&mut self) {
        self.effects
            .entry(Effect::Shield)
            .and_modify(|duration| *duration -= 1);
        self.effects.entry(Effect::Poison).and_modify(|duration| {
            *duration -= 1;
            self.boss.hp -= 3;
        });
        self.effects.entry(Effect::Recharge).and_modify(|duration| {
            *duration -= 1;
            self.player.mana += 101;
        });

        self.effects.retain(|_, duration| *duration > 0);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

type Duration = u32;

enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

struct Spell {
    spell_type: SpellType,
    cost: i32,
}

impl Spell {
    fn can_be_cast(&self, state: &State) -> bool {
        match self.spell_type {
            SpellType::MagicMissile | SpellType::Drain =>
                state.player.mana >= self.cost,
            SpellType::Shield =>
                state.player.mana >= self.cost && !state.effects.contains_key(&Effect::Shield),
            SpellType::Poison =>
                state.player.mana >= self.cost && !state.effects.contains_key(&Effect::Poison),
            SpellType::Recharge =>
                state.player.mana >= self.cost && !state.effects.contains_key(&Effect::Recharge),
        }
    }
    fn cast(&self, state: &mut State) {
        state.player.mana -= self.cost;
        state.player.spent_mana += self.cost;
        match self.spell_type {
            SpellType::MagicMissile => {
                state.boss.hp -= 4;
            }
            SpellType::Drain => {
                state.player.hp += 2;
                state.boss.hp -= 2;
            }
            SpellType::Shield => {
                state.effects.insert(Effect::Shield, 6);
            }
            SpellType::Poison => {
                state.effects.insert(Effect::Poison, 6);
            }
            SpellType::Recharge => {
                state.effects.insert(Effect::Recharge, 5);
            }
        }
    }
}

// Note that the spellbook is sorted by cost. This matters when
// exploring the state space (see simulate_turn function).
const SPELLBOOK: [Spell; 5] = [
    Spell {
        spell_type: SpellType::MagicMissile,
        cost: 53,
    },
    Spell {
        spell_type: SpellType::Drain,
        cost: 73,
    },
    Spell {
        spell_type: SpellType::Shield,
        cost: 113,
    },
    Spell {
        spell_type: SpellType::Poison,
        cost: 173,
    },
    Spell {
        spell_type: SpellType::Recharge,
        cost: 229,
    },
];

fn simulate_turn(
    state_queue: &mut VecDeque<State>,
    current_minimum_mana: i32,
    player_turn_life_loss: bool,
) -> i32 {
    let mut state = state_queue.pop_front().unwrap();

    match state.turn {
        Turn::PlayerTurn => {
            if player_turn_life_loss {
                state.player.hp -= 1;
                if state.player.hp <= 0 {
                    return current_minimum_mana;
                }
            }

            state.handle_effects();
            if state.boss.hp <= 0 {
                return state.player.spent_mana;
            }

            // Go through all spells that can be cast (aka there is
            // enough mana and they would not start an effect that is
            // already active)
            for spell in SPELLBOOK.iter().filter(|s| s.can_be_cast(&state)) {
                let mut new_state = state.clone();
                spell.cast(&mut new_state);
                if new_state.boss.hp <= 0 {
                    // If the boss died, return the amount of mana spent to kill him.
                    //
                    // Note that we can return here and not go through the other spells
                    // since the spellbook is sorted by cost (and the filter operation
                    // preserves the order). If we've managed to kill the boss with a
                    // spell then the the rest of the spells in the spellbook are all
                    // of higher cost so it is impossible to kill the boss with those
                    // spells using less mana than this one.
                    return new_state.player.spent_mana;
                } else if new_state.player.spent_mana < current_minimum_mana {
                    // If we've spent more mana than the current minimum then there
                    // is no need to push the new state since it is impossible for it
                    // to generate a state where the boss is killed with less mana.
                    new_state.turn = Turn::BossTurn;
                    state_queue.push_back(new_state);
                }
            }
        }
        Turn::BossTurn => {
            let mut new_state = state;

            new_state.handle_effects();
            if new_state.boss.hp <= 0 {
                return new_state.player.spent_mana;
            }

            // Boss attacks
            if new_state.effects.contains_key(&Effect::Shield) {
                new_state.player.hp -= new_state.boss.damage - 7;
            } else {
                new_state.player.hp -= new_state.boss.damage;
            }

            if new_state.player.hp > 0 {
                new_state.turn = Turn::PlayerTurn;
                state_queue.push_back(new_state);
            }
        }
    }

    current_minimum_mana
}

// Do BFS and keep track of the least amount of mana spent to kill the boss.
// Pass that as a parameter to the simulate_turn function so that we don't
// explore states (aka prune those subtrees) which lead to more mana being
// spent.
fn find_least_mana_to_win(player: Player, boss: Boss, player_turn_life_loss: bool) -> i32 {
    let initial_state = State {
        player,
        boss,
        effects: HashMap::new(),
        turn: Turn::PlayerTurn,
    };
    let mut minimum_mana = i32::MAX;
    let mut states: VecDeque<State> = VecDeque::from([initial_state]);

    loop {
        let spent_mana = simulate_turn(&mut states, minimum_mana, player_turn_life_loss);
        if minimum_mana > spent_mana {
            minimum_mana = spent_mana;
        }

        if states.is_empty() {
            break;
        }
    }

    minimum_mana
}

fn parse_input_and_run(input: &mut impl BufRead, player_turn_life_loss: bool) -> String {
    find_least_mana_to_win(
        Player {
            hp: 50,
            mana: 500,
            spent_mana: 0,
        },
        Boss::from(
            input
                .lines()
                .take(2)
                .map(|line| line.unwrap())
                .collect::<String>()
                .as_str(),
        ),
        player_turn_life_loss,
    )
    .to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    parse_input_and_run(input, false)
}

fn part2(input: &mut impl BufRead) -> String {
    parse_input_and_run(input, true)
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
    fn find_least_mana_to_win_tests() {
        init();

        // Although not explicitly stated, the examples in the problem text
        // show the optimal choice of spells
        assert_eq!(
            find_least_mana_to_win(
                Player {
                    hp: 10,
                    mana: 250,
                    spent_mana: 0,
                },
                Boss { hp: 13, damage: 8 },
                false
            ),
            226
        );

        assert_eq!(
            find_least_mana_to_win(
                Player {
                    hp: 10,
                    mana: 250,
                    spent_mana: 0,
                },
                Boss { hp: 14, damage: 8 },
                false
            ),
            641
        );
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "1269");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "1309");
    }
}
