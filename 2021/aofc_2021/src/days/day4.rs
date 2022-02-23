use std::collections::{HashMap, HashSet};

use std::fs;

#[derive(Debug, Clone, Default)]
struct Board {
    played_set: HashSet<u8>,
    contained_pices: HashMap<u8, (usize, usize)>,
    col: [u8; 5],
    row: [u8; 5],
    initial_str: String,
    completed: bool,
}


impl Board {
    fn from_str<T>(str_data: T) -> Board
    where
        T: AsRef<str>,
    {
        let mut ret_baord = Board {
            initial_str: str_data.as_ref().to_owned(),
            ..Board::default()
        };

        for (r, row_str) in str_data.as_ref().split('\n').enumerate() {
            for (c, num) in row_str
                .split(' ')
                .filter_map(|v| v.parse::<u8>().ok())
                .enumerate()
            {
                ret_baord.contained_pices.insert(num, (c, r));
            }
        }

        ret_baord
    }

    fn make_move(&mut self, num: u8) -> bool {
        if self.completed || self.played_set.contains(&num) {
            return false;
        }
        if let Some((c, r)) = self.contained_pices.get(&num) {
            self.played_set.insert(num);
            self.col[*c] += 1;
            self.row[*r] += 1;
            if self.col[*c] == 5 || self.row[*r] == 5 {
                self.completed = true;
                return true;
            }
        }
        false
    }
}

pub fn day4_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n\n").collect::<Vec<_>>();
    let moves = data[0].split(',').filter_map(|v| v.parse::<u8>().ok());
    let mut boards: Vec<Board> = data.iter().skip(1).map(Board::from_str).collect();

    let (winning_board, last_num) = {
        let mut ret_val = (0, 0);
        'moves_loop: for m in moves {
            for (i, b) in boards.iter_mut().enumerate() {
                if b.make_move(m) {
                    ret_val = (i, m);
                    break 'moves_loop;
                }
            }
        }
        (boards.remove(ret_val.0), ret_val.1)
    };
    println!("{}\n", winning_board.initial_str);

    winning_board
        .contained_pices
        .keys()
        .into_iter()
        .filter_map(|n| {
            if !winning_board.played_set.contains(n) {
                Some(*n as usize)
            } else {
                None
            }
        })
        .sum::<usize>()
        * last_num as usize
}

pub fn day4_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let data = data.split("\n\n").collect::<Vec<_>>();
    let moves = data[0].split(',').filter_map(|v| v.parse::<u8>().ok());
    let mut boards: Vec<Board> = data.iter().skip(1).map(Board::from_str).collect();

    let mut winning_board = 0;
    let mut last_num = 0;

    for m in moves {
        for (i, b) in boards.iter_mut().enumerate() {
            if b.make_move(m) {
                winning_board = i;
                last_num = m;
            }
        }
    }
    let winning_board = &boards[winning_board];

    println!("{}\n", winning_board.initial_str);

    winning_board
        .contained_pices
        .keys()
        .into_iter()
        .filter_map(|n| {
            if !winning_board.played_set.contains(n) {
                Some(*n as usize)
            } else {
                None
            }
        })
        .sum::<usize>()
        * last_num as usize
}
