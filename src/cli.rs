use std::time::SystemTime;

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
pub struct Color {
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

#[derive(Debug)]
pub struct CliArgs {
	pub color: Color,
	pub borderless: bool,
	pub title: bool,
	pub resize: bool,
	pub topmost: bool,
	pub title_override: Option<String>
}
pub fn parse_cli() -> CliArgs {
	let mut args = std::env::args();
	args.next(); // skip exec name

	let mut color = None::<Color>;
	let mut borderless = false;
	let mut title = true;
	let mut resize = false;
	let mut topmost = false;
	let mut title_override = None::<String>;

	let mut current_arg = args.next();
	while let Some(current) = current_arg.take() {
		if current.starts_with("-") {
			match current.as_str() {
				"-b" | "--borderless" => {
					borderless = true;
				}
				"--no-title" => {
					title = false;
				}
				"-r" | "--resize" => {
					resize = true;
				}
				"--top" => {
					topmost = true;
				}
				"-t" | "--title" => {
					title_override = args.next();
				}

				"-h" | "--help" => {
					eprintln!("window_debug [index | #HHEEXX] [-b | --borderless] [--no-title] [-r | --resize] [--top] [-t TITLE | --title TITLE] [-h | --help]");
					std::process::exit(0);
				}
				opt => {
					eprintln!("Unrecognized CLI option: {}", opt)
				}
			}
		} else {
			color = if current.starts_with("#") {
				u32::from_str_radix(&current[1..], 16).ok().map(|bg| Color::new_hex(bg, 0x000000, None))
			} else {
				usize::from_str_radix(&current, 10).ok().map(|index| COLOR_PRESETS[index % COLOR_PRESETS.len()])
			}
		}

		current_arg = args.next();
	}

	CliArgs {
		color: color.unwrap_or_else(
			|| {
				let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
	
				let md = COLOR_PRESETS.len() as u64;
				let index = (time.as_secs() % md + time.as_micros() as u64 % md) % md;
				COLOR_PRESETS[index as usize]
			}
		),
		borderless,
		title,
		resize,
		topmost,
		title_override
	}
}