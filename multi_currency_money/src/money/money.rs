pub trait MoneyTrait {
    fn new(amount: i64) -> Money;
    fn times(&self, multiplier: i64) -> Money;
    fn equal(&self, obj: Money) -> bool;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    pub amount: i64,
}

impl Money {
    pub fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
        }
    }

    pub fn equal(&self, obj: Money) -> bool {
        self.amount == obj.amount
    }
}
