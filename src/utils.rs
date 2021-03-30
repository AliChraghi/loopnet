// Log
pub fn log(m: &str) {
	eprintln!("{}", rgb(80, 255, 80, format!("[ HOOP ] {}", m).as_str()));
}

// Log Errors
pub fn log_error(m: &str) {
	eprintln!("{}", rgb(255, 30, 30, format!("[ HOOP ] {}", m).as_str()));
}

// Log Warnings
pub fn log_warning(m: &str) {
	eprintln!("{}", rgb(255, 255, 80, format!("[ HOOP ] {}", m).as_str()));
}

// format_rgb() is a Cross-Platform RGB Formater, it was used to write colored text in shell.
// for more info read https://en.wikipedia.org/wiki/ANSI_escape_code
pub fn format_rgb(open: u8, close: u8, r: u8, g: u8, b: u8, msg: &str) -> String {
	return format!("\x1b[{};2;{};{};{}m{}\x1b[{}m", open, r, g, b, msg, close);
}

pub fn rgb(r: u8, g: u8, b: u8, msg: &str) -> String {
	return format_rgb(38, 39, r, g, b, msg);
}

pub fn bg_rgb(r: u8, g: u8, b: u8, msg: &str) -> String {
	return format_rgb(48, 49, r, g, b, msg);
}
