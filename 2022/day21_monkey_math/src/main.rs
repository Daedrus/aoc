use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

//const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    ref1: Option<String>,
    ref2: Option<String>,
    op: Option<Operation>,
    value: Option<i64>,
}

fn get_monkey_yell(monkey_name: String, monkeys: &HashMap<String, Monkey>) -> i64 {
    if let Some(monkey) = monkeys.get(&monkey_name) {
        if let Some(value) = monkey.value {
            value
        } else {
            if let Some(op) = monkey.op {
                // I don't understand this as_ref() which the compiler suggested here
                let monkey1_value = get_monkey_yell(monkey.ref1.as_ref().unwrap().to_string(), &monkeys);
                let monkey2_value = get_monkey_yell(monkey.ref2.as_ref().unwrap().to_string(), &monkeys);
                match op {
                    Operation::Add => {
                        monkey1_value + monkey2_value
                    }
                    Operation::Subtract => {
                        monkey1_value - monkey2_value
                    }
                    Operation::Divide => {
                        monkey1_value / monkey2_value
                    }
                    Operation::Multiply => {
                        monkey1_value * monkey2_value
                    }
                }
            } else {
                unreachable!();
            }
        }
    } else {
        unreachable!();
    }
}

fn postfix_expression(monkey_name: String, monkeys: &HashMap<String, Monkey>, stack: &mut VecDeque<String>) {
    if monkey_name == "humn" {
        stack.push_back("humn".to_string());
        return;
    }
    if let Some(monkey) = monkeys.get(&monkey_name) {
        if let Some(value) = monkey.value {
            stack.push_back(value.to_string());
        } else {
            if let Some(op) = monkey.op {
                postfix_expression(monkey.ref2.as_ref().unwrap().to_string(), &monkeys, stack);
                postfix_expression(monkey.ref1.as_ref().unwrap().to_string(), &monkeys, stack);
                match op {
                    Operation::Add => {
                        stack.push_back("+".to_string());
                    }
                    Operation::Subtract => {
                        stack.push_back("-".to_string());
                    }
                    Operation::Divide => {
                        stack.push_back("/".to_string());
                    }
                    Operation::Multiply => {
                        stack.push_back("*".to_string());
                    }
                }
            } else {
                unreachable!();
            }
        }
    } else {
        unreachable!();
    }
}

fn solve_postfix_expression(stack: &mut VecDeque<String>) -> i64 {
    let mut aux_stack: VecDeque<String> = VecDeque::new();
    while !stack.is_empty() {
        let val = stack.pop_front().unwrap();
        if string_is_number(&val) {
            aux_stack.push_front(val);
            continue;
        } else {
            let val1:i64 = aux_stack.pop_front().unwrap().parse().unwrap();
            let val2:i64 = aux_stack.pop_front().unwrap().parse().unwrap();
            if val == "+".to_string() {
                aux_stack.push_front((val1 + val2).to_string());
            }
            if val == "-".to_string() {
                aux_stack.push_front((val1 - val2).to_string());
            }
            if val == "/".to_string() {
                aux_stack.push_front((val1 / val2).to_string());
            }
            if val == "*".to_string() {
                aux_stack.push_front((val1 * val2).to_string());
            }
        }
    }

    aux_stack[0].parse().unwrap()
}

fn string_is_number(string: &String) -> bool {
    match &string.parse::<i64>() {
        Ok(_) => true,
        Err(_) => false
    }
}

fn solve_postfix_expression_with_humn(stack: &mut VecDeque<String>, val: i64) {
    // println!("OFFFFF");
    // println!("{:?}", stack);

    let mut aux_stack: VecDeque<String> = VecDeque::new();
    while !stack.is_empty() {
        let val = stack.pop_front().unwrap();
        // println!("HERE {}", val);
        if string_is_number(&val) {
            aux_stack.push_front(val);
            // println!("{:?} -- {:?}", stack, aux_stack);
            continue;
        } else {
            if val == "humn".to_string() {
                aux_stack.push_front(val);
                // println!("{:?}", stack);
                // println!("{:?}", aux_stack);
                break;
            }
            let val1:i64 = aux_stack.pop_front().unwrap().parse().unwrap();
            let val2:i64 = aux_stack.pop_front().unwrap().parse().unwrap();
            if val == "+".to_string() {
                aux_stack.push_front((val1 + val2).to_string());
            }
            if val == "-".to_string() {
                aux_stack.push_front((val1 - val2).to_string());
            }
            if val == "/".to_string() {
                aux_stack.push_front((val1 / val2).to_string());
            }
            if val == "*".to_string() {
                aux_stack.push_front((val1 * val2).to_string());
            }
        }
    }

    // aux_stack.push_back(val.to_string());

    // println!("====");
    // println!("{:?}", stack);
    // println!("{:?}", aux_stack);

    while !stack.is_empty() {
        let val = stack.pop_front().unwrap();
        if string_is_number(&val) {
            aux_stack.push_front(val);
            continue;
        } else {
            let val1 = aux_stack.pop_front().unwrap();
            let val2 = aux_stack.pop_front().unwrap();
            if val == "+".to_string() {
                aux_stack.push_front("(".to_owned() + &val1 + &"+".to_owned() + &val2 + &")".to_owned());
            }
            if val == "-".to_string() {
                aux_stack.push_front("(".to_owned() + &val1 + &"-".to_owned() + &val2 + &")".to_owned());
            }
            if val == "/".to_string() {
                aux_stack.push_front("(".to_owned() + &val1 + &"/".to_owned() + &val2 + &")".to_owned());
            }
            if val == "*".to_string() {
                aux_stack.push_front("(".to_owned() + &val1 + &"*".to_owned() + &val2 + &")".to_owned());
            }
        }
        // println!("{:?}", aux_stack);
    }

    /*
    // Reverse the operations in stack
    // Create a new stack since I have no idea how to do it in place :(
    let mut reverse_operation_stack: VecDeque<String> = VecDeque::new();
    let mut i = stack.len() - 1;
    while i >= 0 {
        if stack[i] == "+".to_string() {
            if i > 0 && (stack[i-1] == "+".to_string() ||
                         stack[i-1] == '-'.to_string() ||
                         stack[i-1] == '*'.to_string() ||
                         stack[i-1] == '/'.to_string()) {
                reverse_operation_stack.push_back('-'.to_string());
            } else if i > 0 && string_is_number(&stack[i-1]) {
                reverse_operation_stack.push_back(stack[i-1].clone());
                reverse_operation_stack.push_back('-'.to_string());
                i -= 1;
            } else {
                reverse_operation_stack.push_back('-'.to_string());
            }
        } else if stack[i] == "-".to_string() {
            if i > 0 && (stack[i-1] == "+".to_string() ||
                         stack[i-1] == '-'.to_string() ||
                         stack[i-1] == '*'.to_string() ||
                         stack[i-1] == '/'.to_string()) {
                reverse_operation_stack.push_back('+'.to_string());
            } else if i > 0 && string_is_number(&stack[i-1]) {
                reverse_operation_stack.push_back(stack[i-1].clone());
                reverse_operation_stack.push_back('+'.to_string());
                i -= 1;
            } else {
                reverse_operation_stack.push_back('+'.to_string());
            }
        } else if stack[i] == "/".to_string() {
            if i > 0 && (stack[i-1] == "+".to_string() ||
                         stack[i-1] == '-'.to_string() ||
                         stack[i-1] == '*'.to_string() ||
                         stack[i-1] == '/'.to_string()) {
                reverse_operation_stack.push_back('*'.to_string());
            } else if i > 0 && string_is_number(&stack[i-1]) {
                reverse_operation_stack.push_back(stack[i-1].clone());
                reverse_operation_stack.push_back('*'.to_string());
                i -= 1;
            } else {
                reverse_operation_stack.push_back('*'.to_string());
            }
        } else if stack[i] == "*".to_string() {
            if i > 0 && (stack[i-1] == "+".to_string() ||
                         stack[i-1] == '-'.to_string() ||
                         stack[i-1] == '*'.to_string() ||
                         stack[i-1] == '/'.to_string()) {
                reverse_operation_stack.push_back('/'.to_string());
            } else if i > 0 && string_is_number(&stack[i-1]) {
                reverse_operation_stack.push_back(stack[i-1].clone());
                reverse_operation_stack.push_back('/'.to_string());
                i -= 1;
            } else {
                reverse_operation_stack.push_back('/'.to_string());
            }
        }

        if i > 0 {
            i -= 1;
        } else if i == 0 {
            break;
        }
    }

    println!("==== WE START HERE ====");
    println!("{:?}", reverse_operation_stack);
    println!("{:?}", aux_stack);

    while !reverse_operation_stack.is_empty() {
        let val = reverse_operation_stack.pop_front().unwrap();
        println!("====");
        println!("{:?}", reverse_operation_stack);
        println!("{:?}", aux_stack);
        if string_is_number(&val) {
            aux_stack.push_back(val);
            continue;
        } else {
            let val1:i64 = aux_stack.pop_back().unwrap().parse().unwrap();
            let val2:i64 = aux_stack.pop_back().unwrap().parse().unwrap();
            if val == "+".to_string() {
                aux_stack.push_back((val1 + val2).to_string());
            }
            if val == "-".to_string() {
                aux_stack.push_back((val2 - val1).to_string());
            }
            if val == "/".to_string() {
                aux_stack.push_back((val2 / val1).to_string());
            }
            if val == "*".to_string() {
                aux_stack.push_back((val1 * val2).to_string());
            }
        }
    }

    println!("====");
    println!("{:?}", reverse_operation_stack);
    println!("{:?}", aux_stack);
    */

    // OK, so I initially tried to solve it using postfix notation expressions and messing
    // around with the stacks. As you can see above, it did not work. So eventually I gave
    // up and copy/pasted the equation into an online solver :(
    println!("{:?}", aux_stack);
    println!("{:?}", val);
}

fn main() {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let monkey_op_regex = Regex::new(r"([a-z]{4}): ([a-z]{4}) ([+|*|\-|/]) ([a-z]{4})").unwrap();
    let monkey_val_regex = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
    let mut lines = INPUT.lines();

    while let Some(line) = lines.next() {
        if monkey_val_regex.is_match(line) {
            let monkey = monkey_val_regex.captures_iter(&line).nth(0).unwrap();
            monkeys.insert(
                monkey[1].to_string(),
                Monkey {
                    ref1: None,
                    ref2: None,
                    op: None,
                    value: Some(monkey[2].parse::<i64>().unwrap()),
                }
            );
        } else if monkey_op_regex.is_match(line) {
            let monkey = monkey_op_regex.captures_iter(&line).nth(0).unwrap();
            monkeys.insert(
                monkey[1].to_string(),
                Monkey {
                    ref1: Some(monkey[2].to_string()),
                    ref2: Some(monkey[4].to_string()),
                    op: match &monkey[3] {
                            "+" => { Some(Operation::Add) },
                            "-" => { Some(Operation::Subtract) },
                            "/" => { Some(Operation::Divide) },
                            "*" => { Some(Operation::Multiply) },
                            _ => { unreachable!() }
                        },
                    value: None,
                }
            );
        }
    }

    // Part 1
    println!("{}", get_monkey_yell("root".to_string(), &monkeys));

    // Assume that root always exists and is an operation
    if let Some(monkey) = monkeys.get(&"root".to_string()) {
        let mut left_postfix_expression: VecDeque<String> = VecDeque::new();
        let mut right_postfix_expression: VecDeque<String> = VecDeque::new();
        postfix_expression(monkey.ref1.as_ref().unwrap().to_string(), &monkeys, &mut left_postfix_expression);
        postfix_expression(monkey.ref2.as_ref().unwrap().to_string(), &monkeys, &mut right_postfix_expression);

        // Assume that only one of the expression contains humn, and it only appears once in that expression
        if left_postfix_expression.contains(&"humn".to_string()) {
            let expression_value = solve_postfix_expression(&mut right_postfix_expression);
            solve_postfix_expression_with_humn(&mut left_postfix_expression, expression_value);
        } else {
            let expression_value = solve_postfix_expression(&mut left_postfix_expression);
            solve_postfix_expression_with_humn(&mut right_postfix_expression, expression_value);
        }

        // println!("{:?}", left_postfix_expression);
        // println!("{:?}", right_postfix_expression);
    }
}
