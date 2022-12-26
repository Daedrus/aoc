use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Reverse;

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    items: Rc<RefCell<Vec<u64>>>,
    operation: fn(u64) -> u64,
    divisor: u64,
    test_true: usize,
    test_false: usize,
    inspections: Rc<RefCell<u64>>,
}


fn main() {
    // Hardcoded inputs since it would take too long to implement the parser
    // I wonder if Rust has some sort of eval that could return the "operation" lambdas below
    /*
    let input: Vec<Monkey> = vec![
        // Monkey 0
        Monkey {
            items: Rc::new(RefCell::new(vec![79, 98])),
            operation: |item| item.checked_mul(19).unwrap(),
            divisor: 23,
            test_true: 2,
            test_false: 3,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 1
        Monkey {
            items: Rc::new(RefCell::new(vec![54, 65, 75, 74])),
            operation: |item| item.checked_add(6).unwrap(),
            divisor: 19,
            test_true: 2,
            test_false: 0,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 2
        Monkey {
            items: Rc::new(RefCell::new(vec![79, 60, 97])),
            operation: |item| item.checked_mul(item).unwrap(),
            divisor: 13,
            test_true: 1,
            test_false: 3,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 3
        Monkey {
            items: Rc::new(RefCell::new(vec![74])),
            operation: |item| item.checked_add(3).unwrap(),
            divisor: 17,
            test_true: 0,
            test_false: 1,
            inspections: Rc::new(RefCell::new(0)),
        },
    ];
    */
    let input: Vec<Monkey> = vec![
        // Monkey 0
        Monkey {
            items: Rc::new(RefCell::new(vec![66, 79])),
            operation: |item| item * 11,
            divisor: 7,
            test_true: 6,
            test_false: 7,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 1
        Monkey {
            items: Rc::new(RefCell::new(vec![84, 94, 94, 81, 98, 75])),
            operation: |item| item * 17,
            divisor: 13,
            test_true: 5,
            test_false: 2,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 2
        Monkey {
            items: Rc::new(RefCell::new(vec![85, 79, 59, 64, 79, 95, 67])),
            operation: |item| item + 8,
            divisor: 5,
            test_true: 4,
            test_false: 5,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 3
        Monkey {
            items: Rc::new(RefCell::new(vec![70])),
            operation: |item| item + 3,
            divisor: 19,
            test_true: 6,
            test_false: 0,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 4
        Monkey {
            items: Rc::new(RefCell::new(vec![57, 69, 78, 78])),
            operation: |item| item + 4,
            divisor: 2,
            test_true: 0,
            test_false: 3,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 5
        Monkey {
            items: Rc::new(RefCell::new(vec![65, 92, 60, 74, 72])),
            operation: |item| item + 7,
            divisor: 11,
            test_true: 3,
            test_false: 4,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 6
        Monkey {
            items: Rc::new(RefCell::new(vec![77, 91, 91])),
            operation: |item| item * item,
            divisor: 17,
            test_true: 1,
            test_false: 7,
            inspections: Rc::new(RefCell::new(0)),
        },
        // Monkey 7
        Monkey {
            items: Rc::new(RefCell::new(vec![76, 58, 57, 55, 67, 77, 54, 99])),
            operation: |item| item + 6,
            divisor: 3,
            test_true: 2,
            test_false: 1,
            inspections: Rc::new(RefCell::new(0)),
        },
    ];

    // Part 1
    // const ROUNDS: u64 = 20;
    const ROUNDS: u64 = 10000;
    // The key to the whole thing for part 2, this explains the modulo arithmetic: https://aoc.just2good.co.uk/2022/11#part-2
    // I guess if these weren't primes then lcm of all of them would do the trick
    const PRIME_PRODUCT: u64 = 2*3*5*7*11*13*17*19*23;

    for _ in 0..ROUNDS {
        for (_index, monkey) in input.iter().enumerate() {
            // println!("Monkey {}:", index);
            // There probably is a better way of doing this (the borrow().clone() seems ugly)
            for item in &monkey.items.borrow().clone() {
                // println!("Monkey inspects an item with a worry level of {}", item);
                // Part 1
                // let mut worry_level: u64 = (monkey.operation)(item) / 3;
                let worry_level: u64 = (monkey.operation)(item % PRIME_PRODUCT);
                if worry_level % monkey.divisor == 0 {
                    input[monkey.test_true].items.borrow_mut().push(worry_level);
                    // println!("TRUE Item with worry_level {} is thrown to monkey {} by monkey {}", worry_level, monkey.test_true, x);
                } else {
                    input[monkey.test_false].items.borrow_mut().push(worry_level);
                    // println!("FALSE Item with worry_level {} is thrown to monkey {} by monkey {}", worry_level, monkey.test_false, x);
                }
                *monkey.inspections.borrow_mut() += 1;
            }
            monkey.items.borrow_mut().clear();
        }
    }

    let mut sorted_monkeys = input.clone();
    // Different ways of sorting a vector
    // sorted_monkeys.sort_by(|a, b| b.inspections.borrow().cmp(&a.inspections.borrow()));
    sorted_monkeys.sort_by_key(|x| Reverse(*x.inspections.borrow()));

    for monkey in &sorted_monkeys {
        println!("{} {:?}", monkey.inspections.borrow(), monkey.items.borrow());
    }

    println!("{}", (*sorted_monkeys[0].inspections.borrow()) * (*sorted_monkeys[1].inspections.borrow()));
}
