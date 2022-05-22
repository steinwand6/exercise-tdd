#[derive(Debug, Eq, PartialEq)]
pub struct Dollar {
    amount: i64,
}

impl Dollar {
    pub fn new(amount: i64) -> Self {
        Dollar { amount }
    }

    pub fn times(&self, multiplier: i64) -> Self {
        Dollar {
            amount: self.amount * multiplier,
        }
    }

    pub fn equal(&self, obj: Dollar) -> bool {
        self.amount == obj.amount
    }
}
