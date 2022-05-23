use crate::money::money::Money;

use super::money::MoneyTrait;

pub struct Dollar {}

impl Dollar {}

impl MoneyTrait for Dollar {
    fn new(amount: i64) -> Money {
        Money { amount }
    }
}
