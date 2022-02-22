use std::{cmp::Ordering, fmt::Display, ops::Add};
use DijkstraDistance::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum DijkstraDistance {
    Infinity,
    Distance(u32),
}

impl PartialOrd for DijkstraDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DijkstraDistance::Infinity, DijkstraDistance::Infinity) => Ordering::Equal,
            (DijkstraDistance::Infinity, DijkstraDistance::Distance(_)) => Ordering::Greater,
            (DijkstraDistance::Distance(_), DijkstraDistance::Infinity) => Ordering::Less,
            (DijkstraDistance::Distance(self_val), DijkstraDistance::Distance(other_val)) => {
                self_val.cmp(other_val)
            }
        }
    }
}

impl Default for DijkstraDistance {
    fn default() -> Self {
        Self::Infinity
    }
}

impl Add for DijkstraDistance {
    type Output = DijkstraDistance;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Distance(v1), Distance(v2)) => Distance(v1 + v2),
            _ => Infinity,
        }
    }
}

impl Add<u32> for DijkstraDistance {
    type Output = DijkstraDistance;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Infinity => Infinity,
            Distance(v) => Distance(v + rhs),
        }
    }
}
impl Add<u32> for &DijkstraDistance {
    type Output = DijkstraDistance;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            Infinity => Infinity,
            Distance(v) => Distance(v + rhs),
        }
    }
}

impl Display for DijkstraDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Infinity => write!(f, "Infinity"),
            Distance(val) => write!(f, "{}", val),
        }
    }
}
