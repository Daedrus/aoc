use std::str::Chars;
use std::iter::Peekable;
use std::cmp::Ordering;

#[derive(Debug)]
enum Either {
    Number(u32),
    List(Vec<Either>),
}

fn parse_list(line: &mut Peekable<Chars>) -> Either {
    let mut list: Vec<Either> = Vec::new();

    loop {
        if line.peek() == None {
            break;
        }

        let c = line.next().unwrap();
        match c {
            '[' => {
                list.push(parse_list(line));
            },
            ']' => {
                break;
            },
            '0' => {
                list.push(Either::Number(0));
            },
            '1' => {
                if line.peek() == Some(&'0') {
                    list.push(Either::Number(10));
                    line.next();
                } else {
                    list.push(Either::Number(1));
                }
            },
            '2' => {
                list.push(Either::Number(2));
            },
            '3' => {
                list.push(Either::Number(3));
            },
            '4' => {
                list.push(Either::Number(4));
            },
            '5' => {
                list.push(Either::Number(5));
            },
            '6' => {
                list.push(Either::Number(6));
            },
            '7' => {
                list.push(Either::Number(7));
            },
            '8' => {
                list.push(Either::Number(8));
            },
            '9' => {
                list.push(Either::Number(9));
            },
            ',' => {
            }
            _ => unreachable!(),
        }
    }

    Either::List(list)
}

fn cmp(list1: &Either, list2: &Either) -> Option<bool> {
    match(list1, list2) {
        (Either::Number(x), Either::Number(y)) => {
            // println!("1 Comparing NUMBER {:?} NUMBER {:?}", list1, list2);
            // println!("1 Comparing NUMBER {:?} NUMBER {:?}", x, y);
            if x != y {
                Some(x < y)
            } else {
                None
            }
         },
        (Either::Number(x), Either::List(_)) => {
            // println!("2 Comparing NUMBER {:?} LIST {:?}", list1, list2);
            // println!("2 Comparing NUMBER {:?} LIST {:?}", x, y);
            cmp(&Either::List(vec![Either::Number(*x)]), &list2)
        },
        (Either::List(_), Either::Number(y)) => {
            // println!("3 Comparing LIST {:?} NUMBER {:?}", list1, list2);
            // println!("3 Comparing LIST {:?} NUMBER {:?}", x, y);
            cmp(&list1, &Either::List(vec![Either::Number(*y)]))
        },
        (Either::List(x), Either::List(y)) => {
            // println!("4 Comparing LIST {:?} LIST {:?}", list1, list2);
            // println!("4 Comparing LIST {:?} LIST {:?}", x, y);
            let mut iter1 = x.iter();
            let mut iter2 = y.iter();
            let equal: Option<bool>;
            loop {
                match (iter1.next(), iter2.next()) {
                    (Some(x), Some(y)) => {
                        if let Some(eq) = cmp(x,y) {
                            equal = Some(eq);
                            if eq {
                                // println!("Left side is smaller, right order");
                            } else {
                                // println!("Right side is smaller, not right order");
                            }
                            break;
                        }
                    },
                    (Some(_), None) => {
                        equal = Some(false);
                        // println!("Right side ran out of items, not right order");
                        break;
                    },
                    (None, Some(_)) => {
                        equal = Some(true);
                        // println!("Left side ran out of items, right order");
                        break;
                    },
                    (None, None) => {
                        equal = None;
                        break;
                    }
                }
            }

            equal
        }
    }
}

//const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let mut index = 1;
    let mut index_sum = 0;
    let mut received_packets = Vec::new();
    while let Some(line) = lines.next() {
        let line1 = parse_list(&mut line.chars().peekable());
        let line2 = parse_list(&mut lines.next().unwrap().chars().peekable());
        lines.next();
        // println!("{:?}", line1);
        // println!("{:?}", line2);
        if let Some(result) = cmp(&line1, &line2) {
            if result {
                index_sum += index;
                // println!("TRUE {}", index);
            } else {
                // println!("FALSE {}", index);
            }
        } else {
            println!("FALSE {}", index);
        }
        index += 1;
        received_packets.push(line1);
        received_packets.push(line2);
    }
    println!("{}", index_sum);

    received_packets.push(Either::List(vec![Either::List(vec![Either::Number(2)])]));
    received_packets.push(Either::List(vec![Either::List(vec![Either::Number(6)])]));
    received_packets.sort_by(|a, b| { if cmp(a, b).unwrap() { Ordering::Less } else { Ordering::Greater }});
    
    // for packet in &received_packets {
    //     println!("{:?}", packet);
    // }
    
    let mut index1 = 0;
    let mut index2 = 0;
    for (index, packet) in received_packets.iter().enumerate() {
        if let Some(result) = cmp(&packet, &Either::List(vec![Either::List(vec![Either::Number(2)])])) {
            if result {
                index1 = index+2;
            }
        }
        if let Some(result) = cmp(&packet, &Either::List(vec![Either::List(vec![Either::Number(6)])])) {
            if result {
                index2 = index+2;
            }
        }
    }

    println!("{}", index1 * index2);
}
