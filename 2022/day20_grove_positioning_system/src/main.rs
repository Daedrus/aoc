// const INPUT: &str = include_str!("../input.example");
// const INPUT: &str = include_str!("../input.mine");
const INPUT: &str = include_str!("../input");
const DECRYPTION_KEY: i64 = 811589153;

fn grove_coordinates_sum(numbers: &Vec<i64>, loops: usize) -> i64 {
    let mut numbers_indices: Vec<u64> = (0..numbers.len() as u64).collect();
    for _ in 0..loops {
        for index in 0..numbers.len() {
            if let Some(position) = numbers_indices.iter().position(|&n| n as usize == index) {
                numbers_indices.remove(position);

                let n = numbers.len() - 1;

                if numbers[index as usize] < 0 {
                    let shift = usize::try_from(-numbers[index as usize]).unwrap() % n;
                    if position >= shift {
                        numbers_indices.rotate_right(shift);
                    } else {
                        numbers_indices.rotate_left(n - shift);
                    }
                }

                if numbers[index as usize] > 0 {
                    let shift = usize::try_from(numbers[index as usize]).unwrap() % n;
                    if position + shift < n {
                        numbers_indices.rotate_left(shift);
                    } else {
                        numbers_indices.rotate_right(position - (position + shift) % n);
                    }
                }

                numbers_indices.insert(position, index as u64);
            }
        }
    }

    let final_order = numbers_indices.iter().map(|n| numbers[*n as usize]).collect::<Vec<i64>>();

    let mut grove_coordinates_sum = 0;
    if let Some(position) = final_order.iter().position(|&n| n as usize == 0) {
        grove_coordinates_sum =
            final_order[(position + 1000) % final_order.len()] +
            final_order[(position + 2000) % final_order.len()] +
            final_order[(position + 3000) % final_order.len()];
        // println!("grove_coordinate 1 {}", final_order[(position + 1000) % final_order.len()]);
        // println!("grove_coordinate 2 {}", final_order[(position + 2000) % final_order.len()]);
        // println!("grove_coordinate 3 {}", final_order[(position + 3000) % final_order.len()]);
        // println!("grove_coordinates_sum 2 {}", grove_coordinates_sum);
    }

    grove_coordinates_sum
}

fn main() {
    let numbers: Vec<i64> = INPUT
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    let decrypted_numbers = numbers
        .iter()
        .map(|n| n * DECRYPTION_KEY)
        .collect::<Vec<i64>>();

    println!("{}", grove_coordinates_sum(&numbers, 1));
    println!("{}", grove_coordinates_sum(&decrypted_numbers, 10));



    // This was my initial solution which managed to give the correct answer for part 1 by accident :(
    // Eventually I gave up and just looked at some of the solutions on reddit and saw the one above

    /*
    let mut indices: Vec<u64> = (0..numbers.len() as u64).collect();

    // println!("====");
    // println!("{:?}", numbers);
    // println!("{:?}", indices);
    // println!("====");

    for index in 0..numbers.len() {
        if let Some(position) = indices.iter().position(|&n| n as usize == index) {
            // println!("{} moves from position {}", numbers[index as usize], position);
            indices.remove(position);
            if position as i64 + numbers[index as usize] <= 0 {
                let number_of_wraps = (position as i64 + numbers[index as usize]).abs() / numbers.len() as i64 + 1;
                indices.insert((position as i64 + numbers[index as usize] - number_of_wraps).rem_euclid(numbers.len() as i64) as usize, index as u64);
            } else if position as i64 + numbers[index as usize] >= numbers.len() as i64 {
                let number_of_wraps = (position as i64 + numbers[index as usize]) / numbers.len() as i64;
                indices.insert((position as i64 + numbers[index as usize] + number_of_wraps).rem_euclid(numbers.len() as i64) as usize, index as u64);
            } else {
                indices.insert((position as i64 + numbers[index as usize]).rem_euclid(numbers.len() as i64) as usize, index as u64);
            }
            // println!("{:?}", indices.iter().map(|n| numbers[*n as usize]).collect::<Vec<i64>>());
            // println!("");
        }
    }

    let final_order = indices.iter().map(|n| numbers[*n as usize]).collect::<Vec<i64>>();
    // println!("{:?}", final_order);

    if let Some(position) = final_order.iter().position(|&n| n as usize == 0) {
        let grove_coordinates_sum =
            final_order[(position + 1000) % final_order.len()] +
            final_order[(position + 2000) % final_order.len()] +
            final_order[(position + 3000) % final_order.len()];
        // println!("grove_coordinate 1 {}", final_order[(position + 1000) % final_order.len()]);
        // println!("grove_coordinate 2 {}", final_order[(position + 2000) % final_order.len()]);
        // println!("grove_coordinate 3 {}", final_order[(position + 3000) % final_order.len()]);
        println!("grove_coordinates_sum {}", grove_coordinates_sum);
    }
    */
}
