use flux_bnf::{bnf, lexer::CullStrategy};
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    fn from_str(src: char) -> Operator {
        match src {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    own_num: u128,
    items: RefCell<Vec<u128>>,
    operator: Operator,
    operation_num: Option<u128>,

    divisible_by_num: u128,
    throw_to_true: u128,
    throw_to_false: u128,
    inspected_items: u128,
}

impl Monkey {
    fn new(
        own_num: u128,
        items: Vec<u128>,
        operator: Operator,
        operation_num: Option<u128>,
        divisible_by_num: u128,
        throw_to_true: u128,
        throw_to_false: u128,
    ) -> Self {
        Self {
            own_num,
            items: RefCell::new(items),
            operator,
            operation_num,
            divisible_by_num,
            throw_to_true,
            throw_to_false,
            inspected_items: 0,
        }
    }

    fn add_new_item(&self, new_value: u128) {
        self.items.borrow_mut().push(new_value);
    }

    fn process_items(&self, monkeys: &HashMap<u128, Monkey>, part1: bool) -> u128 {
        let mut items = Vec::new();
        let items_len = self.items.borrow().len();

        for _ in 0..items_len {
            items.push(self.items.borrow_mut().remove(0));
        }
        for item in items.into_iter() {
            let new_item = self.run_operation(item) / if part1 { 3 } else { 45 };
            if new_item % self.divisible_by_num == 0 {
                monkeys[&self.throw_to_true].add_new_item(new_item);
            } else {
                monkeys[&self.throw_to_false].add_new_item(new_item);
            }
        }

        items_len as u128
    }

    fn run_operation(&self, old: u128) -> u128 {
        let operand = self.operation_num.unwrap_or(old);
        match self.operator {
            Operator::Plus => old + operand,
            Operator::Minus => old - operand,
            Operator::Multiply => old * operand,
            Operator::Divide => old / operand,
        }
    }
}

pub fn day11_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file.clone()).unwrap();
    let bnf_src = fs::read_to_string(format!(
        "{}/{}",
        PathBuf::from(input_file)
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
        "day11.bnf"
    ))
    .unwrap();
    let mut lexer = bnf::parse(&bnf_src).unwrap();
    lexer.add_rule_for_names(
        vec!["sep", "bool"].iter().map(|s| s.to_string()).collect(),
        CullStrategy::DeleteAll,
    );
    lexer.add_rule_for_names(
        vec!["operand", "throwToCommand", "ifCase"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        CullStrategy::LiftChildren,
    );
    lexer.set_unnamed_rule(CullStrategy::LiftChildren);
    let token = lexer.tokenize(&data).unwrap();

    let mut monkeys = HashMap::new();

    let mut highest_monkey = 0;
    for monkey in token.children.iter() {
        let own_num = monkey.children[0].get_match().parse::<u128>().unwrap();
        highest_monkey = own_num;
        let start_items = monkey.children[1]
            .children
            .iter()
            .map(|c| c.get_match().parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        let (operator, operand) = {
            let parts = &monkey.children[2].children;
            (
                Operator::from_str(parts[1].get_match().chars().nth(0).unwrap()),
                parts[2].get_match().parse::<u128>().ok(),
            )
        };

        let (div_by, to_true, to_false) = {
            let mut parts = monkey.children[3]
                .children
                .iter()
                .filter_map(|t| t.get_match().parse::<u128>().ok())
                .take(3);
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        };

        monkeys.insert(
            own_num,
            Monkey::new(
                own_num,
                start_items,
                operator,
                operand,
                div_by,
                to_true,
                to_false,
            ),
        );
    }

    for round in 0..20 {
        for m_number in 0..=highest_monkey {
            let changes = monkeys[&m_number].process_items(&monkeys, true);
            monkeys.get_mut(&m_number).unwrap().inspected_items += changes;
        }
    }

    let mut inspects = monkeys
        .values()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();
    inspects.sort();
    inspects.reverse();
    inspects[0] * inspects[1]
}

pub fn day11_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file.clone()).unwrap();
    let bnf_src = fs::read_to_string(format!(
        "{}/{}",
        PathBuf::from(input_file)
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
        "day11.bnf"
    ))
    .unwrap();
    let mut lexer = bnf::parse(&bnf_src).unwrap();
    lexer.add_rule_for_names(
        vec!["sep", "bool"].iter().map(|s| s.to_string()).collect(),
        CullStrategy::DeleteAll,
    );
    lexer.add_rule_for_names(
        vec!["operand", "throwToCommand", "ifCase"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        CullStrategy::LiftChildren,
    );
    lexer.set_unnamed_rule(CullStrategy::LiftChildren);
    let token = lexer.tokenize(&data).unwrap();

    let mut monkeys = HashMap::new();

    let mut highest_monkey = 0;
    for monkey in token.children.iter() {
        let own_num = monkey.children[0].get_match().parse::<u128>().unwrap();
        highest_monkey = own_num;
        let start_items = monkey.children[1]
            .children
            .iter()
            .map(|c| c.get_match().parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        let (operator, operand) = {
            let parts = &monkey.children[2].children;
            (
                Operator::from_str(parts[1].get_match().chars().nth(0).unwrap()),
                parts[2].get_match().parse::<u128>().ok(),
            )
        };

        let (div_by, to_true, to_false) = {
            let mut parts = monkey.children[3]
                .children
                .iter()
                .filter_map(|t| t.get_match().parse::<u128>().ok())
                .take(3);
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        };

        monkeys.insert(
            own_num,
            Monkey::new(
                own_num,
                start_items,
                operator,
                operand,
                div_by,
                to_true,
                to_false,
            ),
        );
    }

    for round in 0..10000 {
        for m_number in 0..=highest_monkey {
            let changes = monkeys[&m_number].process_items(&monkeys, false);
            print!("from {} to", monkeys[&m_number].inspected_items);
            monkeys.get_mut(&m_number).unwrap().inspected_items += changes;
            println!("{}", monkeys[&m_number].inspected_items);
        }
    }

    let mut inspects = monkeys
        .values()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();
    inspects.sort();
    inspects.reverse();
    dbg!(&inspects);
    inspects[0] * inspects[1]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day11_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day11_2("test"));
    }
}
