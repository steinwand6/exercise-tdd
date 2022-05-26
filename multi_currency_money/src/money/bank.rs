use std::collections::HashMap;

use super::expression::Expression;
use super::money::{Currency, Money};
use super::pair::Pair;

#[derive(Debug, Eq, PartialEq)]
pub struct Bank {
    rates: HashMap<Pair, i64>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            rates: HashMap::<Pair, i64>::new(),
        }
    }

    pub fn reduce<T: Expression>(&self, source: T, to: &Currency) -> Money {
        source.reduce(self, to)
    }

    pub fn add_rate(&mut self, from: &Currency, to: &Currency, rate: i64) {
        self.rates.insert(Pair::new(from, to), rate);
    }

    pub fn rate(&self, from: &Currency, to: &Currency) -> i64 {
        match from {
            Currency::CHF => {
                if to == &Currency::USD {
                    2
                } else {
                    1
                }
            }
            _ => 1,
        }
    }
}
