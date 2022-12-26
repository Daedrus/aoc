// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

fn decimal_to_snafu(number: i64) -> String {
    let mut result: String = "".to_string();
    let mut remainder = number;

    // I tried figuring this out on my own but eventually I saw the solution on reddit :(
    while remainder > 0 {
        match remainder % 5 {
            0 => { result.push('0'); remainder = remainder / 5; }
            1 => { result.push('1'); remainder = remainder / 5; }
            2 => { result.push('2'); remainder = remainder / 5; }
            3 => { result.push('='); remainder = remainder / 5; remainder += 1; }
            4 => { result.push('-'); remainder = remainder / 5; remainder += 1; }
            _ => { unreachable!() }
        }

    }

    result.chars().rev().collect::<String>()
}

fn main() {
    let mut lines = INPUT.lines();
    let mut sum: i64 = 0;

    while let Some(line) = lines.next() {
        let mut number: i64 = 0;
        line.chars()
            .zip((0..line.len()).rev())
            .for_each(|(c, i)| {
                match c {
                    '0' | '1' | '2' => {
                        number += 5_i64.pow(i as u32) * c.to_digit(10).unwrap() as i64;
                    }
                    '-' => {
                        number += - 5_i64.pow(i as u32);
                    }
                    '=' => {
                        number += -2 * 5_i64.pow(i as u32);
                    }
                    _ => {
                        unreachable!()
                    }
                }
            });

        sum += number;
    }

    println!("{}", decimal_to_snafu(sum));
}
