pub trait MoneyTrait {
    fn new(amount: i64) -> Money;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    pub amount: i64,
}

impl Money {
    pub fn dollar(amount: i64) -> Money {
        Money { amount }
    }

    pub fn franc(amount: i64) -> Money {
        Money { amount }
    }

    pub fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
        }
    }

    pub fn equal(&self, obj: Money) -> bool {
        self.amount == obj.amount
    }
}
