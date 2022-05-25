#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Currency {
    USD,
    CHF,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

pub trait Expression {
    fn reduce(&self, to: Currency) -> Money;
}

impl Expression for Money {
    fn reduce(&self, _: Currency) -> Money {
        Money {
            amount: self.amount,
            currency: self.currency.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sum {
    pub addend: Money,
    pub augend: Money,
}

impl Expression for Sum {
    fn reduce(&self, to: Currency) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money {
            amount,
            currency: to,
        }
    }
}

impl Sum {
    pub fn new(augend: Money, addend: Money) -> Self {
        Sum { augend, addend }
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

#[cfg(test)]
mod tests {
    use super::Currency;
    use super::Money;
    use super::Sum;
    use crate::bank::bank::Bank;

    #[test]
    fn test_reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let result = bank.reduce(sum, Currency::USD);
        assert_eq!(result, Money::dollar(7));
    }
}
