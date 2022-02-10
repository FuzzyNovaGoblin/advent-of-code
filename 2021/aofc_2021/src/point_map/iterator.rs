use super::CordPoint;


#[derive(Debug)]
pub struct DimentionIter {
    point: CordPoint,
    dimentions: CordPoint,
}

impl DimentionIter {
    pub fn new(dimentions: CordPoint) -> Self {
        Self {
            point: (0, 0),
            dimentions,
        }
    }
}

impl Iterator for DimentionIter {
    type Item = CordPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.point;
        if ret.0 >= self.dimentions.0 || ret.1 >= self.dimentions.1 {
            return None;
        }

        self.point.0 += 1;
        if self.point.0 >= self.dimentions.0 {
            self.point.0 = 0;
            self.point.1 += 1;
        }
        Some(ret)
    }
}
