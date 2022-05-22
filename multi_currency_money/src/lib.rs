mod dollar {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Dollar {
        amount: i64,
    }

    impl Dollar {
        pub fn new(amount: i64) -> Self {
            Dollar { amount }
        }

        pub fn times(&self, multiplier: i64) -> Self {
            Dollar {
                amount: self.amount * multiplier,
            }
        }

        pub fn equal(&self, obj: Dollar) -> bool {
            self.amount == obj.amount
        }
    }
}
#[cfg(test)]
mod dollar_test {
    use crate::dollar::Dollar;

    #[test]
    fn test_mutlipulication() {
        let five = Dollar::new(5);
        assert_eq!(five.times(2), Dollar::new(10));
        assert_eq!(five.times(3), Dollar::new(15));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equal(Dollar::new(5)));
        assert!(!Dollar::new(5).equal(Dollar::new(6)));
    }
}
