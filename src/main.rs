use std::time::SystemTime;
use std::convert::TryInto;

use minifb::{Window, WindowOptions};

mod font;

use font::TextLine;

static COLOR_PRESETS: [Color; 12] = [
	Color::new_hex(0xFF0000, 0x000000, Some("Red")),
	Color::new_hex(0xFF8000, 0x000000, Some("Orange")),
	Color::new_hex(0xFFFF00, 0x000000, Some("Yellow")),
	Color::new_hex(0x00FF80, 0x000000, Some("Light green")),
	Color::new_hex(0x00FF00, 0x000000, Some("Green")),
	Color::new_hex(0x00B300, 0x000000, Some("Dark green")),
	Color::new_hex(0x00FFFF, 0x000000, Some("Cyan")),
	Color::new_hex(0x0000FF, 0x000000, Some("Blue")),
	Color::new_hex(0x0000B3, 0x000000, Some("Dark blue")),
	Color::new_hex(0x8000FF, 0x000000, Some("Purpleish blue")),
	Color::new_hex(0xFF00FF, 0x000000, Some("Purple")),
	Color::new_hex(0xE6003A, 0x000000, Some("Pink")),
];

#[derive(Debug, Clone, Copy)]
struct Color {
	pub background: u32,
	pub foreground: u32,
	pub name: Option<&'static str>
}
impl Color {
	pub const fn new_hex(
		background: u32,
		foreground: u32,
		name: Option<&'static str>
	) -> Self {
		Color {
			background,
			foreground,
			name
		}
	}
}
impl std::fmt::Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.name {
			Some(name) => write!(f, "{}", name),
			None => write!(f, "#{:0>6X}", self.background)
		}
	}
}

fn main() {
	let mut args = std::env::args();
	args.next(); // skip exec name
	
	let color = args.next().and_then(|arg| {
		if arg.starts_with("#") {
			u32::from_str_radix(&arg[1..], 16).ok().map(|bg| Color::new_hex(bg, 0x000000, None))
		} else {
			usize::from_str_radix(&arg, 10).ok().map(|index| COLOR_PRESETS[index % COLOR_PRESETS.len()])
		}
	}).unwrap_or_else(
		|| {
			let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

			let md = COLOR_PRESETS.len() as u64;
			let index = (time.as_secs() % md + time.as_micros() as u64 % md) % md;
			COLOR_PRESETS[index as usize]
		}
	);

	open_window(
		color,
		WindowOptions {
			borderless: true,
			resize: true,
			..WindowOptions::default()
		},
		None
	);
}

fn open_window(color: Color, window_options: WindowOptions, title_override: Option<&str>) {
	let name = format!("window_debug - {}", color);
	
	let mut size: (usize, usize) = (300, 300);
	let mut window = Window::new(
		title_override.unwrap_or(&name),
		size.0, size.1,
		window_options
	).expect("Could not open window");

	let mut buffer: Vec<u32> = Vec::with_capacity(size.0 * size.1);
	let mut is_focused = true;
	let mut redraw = true;

	while window.is_open() {
		window.update();

		let mut kill_keys_pressed = 0b00;
		window.get_keys().map(|keys| {
			for key in keys {
				match key {
					minifb::Key::Q => {
						kill_keys_pressed |= 0b01;
					}
					minifb::Key::LeftShift | minifb::Key::RightShift => {
						kill_keys_pressed |= 0b10;
					}
					_ => ()
				}
			}
		});
		if kill_keys_pressed == 0b11 {
			break;
		}

		let current_size = window.get_size();
		if current_size != size {
			size = current_size;
			redraw = true;
		}

		let is_focused_now = window.is_active();
		if is_focused_now != is_focused {
			is_focused = is_focused_now;
			redraw = true;
		}

		if redraw {
			buffer.clear();

			// resize buffer
			let required_pixels = size.0 * size.1;
			buffer.resize(required_pixels, color.background);

			if buffer.capacity() > required_pixels * 2 {
				buffer.shrink_to_fit();
			}

			update_buffer(&mut buffer, color, size, is_focused);
			window.update_with_buffer(&buffer, size.0, size.1).expect("Could not update the window with buffer");

			redraw = false;
		}
	}
}

fn update_buffer(buffer: &mut [u32], color: Color, size: (usize, usize), is_focused: bool) {
	let is_focused_text = if is_focused {
		TextLine::new(
			"F",
			[font::GLYPH_WIDTH as isize / 2, font::GLYPH_HEIGHT as isize / 4 - font::GLYPH_ASCENT as isize],
			2isize.try_into().unwrap()
		)
	} else {
		TextLine::empty()
	};
	
	#[cfg(feature = "centered_text")]
	let size_lines = {
		let centered_text = format!("{}x{}", size.0, size.1);
		let glyph_count = centered_text.len();

		let text_scale = font::compute_glyph_scale(size, glyph_count.try_into().unwrap(), 1usize.try_into().unwrap());

		let text_width = (font::GLYPH_WIDTH * glyph_count) as isize * text_scale.get();
		let text_height = (font::GLYPH_BASELINE - font::GLYPH_ASCENT) as isize * text_scale.get();

		let base_position = [
			(size.0 as isize - text_width) / 2,
			(size.1 as isize - text_height) / 2
		];

		let line = TextLine::new(
			&centered_text,
			base_position,
			text_scale
		);

		[line]
	};
	
	#[cfg(not(feature = "centered_text"))]
	let size_lines = {
		let width_text = format!("{}", size.0);
		let width_glyph_count = width_text.len();

		let height_text = format!("{}", size.1);
		let height_glyph_count = height_text.len();

		let text_scale = font::compute_glyph_scale(
			size,
			(
				std::cmp::max(
					width_glyph_count,
					height_glyph_count
				) * 2
			).try_into().unwrap(),
			2usize.try_into().unwrap()
		);

		let (base_width_position, base_height_position) = {
			let width_text_width = (font::GLYPH_WIDTH * width_glyph_count) as isize * text_scale.get();

			let margin = [
				(font::GLYPH_WIDTH / 2) as isize * text_scale.get(),
				(font::GLYPH_BASELINE - font::GLYPH_ASCENT) as isize / 2 * text_scale.get()
			];

			(
				[
					size.0 as isize - margin[0] - width_text_width,
					margin[1] - (font::GLYPH_ASCENT as isize) * text_scale.get()
				],
				[
					margin[0],
					size.1 as isize - margin[1] - font::GLYPH_BASELINE as isize * text_scale.get()
				]
			)
		};

		let width_line = TextLine::new(
			&width_text,
			base_width_position,
			text_scale
		);
		let height_line = TextLine::new(
			&height_text,
			base_height_position,
			text_scale
		);

		[width_line, height_line]
	};
	
	// recalculate the buffer
	for y in 0 .. size.1 {
		for x in 0 .. size.0 {
			let covers = is_focused_text.covers(x as isize, y as isize) || size_lines.iter().any(|l| l.covers(x as isize, y as isize));

			if covers {
				buffer[y * size.0 + x] = color.foreground;
			}
		}
	}
}