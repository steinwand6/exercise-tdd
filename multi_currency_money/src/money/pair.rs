use super::money::Currency;

pub struct Pair {
    from: Currency,
    to: Currency,
}

impl Pair {
    pub fn new(from: Currency, to: Currency) -> Self {
        Pair { from, to }
    }

    pub fn equals(&self, obj: Pair) -> bool {
        self.from == obj.from && self.to == obj.to
    }

    pub fn hash_code(&self) -> i64 {
        0
    }
}
