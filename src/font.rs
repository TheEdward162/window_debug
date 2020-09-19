use std::num::NonZeroIsize;

pub const GLYPH_WIDTH: usize = 8;
pub const GLYPH_HEIGHT: usize = 16;
pub const GLYPH_BASELINE: usize = 12;
pub const GLYPH_ASCENT: usize = 2;

/// Font glyph is an 8x16 bitmask.
#[derive(Copy, Clone, PartialEq)]
pub struct FontGlyph {
	mask: u128
}
impl FontGlyph {
	pub const fn new(
		mask_lines: [u8; GLYPH_HEIGHT]
	) -> Self {
		let mut mask: u128 = 0;
		
		macro_rules! add_lines {
			(
				$(
					$line: literal
				),+
			) => {
				$(
					mask |= (mask_lines[$line] as u128) << (
						(GLYPH_HEIGHT - 1 - $line) * GLYPH_WIDTH
					);
				)+
			}
		}

		add_lines!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

		FontGlyph {
			mask
		}
	}

	pub const fn new_fit_xheight(
		mask_lines: [u8; 10]
	) -> Self {
		Self::new(
			[
				0,
				0,
				mask_lines[0],
				mask_lines[1],
				mask_lines[2],
				mask_lines[3],
				mask_lines[4],
				mask_lines[5],
				mask_lines[6],
				mask_lines[7],
				mask_lines[8],
				mask_lines[9],
				0,
				0,
				0,
				0
			]
		)
	}

	pub const fn covers(&self, x: isize, y: isize) -> bool {
		if x < 0 || y < 0 {
			return false;
		}
		let x = x as usize;
		let y = y as usize;

		if x >= GLYPH_WIDTH || y >= GLYPH_HEIGHT {
			return false;
		}
		let shift = y * GLYPH_WIDTH + x;

		(
			self.mask >> (GLYPH_WIDTH * GLYPH_HEIGHT - 1 - shift)
		) & 0b1 == 1
	}

	pub fn map_char(ch: char) -> Self {
		match ch {
			'0' => FontGlyph::NUM_0,
			'1' => FontGlyph::NUM_1,
			'2' => FontGlyph::NUM_2,
			'3' => FontGlyph::NUM_3,
			'4' => FontGlyph::NUM_4,
			'5' => FontGlyph::NUM_5,
			'6' => FontGlyph::NUM_6,
			'7' => FontGlyph::NUM_7,
			'8' => FontGlyph::NUM_8,
			'9' => FontGlyph::NUM_9,
			_ => panic!("Char '{}' not implemented in the font", ch)
		}
	}
}
impl FontGlyph {
	pub const NUM_0: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b01111100,
		]
	);
	pub const NUM_1: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b00011000,
			0b01111000,
			0b00011000,
			0b00011000,
			0b00011000,
			0b00011000,
			0b00011000,
			0b00011000,
			0b00011000,
			0b01111110,
		]
	);
	pub const NUM_2: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b11000110,
			0b00000110,
			0b00001100,
			0b00011000,
			0b00110000,
			0b01100000,
			0b11000110,
			0b11111110,
		]
	);
	pub const NUM_3: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b00000110,
			0b00000110,
			0b00111100,
			0b00000110,
			0b00000110,
			0b00000110,
			0b11000110,
			0b01111100,
		]
	);
	pub const NUM_4: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b00001100,
			0b00011100,
			0b00111100,
			0b01101100,
			0b11001100,
			0b11001100,
			0b11111110,
			0b00001100,
			0b00001100,
			0b00011110,
		]
	);
	pub const NUM_5: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b11111110,
			0b11000000,
			0b11000000,
			0b11000000,
			0b11111100,
			0b00000110,
			0b00000110,
			0b00000110,
			0b11000110,
			0b01111100,
		]
	);
	pub const NUM_6: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b11000000,
			0b11000000,
			0b11111100,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b01111100,
		]
	);
	pub const NUM_7: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b11111110,
			0b11000110,
			0b00000110,
			0b00001100,
			0b00011000,
			0b00110000,
			0b00110000,
			0b00110000,
			0b00110000,
			0b00110000,
		]
	);
	pub const NUM_8: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b11000110,
			0b11000110,
			0b01111100,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b01111100,
		]
	);
	pub const NUM_9: FontGlyph = FontGlyph::new_fit_xheight(
		[
			0b01111100,
			0b11000110,
			0b11000110,
			0b11000110,
			0b11000110,
			0b01111110,
			0b00000110,
			0b00000110,
			0b11000110,
			0b01111100,
		]
	);
}
impl std::fmt::Debug for FontGlyph {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		macro_rules! fmt {
			(
				$($index: literal),+
			) => {
				write!(
					f,
					"\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n{:0>8b}\n",
					$(
						((self.mask >> (GLYPH_HEIGHT - 1 - $index) * GLYPH_WIDTH) & 0xFF) as u8
					),+
				)
			}
		}

		fmt!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
	}
}

/// Positioned and scaled FontGlyph.
#[derive(Debug, Copy, Clone)]
pub struct WorldFontGlyph {
	position: [isize; 2],
	scale: [NonZeroIsize; 2],
	glyph: FontGlyph
}
impl WorldFontGlyph {
	pub const fn new(
		position: [isize; 2],
		scale: NonZeroIsize,
		glyph: FontGlyph
	) -> Self {
		WorldFontGlyph {
			position,
			scale: [scale, scale],
			glyph
		}
	}

	pub const fn covers(&self, x: isize, y: isize) -> bool {
		let relative_x = x - self.position[0];
		let relative_y = y - self.position[1];

		// Example: -3 / 4 == 0 but we don't want to cover that negative side
		if relative_x < 0 || relative_y < 0 {
			return false;
		}
		
		self.glyph.covers(
			relative_x / self.scale[0].get(),
			relative_y / self.scale[1].get()
		)
	}
}

pub struct TextLine {
	glyphs: Vec<WorldFontGlyph>,
	bounding_box: [isize; 4]
}
impl TextLine {
	pub fn new(
		text: &str,
		base_position: [isize; 2],
		scale: std::num::NonZeroIsize
	) -> Self {
		let glyphs = text.chars().enumerate().map(
			|(i, ch)| {
				let glyph = FontGlyph::map_char(ch);
				
				WorldFontGlyph::new(
					[
						base_position[0] + (i * GLYPH_WIDTH) as isize * scale.get(),
						base_position[1]
					],
					scale,
					glyph
				)
			}
		).collect::<Vec<_>>();

		let bounding_box = [
			base_position[0],
			base_position[0] + (glyphs.len() * GLYPH_WIDTH) as isize * scale.get(),
			base_position[1],
			base_position[1] + GLYPH_HEIGHT as isize * scale.get()
		];

		TextLine {
			glyphs,
			bounding_box
		}
	}

	pub fn covers(&self, x: isize, y: isize) -> bool {
		if x < self.bounding_box[0] || x > self.bounding_box[1] {
			return false;
		}
		if y < self.bounding_box[2] || y > self.bounding_box[3] {
			return false;
		}

		self.glyphs.iter().any(|g| g.covers(x, y))
	}
}