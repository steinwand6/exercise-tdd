use super::bank::Bank;
use super::expression::Expression;
use super::money::{Currency, Money};

#[derive(Debug, Eq, PartialEq)]
pub struct Sum {
    pub addend: Money,
    pub augend: Money,
}

impl Expression for Sum {
    fn reduce(&self, _: &Bank, to: &Currency) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money {
            amount,
            currency: to.clone(),
        }
    }
}

impl Sum {
    pub fn new(augend: Money, addend: Money) -> Self {
        Sum { augend, addend }
    }
}
