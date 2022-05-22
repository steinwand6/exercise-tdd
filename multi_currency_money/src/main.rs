// todo
// $5 + 10CHF = $10 (if rate is 2:1)
// $5 * 2 = $10
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_mutlipulication() {
	Dollar five = Dollar::new(5);
	five.times(2);
	assert_eq!(10, five.amount);
}
