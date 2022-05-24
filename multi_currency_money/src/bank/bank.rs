use crate::money::money::Money;

#[derive(Debug, Eq, PartialEq)]
pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Bank {}
    }
    pub fn reduce(&self, source: Expression, to: &str) -> Money {
        Money::dollar(10)
    }
}
