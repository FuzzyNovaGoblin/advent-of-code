use std::{collections::HashMap, fs};

fn generate_orders(length: usize) -> Vec<Vec<usize>> {
    generate_recursion(Vec::new(), length, 0, length)
}
fn generate_recursion(
    prefix: Vec<usize>,
    length: usize,
    depth: usize,
    target_depth: usize,
) -> Vec<Vec<usize>> {
    if depth == target_depth {
        return vec![prefix.clone()];
    }
    let mut ret = Vec::new();
    for i in 0..length {
        if prefix.contains(&i) {
            continue;
        }
        let mut new_prefix = prefix.clone();
        new_prefix.push(i);

        for generated in generate_recursion(new_prefix, length, depth + 1, target_depth) {
            ret.push(generated);
        }
    }
    ret
}

fn order_to_happiness(
    mut order: Vec<usize>,
    names: &Vec<String>,
    table: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    order.push(order[0]);

    order
        .iter()
        .map(|n| &names[*n])
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| table[w[0]][w[1]] + table[w[1]][w[0]])
        .sum::<i32>()
}
pub fn day13_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .replace("would", "")
        .replace("happiness units by sitting next to", "")
        .replace('.', "")
        .replace("  ", " ");

    let mut pairs = HashMap::new();
    let mut names = Vec::new();

    for line in data.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if !pairs.contains_key(parts[0]) {
            pairs.insert(parts[0].to_owned(), HashMap::new());
            names.push(parts[0].to_owned());
        }
        pairs.get_mut(parts[0]).unwrap().insert(
            parts[3].to_owned(),
            if parts[1] == "gain" { 1 } else { -1 } * parts[2].parse::<i32>().unwrap(),
        );
    }
    let orders = generate_orders(names.len());

    orders
        .into_iter()
        .map(|order| order_to_happiness(order, &names, &pairs))
        .max()
}

pub fn day13_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file)
        .unwrap()
        .replace("would", "")
        .replace("happiness units by sitting next to", "")
        .replace('.', "")
        .replace("  ", " ");

    let mut pairs = HashMap::new();
    let mut names = Vec::new();

    for line in data.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if !pairs.contains_key(parts[0]) {
            let mut new_hash = HashMap::new();
            new_hash.insert("You".to_owned(), 0);

            pairs.insert(parts[0].to_owned(), new_hash);
            names.push(parts[0].to_owned());
        }
        pairs.get_mut(parts[0]).unwrap().insert(
            parts[3].to_owned(),
            if parts[1] == "gain" { 1 } else { -1 } * parts[2].parse::<i32>().unwrap(),
        );
    }
    let mut you_hash = HashMap::new();
    for name in names.iter(){
        you_hash.insert(name.clone(), 0);
    }
    names.push("You".to_owned());
    pairs.insert("You".to_owned(), you_hash);

    let orders = generate_orders(names.len());

    orders
        .into_iter()
        .map(|order| order_to_happiness(order, &names, &pairs))
        .max()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day13_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day13_2("test"));
    }
}
