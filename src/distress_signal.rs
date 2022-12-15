use crate::DailyProblem;

pub struct DistressSignal;

impl DailyProblem for DistressSignal {
    fn name(&self) -> &str {
        "Day 13: Distress Signal"
    }
    fn index(&self) -> u8 {
        13
    }
    fn solutions(&self, input: &str) -> (String, String) {
        solve(input)
    }
}

use core::cmp::Ordering;

fn solve(input: &str) -> (String, String) {
    let pairs = input.split("\n\n");
    let divider_packet_one = Node::List(vec![Node::List(vec![Node::Integer(2)])]);
    let divider_packet_two = Node::List(vec![Node::List(vec![Node::Integer(6)])]);

    let mut all_packets = vec![divider_packet_one, divider_packet_two];
    let mut correct = 0;
    for (index, pair) in pairs.enumerate() {
        let mut lines = pair.lines();
        let a = Node::new(lines.next().unwrap());
        let b = Node::new(lines.next().unwrap());
        let cmp = a < b;
        all_packets.push(a);
        all_packets.push(b);
        // println!("a: {:?}, b: {:?}, cmp: {:?}", a, b, cmp);
        if cmp {
            correct += index + 1
        };
    }
    // println!("{}", correct);
    all_packets.sort();

    let mut decoder_key = 1;
    for (index, packet) in all_packets.iter().enumerate() {
        let divider_packet_one = Node::List(vec![Node::List(vec![Node::Integer(2)])]);
        let divider_packet_two = Node::List(vec![Node::List(vec![Node::Integer(6)])]);
        if packet == &divider_packet_one || packet == &divider_packet_two {
            // println!("{}: {:?}", index + 1, packet);
            decoder_key *= index + 1;
        }
    }
    //println!("{}", decoder_key);
    (correct.to_string(), decoder_key.to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
    Integer(u32),
}

impl Node {
    fn new(input: &str) -> Self {
        if input.is_empty() {
            return Node::List(vec![]);
        }
        let mut chars = input.chars();
        if chars.next().unwrap() == '[' {
            let mut list = vec![];
            let mut bracket_level = 0;
            let mut start_char_index = 1;
            let mut char_index = 1;
            for c in chars {
                match c {
                    ',' => {
                        if bracket_level == 0 {
                            list.push(&input[start_char_index..char_index]);
                            start_char_index = char_index + 1;
                        }
                    }
                    ']' => {
                        if bracket_level == 0 {
                            list.push(&input[start_char_index..char_index]);
                            start_char_index = char_index + 1;
                        } else {
                            bracket_level -= 1;
                        }
                    }
                    '[' => {
                        bracket_level += 1;
                    }
                    _ => {}
                }
                char_index += 1;
            }
            Self::List(list.into_iter().map(Node::new).collect())
        } else {
            Self::Integer(input.parse().unwrap())
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Node::Integer(v) => match other {
                Node::Integer(ov) => v.cmp(ov),
                Node::List(_) => Node::List(vec![Node::Integer(*v)]).cmp(other),
            },
            Node::List(l) => match other {
                Node::Integer(ov) => self.cmp(&Node::List(vec![Node::Integer(*ov)])),
                Node::List(ol) => {
                    let mut other_list = ol.iter();
                    for self_element in l {
                        match other_list.next() {
                            Some(other_element) => {
                                if self_element != other_element {
                                    return self_element.cmp(other_element);
                                }
                            }
                            None => {
                                return Ordering::Greater;
                            }
                        }
                    }
                    match other_list.next() {
                        Some(_) => Ordering::Less,
                        None => Ordering::Equal,
                    }
                }
            },
        }
    }
}
