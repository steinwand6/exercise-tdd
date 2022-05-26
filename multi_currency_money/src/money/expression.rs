use super::bank::Bank;
use super::money::{Currency, Money};

pub trait Expression {
    fn reduce(&self, bank: &Bank, to: &Currency) -> Money;
}
