//! This portion will test your familiarity with some of Rust's common traits and your ability to
//! implement those traits in interesting ways. You need to remove all of the `todo!()`s. Most of
//! them will need to be replaced by some code, but some may simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.
use std::cmp::Ordering;

/// A record of an employee at a particular company
#[derive(Debug)]
pub struct Employee {
	/// The name the person likes to be called. Doesn't have to be their "passport name"
	pub name: String,
	/// Amount of experience (in months) the person has working at this company
	pub experience: u32,
	/// Hourly wage paid to this employee
	pub wage: u32,
	/// Unique identifier for this employee
	pub uid: u32,
}

// We want to consider two employee instances equal iff they have the same `uid`.

impl PartialEq for Employee {
	fn eq(&self, other: &Self) -> bool {
		//todo!("complete the implementation");
		self.uid == other.uid
	}
}
impl Eq for Employee {}

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.

impl PartialOrd for Employee {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		//todo!("complete the implementation");
		Some(self.cmp(other))
	}
}

impl Ord for Employee {
	fn cmp(&self, other: &Self) -> Ordering {
		//todo!("complete the implementation");
		let calc_value = |e: &Self| e.experience / e.wage;
		match (self, other) {
			(e1, e2) if (e1.uid == e2.uid) => Ordering::Equal,
			(e1, e2) => calc_value(e1).cmp(&calc_value(e2)),
		}
	}
}

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings with the wrong number of commas or numbers too big for `u32` should return `Err(_)`
// where the error message may be anything.

impl std::convert::TryFrom<String> for Employee {
	type Error = &'static str;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		//todo!("complete the implementation");
		match value.split(',').collect::<Vec<&str>>().as_slice() {
			[name, exp, wage, uid] => Ok(Employee {
				name: name.trim().to_string(),
				experience: exp.trim().parse().ok().ok_or("Invalid experience value")?,
				wage: wage.trim().parse().ok().ok_or("Invalid wage value")?,
				uid: uid.trim().parse().ok().ok_or("Invalid uid value")?,
			}),
			_ => Err("Wrong number of arguments or bad format"),
		}
	}
}

// We also want to convert employees back into strings in the same format as above.
impl From<Employee> for String {
	fn from(e: Employee) -> Self {
		//todo!("complete the implementation");
		format!("{}, {}, {}, {}", e.name, e.experience, e.wage, e.uid)
	}
}

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	//todo!()
	150u8
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	//todo!()
	1u8
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn employee_from_string_success() {
		let s = String::from("Billy, 4, 5, 345");
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};

		assert_eq!(billy, s.try_into().unwrap())
	}

	#[test]
	fn employee_to_string_success() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};

		assert_eq!(String::from("Billy, 4, 5, 345"), String::from(billy))
	}

	#[test]
	fn employee_ord() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};
		let susie = Employee {
			name: String::from("Susie"),
			experience: 5,
			wage: 5,
			uid: 347,
		};

		assert!(susie > billy);
	}

	#[test]
	fn employee_neq() {
		let billy = Employee {
			name: String::from("Billy"),
			experience: 4,
			wage: 5,
			uid: 345,
		};
		let susie = Employee {
			name: String::from("Susie"),
			experience: 5,
			wage: 5,
			uid: 347,
		};

		assert!(susie != billy);
	}
}
