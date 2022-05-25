use crate::money::money::{Currency, Expression, Money};

#[derive(Debug, Eq, PartialEq)]
pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Bank {}
    }

    pub fn reduce<T: Expression>(&self, source: T, to: Currency) -> Money {
        source.reduce(to)
    }
}
