use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    default::default,
    fmt::Display,
    fs, vec,
};

enum BurrowState {
    Empty,
    Done,
    HasOthers,
    HasSame,
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
        if self.letter == 'E' {}
        let ret_val = match self.letter {
            'A' => self.state.apos[self.index],
            'B' => self.state.bpos[self.index],
            'C' => self.state.cpos[self.index],
            'D' => self.state.dpos[self.index],
            _ => return None,
        };
        self.index += 1;
        if self.index > 1 {
            self.index = 0;
            self.letter = (self.letter as u8 + 1) as char;
        }
        Some(ret_val)
    }
}

#[derive(Eq, Default, Debug, Clone)]
struct BoardState {
    pub cost: u32,
    last: Option<Box<BoardState>>,
    steps: u32,
    apos: Vec<(u8, u8)>,
    bpos: Vec<(u8, u8)>,
    cpos: Vec<(u8, u8)>,
    dpos: Vec<(u8, u8)>,
}

impl std::hash::Hash for BoardState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.apos.hash(state);
        self.bpos.hash(state);
        self.cpos.hash(state);
        self.dpos.hash(state);
    }
}

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
        write!(f, "##\n  #")?;
        for x in (2..=8).step_by(2) {
            self.print_at_point(f, (x, 2))?;
            write!(f, "#")?;
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

    pub fn iter(&self) -> BoardStatePointsIter {
        BoardStatePointsIter::new(self)
    }

    fn get_point_loc(&self, point: PointPair) -> Option<(char, usize)> {
        for (count, p) in self.iter().enumerate() {
            if p == point {
                return Some(((65 + (count as u8 / 2_u8)) as char, (count % 2) as usize));
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
            _ => unreachable!(),
        }
    }

    pub fn initialize(init_data: Vec<char>) -> Self {
        let mut count = 0;

        let mut ret_val = BoardState::default();

        for y in 1..=2 {
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
            _ => unreachable!(),
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
        match self.get_point_loc((x, 2)) {
            Some((cc, _)) if cc == c => match self.get_point_loc((x, 1)) {
                Some((cc, _)) if cc == c => BurrowState::Done,
                Some(_) => BurrowState::HasOthers,
                None => BurrowState::HasSame,
            },
            Some(_) => BurrowState::HasOthers,
            None => BurrowState::Empty,
        }
    }

    fn get_point_moves(&self, og_point: PointPair) -> Vec<(PointPair, u32)> {
        let point = self.get_point_loc(og_point).unwrap();

        if Self::x_val_for_char(point.0) == og_point.0 {
            if og_point.1 == 2 {
                return vec![];
            }
            if og_point.1 == 1 {
                if let Some(t) = self.get_point_loc((og_point.0, og_point.1 + 1)) {
                    if t.0 == point.0 {
                        return vec![];
                    }
                }
            }
        }

        if let Some(y) = match self.get_burrow(point.0) {
            BurrowState::Empty => Some(2),
            BurrowState::Done => return vec![],
            BurrowState::HasOthers => None,
            BurrowState::HasSame => Some(1),
        } {
            let end = (Self::x_val_for_char(point.0), y);
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
        let a = (distance((2, 1), self.apos[0]) + distance((2, 2), self.apos[1]))
            .min(distance((2, 1), self.apos[1]) + distance((2, 2), self.apos[0]));

        let b = (distance((4, 1), self.bpos[0]) + distance((4, 2), self.bpos[1]))
            .min(distance((4, 1), self.bpos[1]) + distance((4, 2), self.bpos[0]));

        let c = (distance((6, 1), self.cpos[0]) + distance((6, 2), self.cpos[1]))
            .min(distance((6, 1), self.cpos[1]) + distance((6, 2), self.cpos[0]));
        let d = (distance((8, 1), self.dpos[0]) + distance((8, 2), self.dpos[1]))
            .min(distance((8, 1), self.dpos[1]) + distance((8, 2), self.dpos[0]));

        a + b + c + d
    }

    fn _print_last_moves(&self) {
        if let Some(pat) = &self.last {
            pat._print_last_moves();
        }
        println!("\n{}\n", self);
    }

    fn _print_last_moves_reversed(&self) {
        println!("\n{}\ncost:{}\n", self, self.cost);
        if let Some(pat) = &self.last {
            pat._print_last_moves_reversed();
        }
    }

    fn into_moves(self) -> Vec<Self> {
        let mut ret_val = Vec::new();

        for p in self.iter() {
            let mut new_state = self.clone();
            if let Some(letter) = new_state.remove_point(p) {
                for moves in self.get_point_moves(p) {
                    let mut new_state = new_state.clone();
                    new_state.last = Some(Box::new(self.clone()));
                    match letter {
                        'A' => {
                            new_state.apos.push(moves.0);
                            new_state.steps += 1;
                            new_state.cost += moves.1;
                        }
                        'B' => {
                            new_state.bpos.push(moves.0);
                            new_state.steps += 1;
                            new_state.cost += moves.1 * 10;
                        }
                        'C' => {
                            new_state.cpos.push(moves.0);
                            new_state.steps += 1;
                            new_state.cost += moves.1 * 100;
                        }
                        'D' => {
                            new_state.dpos.push(moves.0);
                            new_state.steps += 1;
                            new_state.cost += moves.1 * 1000;
                        }
                        _ => unreachable!(),
                    }
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
        ((self.apos[0] == other.apos[0] && self.apos[1] == other.apos[1])
            || (self.apos[0] == other.apos[1] && self.apos[1] == other.apos[0]))
            && ((self.bpos[0] == other.bpos[0] && self.bpos[1] == other.bpos[1])
                || (self.bpos[0] == other.bpos[1] && self.bpos[1] == other.bpos[0]))
            && ((self.cpos[0] == other.cpos[0] && self.cpos[1] == other.cpos[1])
                || (self.cpos[0] == other.cpos[1] && self.cpos[1] == other.cpos[0]))
            && ((self.dpos[0] == other.dpos[0] && self.dpos[1] == other.dpos[1])
                || (self.dpos[0] == other.dpos[1] && self.dpos[1] == other.dpos[0]))
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Equal => self.steps.cmp(&other.steps),
            o => o,
        }
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cost.partial_cmp(&other.cost) {
            Some(Ordering::Equal) | None => {
                match self.calc_distances().partial_cmp(&other.calc_distances()) {
                    Some(Ordering::Equal) => self.steps.partial_cmp(&other.steps),
                    Some(o) => Some(o),
                    None => None,
                }
            }
            o => o,
        }
    }
}

pub fn day23_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        std::env::var("ADVENT_OF_CODE_2021").unwrap(),
        file_name
    );

    let data = fs::read_to_string(input_file)
        .unwrap()
        .chars()
        .filter(|c| matches!(c, 'A'..='D'))
        .collect::<Vec<_>>();

    let mut states: BinaryHeap<Reverse<BoardState>> = default();
    states.push(Reverse(BoardState::initialize(data)));

    let mut been_to = HashMap::<BoardState, u32>::new();
    been_to.insert(states.peek().unwrap().0.clone(), 0);

    while let Some(Reverse(state)) = states.pop() {
        if state.calc_distances() == 0 {
            return state.cost;
        }

        for m in state.into_moves() {
            if (!been_to.contains_key(&m)) || *been_to.get(&m).unwrap() > m.cost {
                been_to.insert(m.clone(), m.cost);
                states.push(Reverse(m));
            }
        }
    }
    unreachable!("Ran out of posible board states")
}