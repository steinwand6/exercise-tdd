use super::money::{Currency, Money};

pub trait Expression {
    fn reduce(&self, to: Currency) -> Money;
}
