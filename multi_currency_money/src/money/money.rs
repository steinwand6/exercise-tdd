#[derive(Debug, Eq, PartialEq, Clone)]
enum Currency {
    USD,
    CHF,
}

pub trait MoneyTrait {
    fn new(amount: i64) -> Money;
    fn currency() -> String;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    pub amount: i64,
    currency: Currency,
}

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

    pub fn plus(&self, addend: Money) -> Money {
        Money {
            amount: self.amount + addend.amount,
            currency: self.currency.clone(),
        }
    }
}
