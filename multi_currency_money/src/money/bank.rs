use super::expression::Expression;
use super::money::{Currency, Money};

#[derive(Debug, Eq, PartialEq)]
pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Bank {}
    }

    pub fn reduce<T: Expression>(&self, source: T, to: Currency) -> Money {
        source.reduce(to)
    }

    pub fn add_rate(&self, from: Currency, to: Currency, rate: i32) {}
}
