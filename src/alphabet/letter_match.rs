pub mod letter {
	use super::super::pixelated_letters::alpha_dict::*;

	pub fn eq(ltr: &str) {
		match ltr {
			"a" => println!("{:#?}", px_a()),
			_ => {}
		}
	}
}
