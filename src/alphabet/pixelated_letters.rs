pub mod alpha_dict {
	use super::super::pixel_letter::pl;

	pub fn px_a() -> pl::PixelLetter {
		let x = pl::PixelLetter {
			center: ( 0.0, 0.0 ),
			fill_map: [
				false, 	false, 	true, 	false, 	false,
				false, 	true, 	false, 	true, 	false,
				true, 	false, 	false, 	false, 	true,
				true, 	true, 	true, 	true, 	true,
				true, 	false, 	false, 	false, 	true
			]
		};
		x
	}
}
