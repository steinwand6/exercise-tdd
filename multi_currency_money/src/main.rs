// * todo
// [] $5 + 10CHF = $10 (if rate is 2:1)
// [] $5 * 2 = $10
// [] make amount private
// [] what to do with Dollar side effect
// [] how to round Money

fn main() {
    println!("Hello, world!");
}

struct Dollar {}

impl Dollar {
    fn new(amount: i64) -> Self {
        Dollar {}
    }
}

#[cfg(test)]
mod MoneyTest {
    use crate::Dollar;

    #[test]
    fn test_mutlipulication() {
        let five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
