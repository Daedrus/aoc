use log::{debug, info};
use nom::{bytes::complete::tag, character::complete, sequence::tuple};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
    points: u32,
    current_position: u32,
}

fn parse_input(input: &mut impl BufRead) -> Vec<Reindeer> {
    let mut reindeer: Vec<Reindeer> = Vec::new();

    input.lines().for_each(|line| {
        let line = line.unwrap();

        let (_, (_, _, speed, _, flight_duration, _, rest_duration, _)) =
            tuple::<_, _, nom::error::Error<_>, _>((
                complete::alpha1,
                tag(" can fly "),
                complete::u32,
                tag(" km/s for "),
                complete::u32,
                tag(" seconds, but then must rest for "),
                complete::u32,
                tag(" seconds."),
            ))(line.as_str())
            .unwrap();

        reindeer.push(Reindeer {
            speed,
            flight_duration,
            rest_duration,
            points: 0,
            current_position: 0,
        });
    });

    debug!("{:?}", reindeer);

    reindeer
}

fn simulate_second(reindeer: &mut [Reindeer], second: u32) {
    let mut max_distance = 0;

    reindeer.iter_mut().for_each(|reindeer| {
        //               full sprint                       full sprint
        // ...<--------------------------------><-------------------------------->...
        // ...<---------------><---------------><---------------><--------------->...
        //     flight_duration   rest_duration   flight_duration   rest_duration
        let full_sprints = second / (reindeer.flight_duration + reindeer.rest_duration);

        //                                            /-- seconds_in_last_sprint
        //                                            |
        //               full sprint                  v     last sprint
        // ...<--------------------------------><----------->
        // ...<---------------><---------------><---------------><--------------->
        //     flight_duration   rest_duration   flight_duration   rest_duration
        //
        //                                            /-- seconds_in_last_sprint
        //                                            |
        //               full sprint                  v     last sprint
        // ...<--------------------------------><-------------------->
        // ...<---------------><---------------><---------------><--------------->
        //     flight_duration   rest_duration   flight_duration   rest_duration
        let seconds_in_last_sprint = second % (reindeer.flight_duration + reindeer.rest_duration);

        // Keep in mind that distance is covered only during flight_duration
        reindeer.current_position = if seconds_in_last_sprint < reindeer.flight_duration {
            (full_sprints * reindeer.flight_duration + seconds_in_last_sprint) * reindeer.speed
        } else {
            (full_sprints * reindeer.flight_duration + reindeer.flight_duration) * reindeer.speed
        };

        if reindeer.current_position > max_distance {
            max_distance = reindeer.current_position;
        }
    });

    // Multiple reindeer can be tied for the lead
    reindeer
        .iter_mut()
        .filter(|reindeer| reindeer.current_position == max_distance)
        .for_each(|reindeer| reindeer.points += 1);
}

fn part1(input: &mut impl BufRead, duration: u32) -> String {
    let mut reindeer = parse_input(input);

    simulate_second(&mut reindeer, duration);

    reindeer
        .iter()
        .max_by_key(|reindeer| reindeer.current_position)
        .unwrap()
        .current_position
        .to_string()
}

fn part2(input: &mut impl BufRead, duration: u32) -> String {
    let mut reindeer = parse_input(input);

    (1..=duration).for_each(|second| {
        simulate_second(&mut reindeer, second);
    });

    reindeer
        .iter()
        .max_by_key(|reindeer| reindeer.points)
        .unwrap()
        .points
        .to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader, 2503));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader, 2503));

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

        assert_eq!(part1(&mut reader, 1), "16");
        reader.rewind().unwrap();
        assert_eq!(part1(&mut reader, 10), "160");
        reader.rewind().unwrap();
        assert_eq!(part1(&mut reader, 11), "176");
        reader.rewind().unwrap();
        assert_eq!(part1(&mut reader, 12), "176");
        reader.rewind().unwrap();
        assert_eq!(part1(&mut reader, 1000), "1120");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader, 1), "1");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 140), "139");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 1000), "689");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 2503), "2696");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 2503), "1084");
    }
}
