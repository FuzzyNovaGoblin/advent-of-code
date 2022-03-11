use std::fs;

struct LanternFishPopulation {
    lantern_fish_timers: [u128; 10],
}

impl LanternFishPopulation {
    fn new() -> Self {
        Self {
            lantern_fish_timers: [0; 10],
        }
    }
    fn from(lantern_fish_timers: Vec<u32>) -> Self {
        let mut new_val = LanternFishPopulation::new();
        for v in lantern_fish_timers {
            new_val.lantern_fish_timers[(v + 1) as usize] += 1;
        }
        new_val
    }
}

impl Iterator for LanternFishPopulation {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 1..self.lantern_fish_timers.len(){
            self.lantern_fish_timers[i-1] = self.lantern_fish_timers[i];
        }
        self.lantern_fish_timers[9] = self.lantern_fish_timers[0];
        self.lantern_fish_timers[7] += self.lantern_fish_timers[0];
        self.lantern_fish_timers[0]=0;
        Some(
            self.lantern_fish_timers.iter().sum()
        )
    }
}

struct LanternFishPopulationIter {
    lantern_fish_timers: Vec<u32>,
}

impl LanternFishPopulationIter {
    fn new(lantern_fish_timers: Vec<u32>) -> Self {
        Self {
            lantern_fish_timers,
        }
    }
}

impl Iterator for LanternFishPopulationIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut addvals = 0;
        for time in self.lantern_fish_timers.iter_mut() {
            if *time == 0 {
                *time = 6;
                addvals += 1;
            } else {
                *time -= 1;
            }
        }
        for _ in 0..addvals {
            self.lantern_fish_timers.push(8);
        }
        Some(self.lantern_fish_timers.len())
    }
}

pub fn day6_1 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",std::env::var("ADVENT_OF_CODE_2021").unwrap(),file_name);
    let _data = fs::read_to_string(input_file).unwrap();
    let data = _data
        .split(',')
        .map(|stringval| stringval.parse().unwrap())
        .collect::<Vec<u32>>();

     LanternFishPopulationIter::new(data).nth(80 - 1).unwrap()
}

pub fn day6_2 (file_name: &str)->  impl crate::AnsType{
	let input_file = format!("{}/aofc_2021/input/{}",std::env::var("ADVENT_OF_CODE_2021").unwrap(),file_name);
    let _data = fs::read_to_string(input_file);
        let data = _data.unwrap()
        .split(',')
        .map(|stringval| stringval.parse().unwrap())
        .collect::<Vec<u32>>();
        LanternFishPopulation::from(data).nth(256 - 1).unwrap()
}
