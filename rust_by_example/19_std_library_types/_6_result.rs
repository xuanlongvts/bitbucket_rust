mod checked {
	// Mathematical "errors" we want to catch
	#[derive(Debug)]
	pub enum MathError {
		DivisionByZero,
		NonPositiveLogarithm,
		NegativeSquareRoot,
	}

	pub type MathResult = Result<f64, MathError>;

	pub fn div(x: f64, y: f64) -> MathResult {
		if y == 0.0 {
			Err(MathError::DivisionByZero)
		} else {
			Ok(x / y)
		}
	}

	pub fn sqrt(x: f64) -> MathResult {
		if x < 0.0 {
			Err(MathError::NegativeSquareRoot)
		} else {
			Ok(x.sqrt())
		}
	}

	pub fn ln(x: f64) -> MathResult {
		if x <= 0.0 {
			Err(MathError::NonPositiveLogarithm)
		} else {
			Ok(x.ln())
		}
	}
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
	// This is a three level match pyramid!
	match checked::div(x, y) {
		Err(why) => panic!("{:?}", why),
		Ok(ratio) => {
			println!("ratio: {:?}", ratio);
			println!("=====================");

			return match checked::ln(ratio) {
				Err(why) => panic!("{:?}", why),
				Ok(ln) => {
					println!("ln: {:?}", ln);
					println!("=====================");
					
					match checked::sqrt(ln) {
						Err(why) => panic!("Error: =====> {:?}", why),
						Ok(value_sqrt) => value_sqrt
					}
				}
			}
		}
	}
}

fn main() {
	println!("{}", op(1.0, 10.0));
}