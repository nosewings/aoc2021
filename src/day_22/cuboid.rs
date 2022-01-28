use super::vector::Vector;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cuboid {
    lo: Vector<i64, 3>,
    hi: Vector<i64, 3>,
}

impl Cuboid {
    pub fn new(lo: Vector<i64, 3>, hi: Vector<i64, 3>) -> Option<Self> {
        if lo.iter().zip(hi.iter()).any(|(l, u)| l > u) {
            None
        } else {
            Some(Self { lo, hi })
        }
    }

    pub fn volume(self) -> i64 {
        (self.hi - self.lo).iter().product()
    }

    pub fn intersect(self, other: Self) -> Option<Self> {
        let lo = self.lo.sup(&other.lo);
        let hi = self.hi.inf(&other.hi);
        Self::new(lo, hi)
    }

    pub fn is_sub_cuboid(self, other: Self) -> bool {
        match self.intersect(other) {
            None => false,
            Some(i) => i == self,
        }
    }
}
