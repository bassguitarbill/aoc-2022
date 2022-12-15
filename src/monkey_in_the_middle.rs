use aoc_2022::DailyProblem;

pub struct MonkeyInTheMiddle;

impl DailyProblem for MonkeyInTheMiddle {
    fn name(&self) -> &str {
        "Day 11: Monkey in the Middle"
    }
    fn index(&self) -> u8 {
        11
    }
    fn solutions(&self, input: &str) -> (String, String) {
        (
            monkey_business(input, 20, true).to_string(),
            monkey_business(input, 10000, false).to_string(),
        )
    }
}

fn monkey_business(input: &str, num_rounds: u32, worry_drop: bool) -> u64 {
    let mut monkeys = Monkeys::new(input, worry_drop);
    (0..num_rounds).for_each(|_| monkeys.round());
    let mut monkey_business: Vec<u64> =
        monkeys.monkeys.iter().map(|m| m.inspection_count).collect();
    monkey_business.sort();
    monkey_business[(monkey_business.len() - 2)..]
        .iter()
        .product()
}

struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn new(input: &str, worry_drop: bool) -> Self {
        let mut monkeys: Vec<Monkey> = input
            .split("\n\n")
            .map(|m| Monkey::new(m, worry_drop))
            .collect();
        let worry_factor = monkeys.iter().map(|m| m.test_divisible_by).product();
        monkeys
            .iter_mut()
            .for_each(|m| m.worry_factor = worry_factor);
        Self { monkeys }
    }

    fn round(&mut self) {
        let mut to_throw_list: Vec<Vec<u64>> = vec![];
        for _ in 0..self.monkeys.len() {
            to_throw_list.push(vec![]);
        }
        for i in 0..to_throw_list.len() {
            let m = self.monkeys.get_mut(i).unwrap();
            let items_to_throw = m.take_turn();
            for (item, m_i) in items_to_throw {
                self.monkeys.get_mut(m_i).unwrap().items.push(item);
            }
        }
        /*for m in self.monkeys.iter_mut() {
            for (item, to_throw_to) in m.take_turn() {
                to_throw_list.get_mut(to_throw_to).unwrap().push(item);
            }
        }*/
        for i in 0..to_throw_list.len() {
            for item in to_throw_list.get(i).unwrap() {
                self.monkeys.get_mut(i).unwrap().items.push(*item);
            }
        }
    }

    fn _print(&self) {
        for m in self.monkeys.iter() {
            println!("{:?}", m);
        }
        println!();
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    index: usize,
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    inspection_count: u64,
    worry_drop: bool,
    worry_factor: u64,
}

impl Monkey {
    fn new(input: &str, worry_drop: bool) -> Self {
        let mut lines = input.lines();
        let index = lines
            .next()
            .unwrap()
            .replace(':', "")
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let items_line = lines.next().unwrap().trim().replace(',', "");
        let mut item_chunks = items_line.split(' ');
        item_chunks.nth(1);
        let mut items = vec![];
        item_chunks.for_each(|i| items.push(i.parse().unwrap()));
        let operation = Operation::new(lines.next().unwrap());
        let test_divisible_by = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let throw_to_if_true = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let throw_to_if_false = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            index,
            items,
            operation,
            test_divisible_by,
            throw_to_if_true,
            throw_to_if_false,
            inspection_count: 0,
            worry_drop,
            worry_factor: 0,
        }
    }

    fn take_turn(&mut self) -> Vec<(u64, usize)> {
        let mut items_to_throw = vec![];
        self.items.iter().for_each(|i| {
            items_to_throw.push(self.throw_item(i));
        });
        self.inspection_count += self.items.len() as u64;
        self.items.clear();
        items_to_throw
    }

    fn throw_item(&self, i: &u64) -> (u64, usize) {
        let mut item = *i;
        item = match &self.operation {
            Operation::Add(to_add) => item + to_add,
            Operation::AddSelf => item * 2,
            Operation::Multiply(to_mul) => item * to_mul,
            Operation::MultiplySelf => item * item,
        };
        if self.worry_drop {
            item /= 3;
        } else {
            item %= self.worry_factor;
        }

        (
            item,
            match item % self.test_divisible_by == 0 {
                true => self.throw_to_if_true,
                false => self.throw_to_if_false,
            },
        )
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(u64),
    AddSelf,
    Multiply(u64),
    MultiplySelf,
}

impl Operation {
    fn new(input: &str) -> Self {
        let mut words = input.trim().split(' ');
        match words.nth(4).unwrap() {
            "*" => match words.next().unwrap() {
                "old" => Operation::MultiplySelf,
                num => Operation::Multiply(num.parse().unwrap()),
            },
            "+" => match words.next().unwrap() {
                "old" => Operation::AddSelf,
                num => Operation::Add(num.parse().unwrap()),
            },
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::monkey_in_the_middle::{Monkey, Operation};

    use super::Monkeys;

    #[test]
    fn operation() {
        assert_eq!(
            Operation::new("  Operation: new = old * 3"),
            Operation::Multiply(3)
        );
        assert_eq!(
            Operation::new("  Operation: new = old * old"),
            Operation::MultiplySelf
        );
        assert_eq!(
            Operation::new("  Operation: new = old + 3"),
            Operation::Add(3)
        );
        assert_eq!(
            Operation::new("  Operation: new = old + old"),
            Operation::AddSelf
        );
    }

    #[test]
    fn monkey() {
        let input = "Monkey 0:
  Starting items: 84, 72, 58, 51
  Operation: new = old * 3
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 7";
        assert_eq!(
            Monkey::new(input, false),
            Monkey {
                index: 0,
                items: vec![84, 72, 58, 51],
                operation: Operation::Multiply(3),
                test_divisible_by: 13,
                throw_to_if_true: 1,
                throw_to_if_false: 7,
                inspection_count: 0,
                worry_drop: false,
                worry_factor: 0,
            }
        );
    }

    #[test]
    fn rounds() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let mut monkeys = Monkeys::new(input, true);
        monkeys.round();
        assert_eq!(monkeys.monkeys.get(0).unwrap().items, vec![20, 23, 27, 26]);
        assert_eq!(
            monkeys.monkeys.get(1).unwrap().items,
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert_eq!(monkeys.monkeys.get(2).unwrap().items, vec![]);
        assert_eq!(monkeys.monkeys.get(3).unwrap().items, vec![]);

        monkeys.round();
        assert_eq!(
            monkeys.monkeys.get(0).unwrap().items,
            vec![695, 10, 71, 135, 350]
        );
        assert_eq!(
            monkeys.monkeys.get(1).unwrap().items,
            vec![43, 49, 58, 55, 362]
        );
        assert_eq!(monkeys.monkeys.get(2).unwrap().items, vec![]);
        assert_eq!(monkeys.monkeys.get(3).unwrap().items, vec![]);
        // monkeys.print();
        // assert!(false);
    }
}
