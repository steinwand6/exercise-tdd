#[cfg(test)]
pub mod money_tests {
    use crate::money::bank::Bank;
    use crate::money::money::{Currency, Money};
    use crate::money::sum::Sum;

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
        let sum = five.plus(&Money::dollar(5));
        let reduced = bank.reduce(sum, &Currency::USD);
        assert_eq!(reduced, Money::dollar(10));
    }

    #[test]
    fn test_plus_returns_sum() {
        let five = Money::dollar(5);
        let sum = five.plus(&five);
        assert_eq!(Money::dollar(5), sum.augend);
        assert_eq!(Money::dollar(5), sum.addend);
    }

    #[test]
    fn test_reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let result = bank.reduce(sum, &Currency::USD);
        assert_eq!(result, Money::dollar(7));
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(&Currency::CHF, &Currency::USD, 2);
        let result = bank.reduce(Money::franc(2), &Currency::USD);
        assert_eq!(result, Money::dollar(1));
    }

    #[test]
    fn test_mixed_addtion() {
        let five_bucks = Money::dollar(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(&Currency::CHF, &Currency::USD, 2);
        let result = bank.reduce(five_bucks.plus(&ten_francs), &Currency::USD);
        assert_eq!(result, Money::dollar(10));
    }
}
