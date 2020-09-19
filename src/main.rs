use std::time::SystemTime;

use minifb::{Window, WindowOptions};

mod font;

static COLOR_PRESETS: [Color; 12] = [
	Color::new_hex(0xFF0000, 0x000000),
	Color::new_hex(0x00FF00, 0x000000),
	Color::new_hex(0x0000FF, 0x000000),

	Color::new_hex(0xFFFF00, 0x000000),
	Color::new_hex(0x00FFFF, 0x000000),
	Color::new_hex(0xFF00FF, 0x000000),

	Color::new_hex(0xFF8000, 0x000000),
	Color::new_hex(0x00FF80, 0x000000),
	Color::new_hex(0x8000FF, 0x000000),

	Color::new_hex(0xE6003A, 0x000000),
	Color::new_hex(0x3AE600, 0x000000),
	Color::new_hex(0x003AE6, 0x000000),
];

#[derive(Debug, Clone, Copy)]
struct Color {
	pub background: u32,
	pub foreground: u32
}
impl Color {
	pub const fn new_hex(
		background: u32,
		foreground: u32
	) -> Self {
		Color {
			background,
			foreground
		}
	}
}

fn main() {
	let mut args = std::env::args();
	args.next(); // skip exec name
	
	let color = args.next().and_then(|arg| {
		if arg.starts_with("#") {
			u32::from_str_radix(&arg[1..], 16).ok().map(|bg| Color::new_hex(bg, 0x000000))
		} else {
			usize::from_str_radix(&arg, 16).ok().map(|index| COLOR_PRESETS[index % COLOR_PRESETS.len()])
		}
	}).unwrap_or_else(
		|| {
			let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

			let md = COLOR_PRESETS.len() as u64;
			let index = (time.as_secs() % md + time.as_micros() as u64 % md) % md;
			COLOR_PRESETS[index as usize]
		}
	);

	open_window(color);
}

fn open_window(color: Color) {
	let name = format!("window_debug #{:0>6X}", color.background);
	
	let mut size: [usize; 2] = [300, 300];
	let mut window = Window::new(
		&name,
		size[0], size[1],
		WindowOptions {
			borderless: true,
			resize: true,
			..WindowOptions::default()
		}
	).expect("Could not open window");

	let mut buffer: Vec<u32> = Vec::with_capacity(size[0] * size[1]);
	let mut redraw = true;

	while window.is_open() {
		window.update();

		let current_size = window.get_size();
		if current_size.0 != size[0] || current_size.1 != size[1] {
			size[0] = current_size.0;
			size[1] = current_size.1;
			redraw = true;
		}

		if redraw {
			// resize buffer
			let required_pixels = size[0] * size[1];
			if buffer.len() > required_pixels * 2 {
				buffer.truncate(required_pixels);
				buffer.shrink_to_fit();
				buffer.clear();
			} else {
				buffer.clear();
				buffer.reserve(required_pixels);
			}

			update_buffer(&mut buffer, color, size);
			window.update_with_buffer(&buffer, size[0], size[1]).expect("Could not update the window with buffer");

			redraw = false;
		}
	}
}

fn update_buffer(buffer: &mut Vec<u32>, color: Color, size: [usize; 2]) {
	let (width_line, height_line) = {
		let width_text = format!("{}", size[0]);
		let width_glyph_count = width_text.len();

		let height_text = format!("{}", size[1]);
		let height_glyph_count = height_text.len();

		let text_scale = {
			let max_width = std::cmp::max(
				width_glyph_count * font::GLYPH_WIDTH,
				height_glyph_count * font::GLYPH_WIDTH
			) * 2;
			let max_height = font::GLYPH_HEIGHT * 2;

			// perform flooring ln2 to gain power-of-two scaling for with and height
			let width_scale = 0usize.leading_zeros() - (size[0] / max_width).leading_zeros();
			let height_scale = 0usize.leading_zeros() - (size[1] / max_height).leading_zeros();
			
			let min_scale = std::cmp::min(width_scale, height_scale);
			
			let scale = if min_scale > 0 {
				1isize << (min_scale - 1)
			} else {
				1isize
			};

			std::num::NonZeroIsize::new(scale).unwrap()
		};

		let margin = [
			(font::GLYPH_WIDTH / 2) as isize * text_scale.get(),
			(font::GLYPH_BASELINE - font::GLYPH_ASCENT) as isize / 2 * text_scale.get()
		];

		let width_line = font::TextLine::new(
			&width_text,
			[
				size[0] as isize - margin[0] - (font::GLYPH_WIDTH * width_glyph_count) as isize * text_scale.get(),
				margin[1] - (font::GLYPH_ASCENT as isize) * text_scale.get()
			],
			text_scale
		);
		let height_line = font::TextLine::new(
			&height_text,
			[
				margin[0],
				size[1] as isize - margin[1] - font::GLYPH_BASELINE as isize * text_scale.get()
			],
			text_scale
		);

		(width_line, height_line)
	};
	
	// recalcualte buffer
	for y in 0 .. size[1] as isize {
		for x in 0 .. size[0] as isize {
			let covers_width = width_line.covers(x, y);
			let covers_height = height_line.covers(x, y);

			if covers_width || covers_height {
				buffer.push(color.foreground)
			} else {
				buffer.push(color.background)
			}
		}
	}
}