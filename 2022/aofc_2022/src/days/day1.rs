use std::fs;

#[derive(Default, Debug)]
struct Elf {
    foods: Vec<u32>,
    total_cals: u32,
}

impl Elf {
    fn add_food(&mut self, food: u32) {
        self.foods.push(food);
        self.total_cals += food;
    }
}

pub fn day1_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);

    let mut elves = vec![Elf::default()];

    for food in data.unwrap().lines().map(|line| match line.parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => None,
    }) {
        match food {
            Some(calories) => elves.last_mut().unwrap().add_food(calories),

            None => elves.push(Elf::default()),
        }
    }

    elves
        .iter()
        .fold(0, |carry, new_val| match carry > new_val.total_cals {
            true => carry,
            false => new_val.total_cals,
        })
}

pub fn day1_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file);

    let mut elves = vec![Elf::default()];

    for food in data.unwrap().lines().map(|line| match line.parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => None,
    }) {
        match food {
            Some(calories) => elves.last_mut().unwrap().add_food(calories),

            None => elves.push(Elf::default()),
        }
    }

    elves
        .iter()
        .scan(vec![], |state, new_val| {
            if state.len() < 3 {
                state.push(new_val.total_cals);
            } else if *state.first().unwrap_or(&0) < new_val.total_cals {
                state.remove(0);
                state.push(new_val.total_cals);
                state.sort();
            }
            Some(state.iter().sum::<u32>())
        })
        .last()
        .unwrap()
}
