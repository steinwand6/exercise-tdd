pub trait MoneyTrait {
    fn new(amount: i64) -> Money;
    fn currency() -> String;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    pub amount: i64,
    currency: String,
}

impl Money {
    pub fn dollar(amount: i64) -> Money {
        Money {
            amount,
            currency: "USD".to_string(),
        }
    }

    pub fn franc(amount: i64) -> Money {
        Money {
            amount,
            currency: "CHF".to_string(),
        }
    }

    pub fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency.clone(),
        }
    }

    pub fn equal(&self, obj: Money) -> bool {
        self.amount == obj.amount
    }
}
