use super::bank::Bank;
use super::expression::Expression;
use super::sum::Sum;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Currency {
    USD,
    CHF,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    pub(crate) amount: i64,
    pub(crate) currency: Currency,
}

impl Expression for Money {
    fn reduce(&self, bank: &Bank, to: &Currency) -> Money {
        let rate = bank.rate(&self.currency, to);
        Money {
            amount: self.amount / rate,
            currency: to.clone(),
        }
    }
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

    pub fn plus(&self, addend: &Money) -> Sum {
        Sum::new(
            Money {
                amount: self.amount,
                currency: self.currency.clone(),
            },
            Money {
                amount: addend.amount,
                currency: addend.currency.clone(),
            },
        )
    }
}
