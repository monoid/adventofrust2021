use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Debug)]
pub struct LRange(pub RangeInclusive<i32>);

impl PartialOrd for LRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.0
                .clone()
                .last()
                .unwrap()
                .cmp(&other.0.clone().last().unwrap()),
        )
    }
}

impl Ord for LRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.end().cmp(other.0.clone().end())
    }
}

impl LRange {
    pub fn intersects(&self, other: &LRange) -> bool {
        // Please note that adjacent range are considered intersecting: that's why
        // we add 1.
        !(self.0.end() + 1 < *other.0.start() || other.0.end() + 1 < *self.0.start())
    }

    fn join(&mut self, other: &LRange) {
        let new_range = *std::cmp::min(self.0.start(), other.0.start())
            ..=*std::cmp::max(self.0.end(), other.0.end());
        self.0 = new_range;
    }

    pub fn start(&self) -> i32 {
        *self.0.start()
    }

    pub fn end(&self) -> i32 {
        *self.0.end()
    }
}

#[derive(Default, Debug)]
pub struct RangeSet(Vec<LRange>);

impl RangeSet {
    pub fn insert(&mut self, range: RangeInclusive<i32>) {
        let mut lrange = LRange(range);
        let mut orig = vec![];
        std::mem::swap(&mut orig, &mut self.0);

        for lprev in orig {
            if lrange.intersects(&lprev) {
                lrange.join(&lprev);
            } else {
                self.0.push(lprev);
            }
        }

        let idx = self.0.binary_search(&lrange).unwrap_or_else(|v| v);
        self.0.insert(idx, lrange);
    }

    pub fn into_inner(self) -> Vec<LRange> {
        self.0
    }
}
