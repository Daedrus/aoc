use regex::Regex;

// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");
// Part 1
// const MAX_TICKS: usize = 24;
const MAX_TICKS: usize = 32;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Factory {
    ore: usize,
    ore_robots: usize,
    ore_robot_ore_cost: usize,
    clay: usize,
    clay_robots: usize,
    clay_robot_ore_cost: usize,
    obsidian: usize,
    obsidian_robots: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geodes: usize,
    geode_robots: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

#[derive(Copy, Clone)]
enum FactoryAction {
    ProduceOreRobot,
    ProduceClayRobot,
    ProduceObsidianRobot,
    ProduceGeodeRobot,
    Idle,
}

fn produce_resources(factory: &mut Factory) {
    factory.ore += factory.ore_robots;
    factory.clay += factory.clay_robots;
    factory.obsidian += factory.obsidian_robots;
    factory.geodes += factory.geode_robots;
    // println!("{} ore-collecting robots collect 1 ore; you now have {} ore.", factory.ore_robots, factory.ore);
    // println!("{} clay-collecting robots collect 1 clay; you now have {} clay.", factory.clay_robots, factory.clay);
    // println!("{} obsidian-collecting robots collect 1 obsidian; you now have {} obsidian.", factory.obsidian_robots, factory.obsidian);
    // println!("{} geode-collecting robots collects 1 geode; you now have {} geode.", factory.geode_robots, factory.geodes);
}

fn simulate_factory(tick: usize, factory: &mut Factory, current_action: Option<FactoryAction>, max_geodes: &mut usize) {
    let mut possible_factory_actions: Vec<FactoryAction> = Vec::new();
    let mut geode_robot_built = false;
    let mut geode_robot_could_have_been_built = false;
    let mut obsidian_robot_could_have_been_built = false;
    let mut clay_robot_could_have_been_built = false;
    let mut ore_robot_could_have_been_built = false;

    if let Some(action) = current_action {
        match action {
            FactoryAction::ProduceGeodeRobot => {
                factory.ore -= factory.geode_robot_ore_cost;
                factory.obsidian -= factory.geode_robot_obsidian_cost;
            },
            FactoryAction::ProduceObsidianRobot => {
                factory.ore -= factory.obsidian_robot_ore_cost;
                factory.clay -= factory.obsidian_robot_clay_cost;
            },
            FactoryAction::ProduceClayRobot => {
                factory.ore -= factory.clay_robot_ore_cost;
            },
            FactoryAction::ProduceOreRobot => {
                factory.ore -= factory.ore_robot_ore_cost;
            },
            FactoryAction::Idle => {
                // If we could have built a robot but decided to idle instead, don't try and build it again
                if factory.ore >= factory.geode_robot_ore_cost && factory.obsidian >= factory.geode_robot_obsidian_cost {
                    geode_robot_could_have_been_built = true;
                }
                if factory.ore >= factory.obsidian_robot_ore_cost && factory.clay >= factory.obsidian_robot_clay_cost {
                    obsidian_robot_could_have_been_built = true;
                }
                if factory.ore >= factory.clay_robot_ore_cost {
                    clay_robot_could_have_been_built = true;
                }
                if factory.ore >= factory.ore_robot_ore_cost {
                    ore_robot_could_have_been_built = true;
                }
            }
        }
    }

    produce_resources(factory);

    if let Some(action) = current_action {
        match action {
            FactoryAction::ProduceGeodeRobot => {
                factory.geode_robots += 1;
            },
            FactoryAction::ProduceObsidianRobot => {
                factory.obsidian_robots += 1;
            },
            FactoryAction::ProduceClayRobot => {
                factory.clay_robots += 1;
            },
            FactoryAction::ProduceOreRobot => {
                factory.ore_robots += 1;
            },
            FactoryAction::Idle => {
            }
        }
    }

    if tick != MAX_TICKS {
        // Don't build anything on the last tick it's useless (this is the only heuristic I came up
        // with myself, the rest are from reading reddit Sadge)
        if tick == MAX_TICKS - 1 {
            possible_factory_actions.push(FactoryAction::Idle);
        } else {
            if factory.ore >= factory.geode_robot_ore_cost && factory.obsidian >= factory.geode_robot_obsidian_cost {
                possible_factory_actions.push(FactoryAction::ProduceGeodeRobot);
                // Always prioritize building a geode robot if possible
                geode_robot_built = true;
            }
            // Don't build obsidian robots when we can produce enough obsidian per turn with the existing
            // ones to pay for a geode robot (only one robot can be produced per turn)
            if factory.obsidian_robots < factory.geode_robot_obsidian_cost &&
                !geode_robot_built &&
                !obsidian_robot_could_have_been_built &&
                factory.ore >= factory.obsidian_robot_ore_cost && factory.clay >= factory.obsidian_robot_clay_cost {
                possible_factory_actions.push(FactoryAction::ProduceObsidianRobot);
            }
            // Don't build clay robots when we can produce enough clay per turn with the existing
            // ones to pay for an obsidian robot (only one robot can be produced per turn)
            if factory.clay_robots < factory.obsidian_robot_clay_cost &&
                !geode_robot_built &&
                !clay_robot_could_have_been_built &&
                factory.ore >= factory.clay_robot_ore_cost {
                possible_factory_actions.push(FactoryAction::ProduceClayRobot);
            }
            // Don't build ore robots when we can produce enough ore per turn with the existing
            // ones to pay for one of any other robot (only one robot can be produced per turn)
            if !(factory.ore_robots >= factory.clay_robot_ore_cost &&
                factory.ore_robots >= factory.obsidian_robot_ore_cost &&
                factory.ore_robots >= factory.geode_robot_ore_cost) &&
                !geode_robot_built &&
                !ore_robot_could_have_been_built &&
                factory.ore >= factory.ore_robot_ore_cost {
                possible_factory_actions.push(FactoryAction::ProduceOreRobot);
            }
            if !geode_robot_built {
                possible_factory_actions.push(FactoryAction::Idle);
            }
        }

        for factory_action in possible_factory_actions {
            simulate_factory(tick + 1, &mut factory.clone(), Some(factory_action), max_geodes);
        }
    }

    if tick == MAX_TICKS {
        if max_geodes < &mut factory.geodes {
            *max_geodes = factory.geodes;
        }
    }
}

fn main() {
    let mut factories: Vec<Factory> = Vec::new();
    let mut max_geodes: usize;
    let blueprint_regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let mut lines = INPUT.lines();

    while let Some(line) = lines.next() {
        let blueprint = blueprint_regex.captures_iter(&line).nth(0).unwrap();
        factories.push(Factory{
            ore: 0,
            ore_robots: 1,
            ore_robot_ore_cost: blueprint[2].parse::<usize>().unwrap(),
            clay: 0,
            clay_robots: 0,
            clay_robot_ore_cost: blueprint[3].parse::<usize>().unwrap(),
            obsidian: 0,
            obsidian_robots: 0,
            obsidian_robot_ore_cost: blueprint[4].parse::<usize>().unwrap(),
            obsidian_robot_clay_cost: blueprint[5].parse::<usize>().unwrap(),
            geodes: 0,
            geode_robots: 0,
            geode_robot_ore_cost: blueprint[6].parse::<usize>().unwrap(),
            geode_robot_obsidian_cost: blueprint[7].parse::<usize>().unwrap(),
        });
    }

    // Part 1
    // let mut total_quality_level: usize = 0;
    // // Assume that you can't build anything in Tick 1 (pass None)
    // for (index, factory) in factories.iter_mut().enumerate() {
    //     max_geodes = 0;
    //     simulate_factory(1, factory, None, &mut max_geodes);
    //     total_quality_level += (index + 1) * max_geodes;
    //     println!("{} {}", index + 1, max_geodes);
    // }

    // println!("{}", total_quality_level);

    let mut multiplied_geodes: usize = 1;
    for factory in factories[..3].iter_mut() {
        max_geodes = 0;
        simulate_factory(1, factory, None, &mut max_geodes);
        multiplied_geodes *= max_geodes;
    }

    println!("{}", multiplied_geodes);
}
