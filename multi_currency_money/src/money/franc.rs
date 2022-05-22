#[derive(Debug, Eq, PartialEq)]
pub struct Franc {
    amount: i64,
}

impl Franc {
    pub fn new(amount: i64) -> Self {
        Franc { amount }
    }

    pub fn times(&self, multiplier: i64) -> Self {
        Franc {
            amount: self.amount * multiplier,
        }
    }

    pub fn equal(&self, obj: Franc) -> bool {
        self.amount == obj.amount
    }
}
