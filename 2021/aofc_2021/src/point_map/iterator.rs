use super::CordPointTuple;


#[derive(Debug)]
pub struct DimentionIter {
    point: CordPointTuple,
    dimentions: CordPointTuple,
}

impl DimentionIter {
    pub fn new(dimentions: CordPointTuple) -> Self {
        Self {
            point: (0, 0),
            dimentions,
        }
    }
}

impl Iterator for DimentionIter {
    type Item = CordPointTuple;

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
