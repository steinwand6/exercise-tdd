use super::money::{Money, MoneyTrait};

#[derive(Debug, Eq, PartialEq)]
pub struct Franc {}

impl MoneyTrait for Franc {
    fn new(amount: i64) -> Money {
        Money { amount }
    }
}
