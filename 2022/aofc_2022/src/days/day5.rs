use std::fs;

#[derive(Debug, Default, Clone)]
struct Room {
    crates: Vec<char>,
}

pub fn day5_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n\n").collect::<Vec<_>>();
    let mut rooms = build_rooms(data[0].into());
    let instructions = data[1].split('\n').map(|line| {
        line.split(' ')
            .skip(1)
            .step_by(2)
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    for instruction in instructions {
        move_crates(
            &mut rooms,
            instruction[1] - 1,
            instruction[2] - 1,
            instruction[0],
        );
    }

    stack_tops(&mut rooms)
}

pub fn day5_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2022/input/{}",
        env!("ADVENT_OF_CODE_2022"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n\n").collect::<Vec<_>>();
    let mut rooms = build_rooms(data[0].into());
    let instructions = data[1].split('\n').map(|line| {
        line.split(' ')
            .skip(1)
            .step_by(2)
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    for instruction in instructions {
        move_crates_9001(
            &mut rooms,
            instruction[1] - 1,
            instruction[2] - 1,
            instruction[0],
        );
    }

    stack_tops(&mut rooms)
}

fn stack_tops(rooms: &mut Vec<Room>) -> String {
    rooms
        .iter()
        .map(|r| r.crates.last().unwrap_or(&' '))
        .collect()
}

fn move_crates(rooms: &mut Vec<Room>, start: usize, dest: usize, repeat: usize) {
    for _ in 0..repeat {
        if let Some(val) = rooms[start].crates.pop() {
            rooms[dest].crates.push(val);
        }
    }
}

fn move_crates_9001(rooms: &mut Vec<Room>, start: usize, dest: usize, repeat: usize) {

    let mut tmp_stack = Vec::new();
    for _ in 0..repeat {
        if let Some(val) = rooms[start].crates.pop() {
            tmp_stack.push(val);

        }
    }
    for val in tmp_stack.iter().rev(){
        rooms[dest].crates.push(*val);
    }
}

fn build_rooms(data: String) -> Vec<Room> {
    let data = data
        .split('\n')
        .rev()
        .skip(1)
        .map(|s| s.chars().skip(1).step_by(4).collect::<String>())
        .collect::<Vec<_>>();

    let mut rooms = Vec::new();

    for line in data {
        for (i, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            if rooms.len() < i + 1 {
                rooms.push(Room::default());
            }
            rooms[i].crates.push(c)
        }
    }

    rooms
}
