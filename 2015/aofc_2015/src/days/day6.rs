use std::fs;

struct LightGrid {
    lights: [[u32; 1000]; 1000],
}

impl LightGrid {
    pub fn new() -> LightGrid {
        LightGrid {
            lights: [[0; 1000]; 1000],
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.lights[x][y] = if self.lights[x][y] == 0 { 1 } else { 0 };
    }
    pub fn on(&mut self, x: usize, y: usize) {
        self.lights[x][y] = 1;
    }
    pub fn off(&mut self, x: usize, y: usize) {
        self.lights[x][y] = 0;
    }
    pub fn inc2(&mut self, x: usize, y: usize) {
        self.lights[x][y] += 2;
    }
    pub fn inc(&mut self, x: usize, y: usize) {
        self.lights[x][y] += 1;
    }
    pub fn dec(&mut self, x: usize, y: usize) {
        if self.lights[x][y] > 0 {
            self.lights[x][y] -= 1;
        }
    }

    pub fn toggle_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.toggle(x, y);
            }
        }
    }
    pub fn on_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.on(x, y);
            }
        }
    }
    pub fn off_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.off(x, y);
            }
        }
    }

    pub fn inc2_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.inc2(x, y);
            }
        }
    }
    pub fn inc_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.inc(x, y);
            }
        }
    }
    pub fn dec_range(&mut self, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.dec(x, y);
            }
        }
    }

    pub fn count_on(&self) -> usize {
        self.lights
            .iter()
            .map(|sub| {
                sub.iter()
                    // .map(|num| if *num > 0 { 1 } else { 0 })
                    .map(|n| *n as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

pub fn day6_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let mut light_grid = LightGrid::new();

    let instructions = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.strip_prefix("turn ")
                .unwrap_or(l)
                .split(' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for ins in instructions {
        let a = ins[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let b = ins[3]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        match ins[0] {
            "on" => light_grid.on_range((a[0], a[1]), (b[0], b[1])),
            "off" => light_grid.off_range((a[0], a[1]), (b[0], b[1])),
            "toggle" => light_grid.toggle_range((a[0], a[1]), (b[0], b[1])),
            _ => unreachable!(),
        }
    }
    light_grid.count_on()
}

pub fn day6_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "/home/fuzzy/DevSSD/advent-of-code/2015/aofc_2015/input/{}",
        file_name
    );
    let data = fs::read_to_string(input_file).unwrap();
    let mut light_grid = LightGrid::new();

    let instructions = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.strip_prefix("turn ")
                .unwrap_or(l)
                .split(' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for ins in instructions {
        let a = ins[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let b = ins[3]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        match ins[0] {
            "on" => light_grid.inc_range((a[0], a[1]), (b[0], b[1])),
            "off" => light_grid.dec_range((a[0], a[1]), (b[0], b[1])),
            "toggle" => light_grid.inc2_range((a[0], a[1]), (b[0], b[1])),
            _ => unreachable!(),
        }
    }
    light_grid.count_on()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_ansval;

    #[test]
    #[ignore]
    fn t1() {
        assert_eq_ansval!((), day6_1("test"));
    }
    #[test]
    #[ignore]
    fn t2() {
        assert_eq_ansval!((), day6_2("test"));
    }
}
