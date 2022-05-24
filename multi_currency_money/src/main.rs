// * todo
// [ ] $5 + 10CHF = $10 (if rate is 2:1)
// [x] $5 * 2 = $10
// [x] make amount private
// [x] what to do with Dollar side effect
// [ ] how to round Money
// [x] equal()
// [ ] hash_code()
// [ ] equality comparison with NULL
// [ ] equality comparison with Other Object
// [x] 5CHF * 2 = 10CHF
// [x] overlapping dollar and franc
// [x] generalization of equal()
// [x] generalization of times()
// [x] Compare Dollar and Franc
// [x] idea of currency

mod bank;
mod money;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod money_test {
    use crate::bank::bank::Bank;
    use crate::money::money::Money;

    #[test]
    fn test_mutlipulication() {
        let five = Money::dollar(5);
        assert_eq!(five.times(2), Money::dollar(10));
        assert_eq!(five.times(3), Money::dollar(15));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).equal(Money::dollar(5)));
        assert!(!Money::dollar(5).equal(Money::dollar(6)));
        assert!(!Money::franc(5).equal(Money::dollar(5)));
    }

    #[test]
    fn test_simple_addition() {
        let bank = Bank::new();
        let five = Money::dollar(5);
        let sum = five.plus(Money::dollar(5));
        let reduced = Bank::reduce(sum, "USD");
        assert_eq!(sum, Money::dollar(10));
    }
}
