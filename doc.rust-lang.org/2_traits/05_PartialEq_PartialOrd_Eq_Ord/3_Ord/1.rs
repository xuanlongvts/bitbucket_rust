use std::cmp::Ordering;

fn main() {
	assert_eq!(5.cmp(&10), Ordering::Less);
	assert_eq!(10.cmp(&5), Ordering::Greater);
	assert_eq!(5.cmp(&5), Ordering::Equal);

	assert_eq!(2, 1.max(2));
	assert_eq!(2, 2.max(2));

	assert_eq!(1, 1.min(2));
	assert_eq!(2, 2.min(2));

	// fn clamp(self, min: Self, max: Self) -> Self
	// Returns max if self is greater than max, and min if self is less than min. Otherwise this returns self.
	assert!((-3).clamp(-2, 1) == -2);
	assert!(0.clamp(-2, 1) == 0);
	assert!(2.clamp(-2, 1) == 1);
}