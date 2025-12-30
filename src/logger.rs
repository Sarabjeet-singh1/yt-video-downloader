use std::time::SystemTime;
use std::io::Write;

static mut START_TIME: Option<SystemTime> = None;

// ANSI color codes
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_INFO: &str = "\x1b[36m";     // Cyan
const COLOR_SUCCESS: &str = "\x1b[32m";  // Green
const COLOR_WARNING: &str = "\x1b[33m";  // Yellow
const COLOR_ERROR: &str = "\x1b[31m";    // Red

// Text symbols
const SYMBOL_INFO: &str = "INFO:";
const SYMBOL_SUCCESS: &str = "OK:";
const SYMBOL_WARNING: &str = "WARNING:";
const SYMBOL_ERROR: &str = "ERROR:";
const SYMBOL_DOWNLOAD: &str = "DOWNLOAD:";
const SYMBOL_SEARCH: &str = "SEARCH:";
const SYMBOL_VIDEO: &str = "VIDEO:";
const SYMBOL_AUDIO: &str = "AUDIO:";
const SYMBOL_FILE: &str = "FILE:";
const SYMBOL_STATS: &str = "STATS:";
const SYMBOL_WALLPAPER: &str = "WALLPAPER:";
const SYMBOL_BACKUP: &str = "BACKUP:";
const SYMBOL_INSTALL: &str = "INSTALL:";
const SYMBOL_CONVERT: &str = "CONVERT:";

fn init_start_time() {
    unsafe {
        let ptr = &raw const START_TIME as *const Option<SystemTime>;
        if (*ptr).is_none() {
            START_TIME = Some(SystemTime::now());
        }
    }
}

fn elapsed_time() -> String {
    unsafe {
        if let Some(start) = START_TIME {
            if let Ok(elapsed) = start.elapsed() {
                let seconds = elapsed.as_secs_f64();
                return format!("[{:.1}s]", seconds);
            }
        }
        "[0.0s]".to_string()
    }
}

fn colorize(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, COLOR_RESET)
}

pub fn init() {
    init_start_time();
}

pub fn header(s: &str) {
    println!();
    separator();
    println!("  {}", s);
    separator();
    println!();
}

pub fn info(s: &str) {
    println!("{}", format_message("info", SYMBOL_INFO, s));
}

pub fn success(s: &str) {
    println!("{}", format_message("success", SYMBOL_SUCCESS, s));
}

pub fn warning(s: &str) {
    println!("{}", format_message("warning", SYMBOL_WARNING, s));
}

pub fn error(s: &str) {
    eprintln!("{}", format_message("error", SYMBOL_ERROR, s));
}

pub fn video(s: &str) {
    println!("{}", format_message("info", SYMBOL_VIDEO, s));
}

pub fn audio(s: &str) {
    println!("{}", format_message("info", SYMBOL_AUDIO, s));
}

pub fn file(s: &str) {
    println!("{}", format_message("info", SYMBOL_FILE, s));
}

pub fn stats(s: &str) {
    println!("{}", format_message("info", SYMBOL_STATS, s));
}

pub fn download(s: &str) {
    println!("{}", format_message("info", SYMBOL_DOWNLOAD, s));
}

pub fn search(s: &str) {
    println!("{}", format_message("info", SYMBOL_SEARCH, s));
}

pub fn wallpaper(s: &str) {
    println!("{}", format_message("info", SYMBOL_WALLPAPER, s));
}

pub fn backup(s: &str) {
    println!("{}", format_message("info", SYMBOL_BACKUP, s));
}

pub fn install(s: &str) {
    println!("{}", format_message("info", SYMBOL_INSTALL, s));
}

pub fn convert(s: &str) {
    println!("{}", format_message("info", SYMBOL_CONVERT, s));
}

pub fn progress(s: &str) {
    clear_line();
    print!("{} {}", elapsed_time(), s);
    std::io::stdout().flush().ok();
}

pub fn progress_complete(s: &str) {
    clear_line();
    success(s);
}

fn clear_line() {
    print!("\r\x1b[K");
}

fn separator() {
    println!("{}", colorize("─".repeat(60).as_str(), COLOR_INFO));
}

fn format_message(level: &str, symbol: &str, message: &str) -> String {
    let timestamp = elapsed_time();
    let color = match level {
        "info" => COLOR_INFO,
        "success" => COLOR_SUCCESS,
        "warning" => COLOR_WARNING,
        "error" => COLOR_ERROR,
        _ => COLOR_RESET,
    };
    
    let colored_symbol = colorize(symbol, color);
    let colored_message = colorize(message, color);
    
    format!("{} {} {}", timestamp, colored_symbol, colored_message)
}
