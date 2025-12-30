use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct VideoPreferences {
    pub preferred_formats: Vec<&'static str>,
    pub preferred_codecs: Vec<&'static str>,
    pub max_resolution: u32,
    pub prefer_high_fps: bool,
    pub prefer_60fps: bool,
}

#[derive(Debug, Clone)]
pub struct AudioPreferences {
    pub preferred_formats: Vec<&'static str>,
    pub preferred_codecs: Vec<&'static str>,
    pub min_bitrate: u32,
    pub preferred_bitrate: u32,
}

#[derive(Debug, Clone)]
pub struct DownloadSettings {
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
    pub merge_output_format: &'static str,
    pub embed_subtitles: bool,
    pub embed_thumbnail: bool,
    pub convert_to_mov: bool,
    pub optimize_for_video: bool,
    pub use_hevc: bool,
    pub target_frame_rate: u32,
    pub target_resolution: &'static str,
}

#[derive(Debug, Clone)]
pub struct ConversionSettings {
    pub max_attempts: u32,
    pub fallback_resolutions: Vec<&'static str>,
    pub fallback_bitrates: Vec<&'static str>,
    pub fallback_frame_rates: Vec<u32>,
    pub conservative_mode: bool,
}

#[derive(Debug, Clone)]
pub struct VideoSettings {
    pub customer_dir: &'static str,
    pub target_sub_dir: &'static str,
    pub backup_dir: &'static str,
    pub required_format: &'static str,
    pub min_recommended_resolution: u32,
    pub min_recommended_duration: u32,
    pub max_retry_attempts: u32,
    pub retry_interval: u64,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub colors: ColorConfig,
    pub symbols: SymbolConfig,
}

#[derive(Debug, Clone)]
pub struct ColorConfig {
    pub info: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub reset: &'static str,
}

#[derive(Debug, Clone)]
pub struct SymbolConfig {
    pub info: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub download: &'static str,
    pub search: &'static str,
    pub video: &'static str,
    pub audio: &'static str,
    pub file: &'static str,
    pub stats: &'static str,
    pub wallpaper: &'static str,
    pub backup: &'static str,
    pub install: &'static str,
    pub convert: &'static str,
}

#[derive(Debug, Clone)]
pub struct DependencyConfig {
    pub command: &'static str,
    pub args: Vec<&'static str>,
    pub install_hint: &'static str,
}

#[derive(Debug, Clone)]
pub struct FileNamingConfig {
    pub max_title_length: usize,
    pub invalid_chars: &'static str,
    pub space_replacement: &'static str,
    pub template: &'static str,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub enable_video: bool,
    pub output_dir: PathBuf,
    
    pub video_preferences: VideoPreferences,
    pub audio_preferences: AudioPreferences,
    pub download_settings: DownloadSettings,
    pub conversion_settings: ConversionSettings,
    pub video_settings: VideoSettings,
    pub logging: LoggingConfig,
    pub dependencies: Vec<DependencyConfig>,
    pub file_naming: FileNamingConfig,
}

impl Config {
    pub fn default() -> Self {
        Self {
            enable_video: false,
            output_dir: PathBuf::from("outputs"),
            
            video_preferences: VideoPreferences {
                preferred_formats: vec!["mp4", "mkv", "webm"],
                preferred_codecs: vec!["h264", "vp9", "av01"],
                max_resolution: 2160, // 4K
                prefer_high_fps: true,
                prefer_60fps: true,
            },
            
            audio_preferences: AudioPreferences {
                preferred_formats: vec!["m4a", "mp3", "webm"],
                preferred_codecs: vec!["aac", "mp3", "opus"],
                min_bitrate: 128,
                preferred_bitrate: 320,
            },
            
            download_settings: DownloadSettings {
                retry_attempts: 3,
                timeout_seconds: 300,
                merge_output_format: "mp4",
                embed_subtitles: false,
                embed_thumbnail: false,
                convert_to_mov: true,
                optimize_for_video: true,
                use_hevc: true,
                target_frame_rate: 60,
                target_resolution: "3840x2160",
            },

            conversion_settings: ConversionSettings {
                max_attempts: 5,
                fallback_resolutions: vec!["3840x2160", "2560x1440", "1920x1080", "1280x720"],
                fallback_bitrates: vec!["50M", "30M", "20M", "10M"],
                fallback_frame_rates: vec![60, 30, 24],
                conservative_mode: false,
            },

            video_settings: VideoSettings {
                customer_dir: "/Library/Application Support/com.apple.idleassetsd/Customer",
                target_sub_dir: "4KSDR240FPS",
                backup_dir: "video_backups",
                required_format: ".mov",
                min_recommended_resolution: 2160, // 4K
                min_recommended_duration: 60, // 1 minute in seconds
                max_retry_attempts: 30,
                retry_interval: 1000,
            },
            
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
                colors: ColorConfig {
                    info: "\x1b[36m",    // Cyan
                    success: "\x1b[32m", // Green
                    warning: "\x1b[33m", // Yellow
                    error: "\x1b[31m",   // Red
                    reset: "\x1b[0m"     // Reset
                },
                symbols: SymbolConfig {
                    info: "ℹ️",
                    success: "✅",
                    warning: "⚠️",
                    error: "❌",
                    download: "⬇️",
                    search: "🔍",
                    video: "📺",
                    audio: "🎵",
                    file: "📁",
                    stats: "📊",
                    wallpaper: "🖼️",
                    backup: "💾",
                    install: "🔧",
                    convert: "🔄"
                }
            },
            
            dependencies: vec![
                DependencyConfig {
                    command: "yt-dlp",
                    args: vec!["--version"],
                    install_hint: "Install with: brew install yt-dlp (macOS) or pip install yt-dlp",
                },
                DependencyConfig {
                    command: "ffmpeg",
                    args: vec!["-version"],
                    install_hint: "Install with: brew install ffmpeg (macOS) or apt install ffmpeg (Ubuntu)",
                }
            ],
            
            file_naming: FileNamingConfig {
                max_title_length: 50,
                invalid_chars: "[^\\w\\s-]",
                space_replacement: "_",
                template: "{title}_{quality}.{ext}"
            }
        }
    }
}
