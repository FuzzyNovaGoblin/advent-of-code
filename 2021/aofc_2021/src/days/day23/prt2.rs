use dbgu::dbgu;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    default::default,
    fmt::Display,
    fs,
    io::stdin,
    vec,
};

const NEW_INSERT: &str = "  #D#C#B#A#\n  #D#B#A#C#";

enum BurrowState {
    Valid(u8),
    Done,
    HasOthers,
}

type PointPair = (u8, u8);

const POINTS: [(u8, u8); 19] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (2, 1),
    (2, 2),
    (3, 0),
    (4, 0),
    (4, 1),
    (4, 2),
    (5, 0),
    (6, 0),
    (6, 1),
    (6, 2),
    (7, 0),
    (8, 0),
    (8, 1),
    (8, 2),
    (9, 0),
    (10, 0),
];

const STOPING_POINTS: [PointPair; 7] = [(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)];

fn distance(a: PointPair, b: PointPair) -> i32 {
    if a == b {
        return 0;
    }

    if a.0 == b.0 {
        a.1.abs_diff(b.1) as i32
    } else {
        (a.1 + b.1 + b.0.abs_diff(a.0)) as i32
    }
}

struct BoardStatePointsIter<'a> {
    letter: char,
    index: usize,
    state: &'a BoardState,
}

impl<'a> BoardStatePointsIter<'a> {
    fn new(state: &'a BoardState) -> Self {
        Self {
            letter: 'A',
            index: 0,
            state,
        }
    }
}

impl<'a> Iterator for BoardStatePointsIter<'a> {
    type Item = PointPair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.letter == 'E' {
            return None;
        }
        let ret_val = match self.letter {
            'A' => self.state.apos[self.index],
            'B' => self.state.bpos[self.index],
            'C' => self.state.cpos[self.index],
            'D' => self.state.dpos[self.index],
            _ => return None,
        };
        self.index += 1;
        if self.index > 3 {
            self.index = 0;
            self.letter = (self.letter as u8 + 1) as char;
        }
        Some(ret_val)
    }
}

#[derive(Eq, Default, Debug, Clone)]
struct BoardState {
    pub cost: u32,
    // last: Option<Box<BoardState>>,
    // pub allprior: HashSet<String>,
    steps: u32,
    apos: Vec<(u8, u8)>,
    bpos: Vec<(u8, u8)>,
    cpos: Vec<(u8, u8)>,
    dpos: Vec<(u8, u8)>,
    // image: String,
}

// impl std::hash::Hash for BoardState {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.image.hash(state);
//     }
// }

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n#")?;
        for x in 0..=10 {
            self.print_at_point(f, (x, 0))?;
        }
        write!(f, "#\n###")?;
        for x in (2..=8).step_by(2) {
            self.print_at_point(f, (x, 1))?;
            write!(f, "#")?;
        }
        write!(f, "##")?;
        for y in 2..=4 {
            write!(f, "\n  #")?;
            for x in (2..=8).step_by(2) {
                self.print_at_point(f, (x, y))?;
                write!(f, "#")?;
            }
        }
        write!(f, "\n  #########")
    }
}

impl BoardState {
    fn print_at_point(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        point: PointPair,
    ) -> std::fmt::Result {
        match self.get_point_loc(point) {
            Some((c, _)) => write!(f, "{}", c),
            None => write!(f, "."),
        }
    }

    // pub fn is_loop(&self, other: &BoardState)->bool{
    //     self.allprior.contains(&other.image)
    // }

    pub fn iter(&self) -> BoardStatePointsIter {
        BoardStatePointsIter::new(self)
    }

    fn get_point_loc(&self, point: PointPair) -> Option<(char, usize)> {
        for (count, p) in self.iter().enumerate() {
            if p == point {
                return Some(((65 + (count as u8 / 4_u8)) as char, (count % 4) as usize));
            }
        }
        None
    }

    fn remove_point(&mut self, point: PointPair) -> Option<char> {
        match self.get_point_loc(point) {
            Some(('A', index)) => {
                self.apos.remove(index);
                Some('A')
            }
            Some(('B', index)) => {
                self.bpos.remove(index);
                Some('B')
            }
            Some(('C', index)) => {
                self.cpos.remove(index);
                Some('C')
            }
            Some(('D', index)) => {
                self.dpos.remove(index);
                Some('D')
            }
            None => None,
            _ => unreachable!("Point can only have letter 'A', 'B', 'C', or 'D': "),
        }
    }

    pub fn initialize(init_data: Vec<Vec<char>>) -> Self {
        let mut ret_val = BoardState::default();

        for (y, row) in init_data.into_iter().enumerate() {
            for (x, c) in row.into_iter().enumerate() {
                match c {
                    'A' => ret_val.apos.push(((x) as u8, (y) as u8)),
                    'B' => ret_val.bpos.push(((x) as u8, (y) as u8)),
                    'C' => ret_val.cpos.push(((x) as u8, (y) as u8)),
                    'D' => ret_val.dpos.push(((x) as u8, (y) as u8)),
                    _ => (),
                }
            }
        }

        // ret_val.image = format!("{}", ret_val);
        ret_val
    }

    pub fn initialize_with_addin(init_data: Vec<char>) -> Self {
        let mut count = 0;

        let mut ret_val = BoardState::default();

        for y in 1..=4 {
            for x in (2..=8).step_by(2) {
                let c = init_data[count];
                match c {
                    'A' => ret_val.apos.push((x, y)),
                    'B' => ret_val.bpos.push((x, y)),
                    'C' => ret_val.cpos.push((x, y)),
                    'D' => ret_val.dpos.push((x, y)),
                    _ => unreachable!(),
                }
                count += 1;
            }
        }
        // ret_val.image = format!("{}", ret_val);
        ret_val
    }

    fn get_neighbors((start_x, start_y): PointPair) -> Vec<PointPair> {
        let mut ret_val = Vec::new();
        for x in sub_one_or_zero(start_x)..=start_x + 1 {
            for y in sub_one_or_zero(start_y)..=start_y + 1 {
                let tmp_point = (x, y);
                if (x != start_x && y != start_y) || (x, y) == (start_x, start_y) {
                    continue;
                }
                if POINTS.contains(&tmp_point) {
                    ret_val.push(tmp_point)
                }
            }
        }
        ret_val
    }

    fn x_val_for_char(c: char) -> u8 {
        match c {
            'A' => 2,
            'B' => 4,
            'C' => 6,
            'D' => 8,
            _ => unreachable!("Point can only have letter 'A', 'B', 'C', or 'D': "),
        }
    }

    fn clear_path(&self, start: PointPair, end: PointPair) -> Option<u32> {
        let mut point = start;
        let mut count = 0;
        let used_points = self.iter().collect::<HashSet<_>>();
        let mut been_to = HashSet::new();
        while point != end {
            match Self::get_neighbors(point)
                .into_iter()
                .filter(|v| !used_points.contains(v) && !been_to.contains(v))
                .reduce(|fv, nv| {
                    if distance(end, fv) < distance(end, nv) {
                        fv
                    } else {
                        nv
                    }
                }) {
                Some(p) => {
                    been_to.insert(p);
                    point = p;
                    count += 1;
                }
                None => return None,
            }
        }
        Some(count)
    }

    fn get_burrow(&self, c: char) -> BurrowState {
        let x = Self::x_val_for_char(c);
        for y in (1..=4).rev() {
            match self.get_point_loc((x, y)) {
                Some((c_in_b, _)) if c_in_b != c => return BurrowState::HasOthers,
                Some(_) => (),
                // Some(_) => panic!(
                //     "how is it getting here char: {c}\tSelf::x_val_for_char({c}): {}\t y:{y}",
                //     Self::x_val_for_char(c)
                // ),
                None => return BurrowState::Valid(y),
            }
        }
        BurrowState::Done
        // match self.get_point_loc((x, 2)) {
        //     Some((cc, _)) if cc == c => match self.get_point_loc((x, 1)) {
        //         Some((cc, _)) if cc == c => BurrowState::Done,
        //         Some(_) => BurrowState::HasOthers,
        //         None => BurrowState::HasSame,
        //     },
        //     Some(_) => BurrowState::HasOthers,
        //     None => BurrowState::Empty,
        // }
    }

    fn get_point_moves(
        &self,
        og_point: PointPair,
        burrow_states: &HashMap<char, BurrowState>,
    ) -> Vec<(PointPair, u32)> {
        let point = self.get_point_loc(og_point).unwrap();

        // if in destination burrow
        if Self::x_val_for_char(point.0) == og_point.0 {
            for y in (og_point.1 + 1)..=4 {
                match self.get_point_loc((og_point.0, y)) {
                    Some((c, _)) if c == point.0 => (),
                    _ => break,
                }
                if y == 4 {
                    return vec![];
                }
            }
        }

        if let Some(y) = match burrow_states.get(&point.0).unwrap() {
            BurrowState::Done => return vec![],
            BurrowState::HasOthers => None,
            BurrowState::Valid(v) => Some(v),
        } {
            let end = (Self::x_val_for_char(point.0), *y);
            if let Some(cp) = self.clear_path(og_point, end) {
                return vec![(end, cp)];
            }
        }

        if og_point.1 != 0 {
            let mut ret_val = Vec::new();

            for p in STOPING_POINTS {
                if let Some(cp) = self.clear_path(og_point, p) {
                    ret_val.push((p, cp));
                }
            }
            ret_val
        } else {
            vec![]
        }
    }

    fn calc_distances(&self) -> i32 {
        let mut ret_val = 0;

        macro_rules! calc_dist_mac {
            ($vec_name:ident, $target:literal) => {
                for i in 0..4 {
                    let mut dist = ($target - (self.$vec_name[i].0 as i32)).abs();
                    if self.$vec_name[i].1 != 0 {
                        dist *= 2;
                    }
                    ret_val += dist;
                }
            };
        }

        calc_dist_mac!(apos, 2);
        calc_dist_mac!(bpos, 4);
        calc_dist_mac!(cpos, 6);
        calc_dist_mac!(dpos, 8);
        ret_val
    }

    // fn _print_last_moves(&self) {
    //     if let Some(pat) = &self.last {
    //         pat._print_last_moves();
    //     }
    //     println!("\n{}\n", self);
    // }

    // fn _print_last_moves_reversed(&self) {
    //     println!("\n{}\ncost:{}\n", self, self.cost);
    //     if let Some(pat) = &self.last {
    //         pat._print_last_moves_reversed();
    //     }
    // }

    fn into_moves(self) -> Vec<Self> {
        let mut ret_val = Vec::new();

        //chache the state of each burrow
        let mut burrow_states = HashMap::<char, BurrowState>::new();
        for i in 'A'..='D' {
            burrow_states.insert(i, self.get_burrow(i));
        }

        for p in self.iter() {
            let mut new_state = self.clone();
            if let Some(letter) = new_state.remove_point(p) {
                for moves in self.get_point_moves(p, &burrow_states) {
                    let mut new_state = new_state.clone();
                    // new_state.last = Some(Box::new(self.clone()));
                    new_state.steps += 1;
                    match letter {
                        'A' => {
                            new_state.apos.push(moves.0);
                            new_state.cost += moves.1;
                        }
                        'B' => {
                            new_state.bpos.push(moves.0);
                            new_state.cost += moves.1 * 10;
                        }
                        'C' => {
                            new_state.cpos.push(moves.0);
                            new_state.cost += moves.1 * 100;
                        }
                        'D' => {
                            new_state.dpos.push(moves.0);
                            new_state.cost += moves.1 * 1000;
                        }
                        _ => unreachable!("Point can only have letter 'A', 'B', 'C', or 'D': "),
                    }
                    // new_state.image = format!("{}", new_state);
                    // new_state.allprior.insert(new_state.image.clone());
                    ret_val.push(new_state);
                }
            }
        }

        ret_val
    }
}

fn sub_one_or_zero(v: u8) -> u8 {
    match v {
        0 => 0,
        v => v - 1,
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
        // self.image == other.image
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
        // match self.cost.cmp(&other.cost) {
        //     Ordering::Equal => self.steps.cmp(&other.steps),
        //     o => o,
        // }
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
        // match self.cost.partial_cmp(&other.cost) {
        //     Some(Ordering::Equal) | None => {
        //         match self.calc_distances().partial_cmp(&other.calc_distances()) {
        //             Some(Ordering::Equal) => self.steps.partial_cmp(&other.steps),
        //             Some(o) => Some(o),
        //             None => None,
        //         }
        //     }
        //     o => o,
        // }
    }
}

pub fn day23_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );

    let data = {
        let mut file_data = fs::read_to_string(input_file)
            .unwrap()
            .split('\n')
            .map(str::to_owned)
            .collect::<Vec<String>>();
        if file_data.len() <= 5 {
            file_data.insert(3, String::from(NEW_INSERT));
        }
        file_data
            .into_iter()
            .fold(String::new(), |fv, nv| format!("{}\n{}", fv, nv))
            .split('\n')
            .skip(2)
            .map(|s| s.chars().skip(1).collect())
            .collect::<Vec<Vec<char>>>()
    };

    // let tmp_board = BoardState::initialize(data);

    let mut states: BinaryHeap<Reverse<BoardState>> = default();
    println!("{:?}", data);
    dbg!(&data);
    println!("{}", BoardState::initialize(data.clone()));
    states.push(Reverse(BoardState::initialize(data)));

    // println!("{:?}", states);

    // println!("{}", states.pop().unwrap().0);
    // println!("{:?}", states.pop().unwrap().0);
    // let mut been_to = HashMap::<BoardState, u32>::new();
    // been_to.insert(states.peek().unwrap().0.clone(), 0);

    let mut count = 0;
    while let Some(Reverse(state)) = states.pop() {
        count += 1;
        // if count >= 500 {
        //     count = 0;
        //     println!(
        //         "remaining moves:{}\ncost: {}\n{}\n\n",
        //         states.len(),
        //         state.cost,
        //         state
        //     );
        // }

        if state.calc_distances() == 0 {
            return state.cost;
        }

        for m in state.clone().into_moves() {
            // if m.cost == 40 {
            //     println!("--------------");
            //     println!("first");
            //     println!("{}\n", m);
            //     for b in &been_to {
            //         println!("{}\n", b.0);
            //     }
            //     println!("--------------");
            // }
            states.push(Reverse(m));

            // else {
            //     let e = been_to.entry(m.clone()).or_insert(m.cost);
            //     if *e > m.cost {
            //         *e = m.cost;
            //     }
            // }
        }
        // let mut tmp = String::new();
        // stdin().read_line(&mut tmp);
    }
    unreachable!("Can't run out of posible board states before finding answer")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    fn t2() {
        assert_eq_ansval!((), day23_2("test"));
    }
}
