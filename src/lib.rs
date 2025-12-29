// Re-export all modules for easier importing
pub mod config;
pub mod logger;
pub mod utils;
pub mod video_info;
pub mod downloader;
pub mod wallpaper_manager;
pub mod dependencies;

// Re-export commonly used types
pub use config::Config;
pub use logger::*;
pub use utils::*;
pub use video_info::{analyze, VideoInfo, SelectedFormats, VideoFormat, AudioFormat};
pub use downloader::Downloader;
pub use wallpaper_manager::WallpaperManager;
pub use dependencies::DependencyChecker;
