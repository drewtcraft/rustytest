pub mod pl {
	use std::fmt;

	pub struct PixelLetter {
		pub center: ( f32, f32 ),
		pub fill_map: [ bool; 25 ]
	}

	impl fmt::Debug for PixelLetter {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f,
				"
  PixelLetter:
    center: ({}, {}),
    fill_map: [
      {}, {}, {}, {}, {},
      {}, {}, {}, {}, {},
      {}, {}, {}, {}, {},
      {}, {}, {}, {}, {},
      {}, {}, {}, {}, {},
    ]
				",
				self.center.0,
				self.center.1,
				self.fill_map[0], 	self.fill_map[1], 	self.fill_map[2], 	self.fill_map[3],	self.fill_map[4],
				self.fill_map[5], 	self.fill_map[6], 	self.fill_map[7], 	self.fill_map[8],	self.fill_map[9],
		 		self.fill_map[10], 	self.fill_map[11], 	self.fill_map[12], 	self.fill_map[13],	self.fill_map[14],
				self.fill_map[15], 	self.fill_map[16], 	self.fill_map[17], 	self.fill_map[18],	self.fill_map[19],
				self.fill_map[20], 	self.fill_map[21], 	self.fill_map[22], 	self.fill_map[23],	self.fill_map[24],
			)
		}
	}
}
