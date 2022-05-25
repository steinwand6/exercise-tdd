#[derive(Debug, Eq, PartialEq, Clone)]
enum Currency {
    USD,
    CHF,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

pub trait Expression {}

impl Expression for Money {}

#[derive(Debug, Eq, PartialEq)]
pub struct Sum {
    pub addend: Money,
    pub augend: Money,
}

impl Expression for Sum {}

impl Money {
    pub fn dollar(amount: i64) -> Money {
        Money {
            amount,
            currency: Currency::USD,
        }
    }

    pub fn franc(amount: i64) -> Money {
        Money {
            amount,
            currency: Currency::CHF,
        }
    }

    pub fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency.clone(),
        }
    }

    pub fn equal(&self, obj: Money) -> bool {
        if self.currency != obj.currency {
            return false;
        }
        self.amount == obj.amount
    }

    pub fn plus(self, addend: Money) -> Sum {
        Sum {
            augend: self,
            addend,
        }
    }
}
