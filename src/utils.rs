use std::fs;
use std::path::{Path, PathBuf};

use std::os::unix::fs::PermissionsExt;
use regex::Regex;

pub fn format_file_size(bytes: Option<u64>) -> String {
    match bytes {
        None | Some(0) => "Unknown size".into(),
        Some(size) => {
            let units = ["B", "KB", "MB", "GB", "TB"];
            let mut unit_index = 0usize;
            let mut s = size as f64;
            while s >= 1024.0 && unit_index < units.len() - 1 {
                s /= 1024.0;
                unit_index += 1;
            }
            format!("{:.1}{}", s, units[unit_index])
        }
    }
}

pub fn format_duration(seconds: Option<u64>) -> String {
    match seconds {
        None => "Unknown duration".into(),
        Some(sec) => {
            let hours = sec / 3600;
            let mins = (sec % 3600) / 60;
            let secs = sec % 60;
            if hours > 0 {
                format!("{}:{:02}:{:02}", hours, mins, secs)
            } else {
                format!("{}:{:02}", mins, secs)
            }
        }
    }
}

pub fn format_time(seconds: f64) -> String {
    if seconds < 60.0 {
        format!("{}s", seconds.round() as u64)
    } else if seconds < 3600.0 {
        let mins = (seconds / 60.0).floor() as u64;
        let secs = (seconds % 60.0).round() as u64;
        format!("{}m {}s", mins, secs)
    } else {
        let hours = (seconds / 3600.0).floor() as u64;
        let mins = ((seconds % 3600.0) / 60.0).floor() as u64;
        let secs = (seconds % 60.0).round() as u64;
        format!("{}h {}m {}s", hours, mins, secs)
    }
}

pub fn format_number(n: Option<u64>) -> String {
    match n {
        None => "Unknown".into(),
        Some(val) => {
            let s = format!("{}", val);
            // simple thousands separator
            let bytes_rev: String = s.chars().rev().collect();
            let chunks: Vec<String> = bytes_rev
                .as_bytes()
                .chunks(3)
                .map(|c| String::from_utf8_lossy(c).to_string())
                .collect();
            let joined = chunks.join(",");
            joined.chars().rev().collect()
        }
    }
}

pub fn format_date(date_string: &str) -> String {
    if date_string.is_empty() {
        return "Unknown date".into();
    }

    // yt-dlp date format is usually YYYYMMDD
    if date_string.len() == 8 {
        let year = &date_string[0..4];
        let month = &date_string[4..6];
        let day = &date_string[6..8];
        return format!("{}-{}-{}", year, month, day);
    }

    date_string.to_string()
}

pub fn create_safe_filename(title: &str, quality: &str, extension: &str, max_len: usize) -> String {
    let config = crate::config::Config::default();
    
    // Clean title
    let _invalid_chars = &config.file_naming.invalid_chars;
    let replacement = config.file_naming.space_replacement;
    
    let mut s: String = title
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect();
    s = s.trim().replace(' ', replacement);
    
    if s.len() > max_len {
        s.truncate(max_len);
    }
    
    // Use template from config
    let template = config.file_naming.template;
    template
        .replace("{title}", &s)
        .replace("{quality}", quality)
        .replace("{ext}", extension)
}

pub fn ensure_directory_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_unique_filename(base: &Path) -> std::io::Result<std::path::PathBuf> {
    if !base.exists() {
        return Ok(base.to_path_buf());
    }
    let dir = base.parent().unwrap_or_else(|| Path::new("."));
    let ext = base.extension().and_then(|e| e.to_str()).unwrap_or("");
    let stem = base.file_stem().and_then(|s| s.to_str()).unwrap_or("file");

    for i in 1.. {
        let candidate = if ext.is_empty() {
            dir.join(format!("{}_{}", stem, i))
        } else {
            dir.join(format!("{}_{}.{}", stem, i, ext))
        };
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    // unreachable
    Ok(base.to_path_buf())
}

pub fn get_file_stats(file_path: &Path) -> Option<fs::Metadata> {
    match fs::metadata(file_path) {
        Ok(metadata) => Some(metadata),
        Err(_) => None,
    }
}

pub fn validate_youtube_url(url: &str) -> bool {
    let patterns = [
        r"^https?://(www\.)?youtube\.com/watch\?v=[\w-]+",
        r"^https?://(www\.)?youtu\.be/[\w-]+",
        r"^https?://(www\.)?youtube\.com/embed/[\w-]+",
        r"^https?://(www\.)?youtube\.com/v/[\w-]+",
    ];

    for p in patterns.iter() {
        if let Ok(re) = Regex::new(p) {
            if re.is_match(url) {
                return true;
            }
        }
    }
    false
}

pub fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/embed/|youtube\.com/v/)([^&\n?#]+)").unwrap();
    if let Some(caps) = re.captures(url) {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str().to_string());
        }
    }
    None
}

pub fn create_progress_bar(percentage: f64, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f64).round() as usize;
    let empty = width - filled;
    
    let bar = "█".repeat(filled) + &"░".repeat(empty);
    format!("[{}] {:.1}%", bar, percentage)
}

pub fn parse_progress(line: &str) -> Option<(f64, String, String, String)> {
    // Match yt-dlp progress format: [download]  45.2% of 123.45MiB at 1.23MiB/s ETA 00:30
    let re = Regex::new(r"\[download\]\s+(\d+\.?\d*)%\s+of\s+([\d.]+\w+)\s+at\s+([\d.]+\w+\/s)(?:\s+ETA\s+(\d+:\d+))?").unwrap();
    
    if let Some(caps) = re.captures(line) {
        let percentage = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let total_size = caps.get(2)?.as_str().to_string();
        let speed = caps.get(3)?.as_str().to_string();
        let eta = caps.get(4).map_or("Unknown".to_string(), |m| m.as_str().to_string());
        Some((percentage, total_size, speed, eta))
    } else {
        None
    }
}

pub fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input.replace(|c: char| matches!(c, ';' | '&' | '|' | '`' | '$' | '(' | ')' | '{' | '}' | '[' | ']'), "")
}

pub fn get_output_path(filename: &str) -> PathBuf {
    let config = crate::config::Config::default();
    let output_dir = &config.output_dir;
    ensure_directory_exists(output_dir).ok();
    output_dir.join(filename)
}

pub fn has_permission_issues(file_path: &Path) -> bool {
    // Basic implementation - would need platform-specific code for full implementation
    if !file_path.exists() {
        return false;
    }
    
    // Try to check write access
    match fs::metadata(file_path) {
        Ok(_metadata) => {
            // Check if we can write to the file
            match fs::OpenOptions::new().write(true).open(file_path) {
                Ok(_) => false, // Can write, no permission issues
                Err(_) => true, // Cannot write, permission issues
            }
        }
        Err(_) => true, // Cannot access metadata
    }
}

pub fn fix_file_permissions(file_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {


    // Basic implementation - would need more sophisticated permission handling
    match fs::metadata(file_path) {
        Ok(_metadata) => {
            // Set readable/writable permissions for user and group (0o664)
            let perms = fs::Permissions::from_mode(0o664);
            fs::set_permissions(file_path, perms)?;
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

pub fn find_files_with_permission_issues(dir_path: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut problematic_files = Vec::new();
    
    if !dir_path.exists() {
        return problematic_files;
    }
    
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext) && has_permission_issues(&path) {
                        problematic_files.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursively check subdirectories
                problematic_files.extend(find_files_with_permission_issues(&path, extensions));
            }
        }
    }
    
    problematic_files
}
