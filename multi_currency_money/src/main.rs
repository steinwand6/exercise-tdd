// * todo
// [ ] $5 + 10CHF = $10 (if rate is 2:1)
// [x] $5 * 2 = $10
// [ ] make amount private
// [ ] what to do with Dollar side effect
// [ ] how to round Money

fn main() {
    println!("Hello, world!");
}

struct Dollar {
    amount: i64,
}

impl Dollar {
    fn new(amount: i64) -> Self {
        Dollar { amount }
    }
    fn times(&self, multiplier: i64) -> Self {
        Dollar {
            amount: self.amount * multiplier,
        }
    }
}

#[cfg(test)]
mod MoneyTest {
    use crate::Dollar;

    #[test]
    fn test_mutlipulication() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
