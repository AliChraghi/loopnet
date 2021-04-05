pub mod out {
    use crate::tips;
    use std::io::Error;
    // Log
    pub fn log(m: &str) {
        println!(
            "{}",
            format!("[ {} ] {}", color_bold(92, "HOOP"), m).as_str()
        );
    }

    pub fn help(m: Error) {
        println!(
            "{}",
            format!("[ {} ] {}", color_bold(96, "HOOP"), tips::help_text(m)).as_str()
        );
    }

    // Log Warnings
    pub fn warning(m: &str) {
        println!(
            "{}",
            format!("[ {} ] {}", color_bold(93, "HOOP"), m).as_str()
        );
    }

    // Log Errors
    pub fn error(m: Error) {
        eprintln!(
            "{}",
            format!("[ {} ] {}", color_bold(91, "HOOP"), m).as_str()
        );
        help(m);
    }

    pub fn color_bold(color: u8, message: &str) -> String {
        return format!("\x1b[1m\x1b[{}m{}\x1b[0m", color, message);
    }

    pub fn color(color: u8, message: &str) -> String {
        return format!("\x1b[{}m{}\x1b[0m", color, message);
    }
}
