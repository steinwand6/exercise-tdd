use crate::money::money::Money;

use super::money::MoneyTrait;

pub struct Dollar {
    amount: i64,
}

impl Dollar {}

impl MoneyTrait for Dollar {
    fn new(amount: i64) -> Money {
        Money { amount }
    }

    fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
        }
    }

    fn equal(&self, obj: Money) -> bool {
        self.amount == obj.amount
    }
}
