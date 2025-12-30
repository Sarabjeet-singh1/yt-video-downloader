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
#[allow(dead_code)]
const SYMBOL_SUCCESS: &str = "OK:";
const SYMBOL_DOWNLOAD: &str = "DOWNLOAD:";
const SYMBOL_STATS: &str = "STATS:";
#[allow(dead_code)]
const SYMBOL_INFO: &str = "INFO:";
#[allow(dead_code)]
const SYMBOL_WARNING: &str = "WARNING:";
#[allow(dead_code)]
const SYMBOL_ERROR: &str = "ERROR:";
#[allow(dead_code)]
const SYMBOL_SEARCH: &str = "SEARCH:";
#[allow(dead_code)]
const SYMBOL_VIDEO: &str = "VIDEO:";
#[allow(dead_code)]
const SYMBOL_AUDIO: &str = "AUDIO:";
#[allow(dead_code)]
const SYMBOL_FILE: &str = "FILE:";
#[allow(dead_code)]
const SYMBOL_WALLPAPER: &str = "WALLPAPER:";
#[allow(dead_code)]
const SYMBOL_BACKUP: &str = "BACKUP:";
#[allow(dead_code)]
const SYMBOL_INSTALL: &str = "INSTALL:";
#[allow(dead_code)]
const SYMBOL_CONVERT: &str = "CONVERT:";

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn init() {
    init_start_time();
}

#[allow(dead_code)]
pub fn header(s: &str) {
    println!();
    separator();
    println!("  {}", s);
    separator();
    println!();
}

#[allow(dead_code)]
pub fn info(s: &str) {
    println!("{}", format_message("info", SYMBOL_INFO, s));
}

#[allow(dead_code)]
pub fn success(s: &str) {
    println!("{}", format_message("success", SYMBOL_SUCCESS, s));
}

#[allow(dead_code)]
pub fn warning(s: &str) {
    println!("{}", format_message("warning", SYMBOL_WARNING, s));
}

#[allow(dead_code)]
pub fn error(s: &str) {
    eprintln!("{}", format_message("error", SYMBOL_ERROR, s));
}

#[allow(dead_code)]
pub fn video(s: &str) {
    println!("{}", format_message("info", SYMBOL_VIDEO, s));
}

#[allow(dead_code)]
pub fn audio(s: &str) {
    println!("{}", format_message("info", SYMBOL_AUDIO, s));
}

#[allow(dead_code)]
pub fn file(s: &str) {
    println!("{}", format_message("info", SYMBOL_FILE, s));
}

#[allow(dead_code)]
pub fn stats(s: &str) {
    println!("{}", format_message("info", SYMBOL_STATS, s));
}

#[allow(dead_code)]
pub fn download(s: &str) {
    println!("{}", format_message("info", SYMBOL_DOWNLOAD, s));
}

#[allow(dead_code)]
pub fn search(s: &str) {
    println!("{}", format_message("info", SYMBOL_SEARCH, s));
}

#[allow(dead_code)]
pub fn wallpaper(s: &str) {
    println!("{}", format_message("info", SYMBOL_WALLPAPER, s));
}

#[allow(dead_code)]
pub fn backup(s: &str) {
    println!("{}", format_message("info", SYMBOL_BACKUP, s));
}

#[allow(dead_code)]
pub fn install(s: &str) {
    println!("{}", format_message("info", SYMBOL_INSTALL, s));
}

#[allow(dead_code)]
pub fn convert(s: &str) {
    println!("{}", format_message("info", SYMBOL_CONVERT, s));
}

#[allow(dead_code)]
pub fn progress(s: &str) {
    clear_line();
    print!("{} {}", elapsed_time(), s);
    std::io::stdout().flush().ok();
}

#[allow(dead_code)]
pub fn progress_complete(s: &str) {
    clear_line();
    success(s);
}

#[allow(dead_code)]
fn clear_line() {
    print!("\r\x1b[K");
}

#[allow(dead_code)]
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
