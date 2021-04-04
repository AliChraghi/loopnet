pub mod out {
    use crate::tips;
    use std::io::Error;
    // Log
    pub fn log(m: &str) {
        println!("{}", color(92, format!("[ HOOP ] {}", m)));
    }

    pub fn help(m: Error) {
        println!(
            "{}",
            color(96, format!("[ HOOP:HLP ] {}", tips::help_text(m)))
        );
    }

    // Log Warnings
    pub fn warning(m: &str) {
        println!("{}", color_bold(93, format!("[ HOOP:WRN ] {}", m)));
    }

    // Log Errors
    pub fn error(m: Error) {
        eprintln!("{}", color_bold(91, format!("[ HOOP:ERR ] {}", m)));
        help(m);
    }

    pub fn color_bold(color: u8, message: String) -> String {
        return format!("\x1b[1m\x1b[{}m{}\x1b[0m", color, message);
    }

    pub fn color(color: u8, message: String) -> String {
        return format!("\x1b[{}m{}\x1b[0m", color, message);
    }
}
