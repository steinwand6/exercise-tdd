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

mod dollar;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod money_test {
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
