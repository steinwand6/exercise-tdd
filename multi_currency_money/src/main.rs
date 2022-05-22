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
    fn times(&mut self, multiplier: i64) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod MoneyTest {
    use crate::Dollar;

    #[test]
    fn test_mutlipulication() {
        let mut five = Dollar::new(5);
        product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
