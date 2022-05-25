use super::expression::Expression;
use super::money::{Currency, Money};

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
