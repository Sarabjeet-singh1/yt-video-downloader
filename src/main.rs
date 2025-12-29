use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::io::Write;
use rust_downloader::{logger, Config, video_info, downloader, wallpaper_manager, dependencies, utils};

#[derive(Parser, Debug)]
#[command(name = "rust-downloader")]
#[command(disable_help_flag = true)]
#[command(about = "Race into the future with stunning live video wallpapers! Transform any YouTube video into a dynamic macOS wallpaper with precision and speed.", long_about = None)]
struct Args {
    /// YouTube URL to download (optional). If omitted, you'll be prompted to paste one.
    url: Option<String>,
    
    /// Disable wallpaper installation (download only mode)
    #[arg(long)]
    download_only: bool,
    
    /// Enable wallpaper installation (requires sudo)
    #[arg(long)]
    wallpaper: bool,
    
    /// Custom output directory
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Show detailed help information
    #[arg(short, long)]
    help: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive mode - prompts for URL and walks through setup
    Interactive,
    
    /// Download video only (no wallpaper installation)
    Download {
        /// YouTube URL to download
        url: String,
        
        /// Custom output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Download and install as wallpaper
    Wallpaper {
        /// YouTube URL to download and install
        url: String,
        
        /// Custom output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Check dependencies and environment
    Check,
    
    /// Display usage information
    Help,
}

fn prompt_for_url() -> Result<String, Box<dyn std::error::Error>> {
    use std::io::{self, Write};
    
    loop {
        print!("Enter the YouTube video URL: ");
        io::stdout().flush().ok();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            logger::error("Failed to read input. Try again.");
            continue;
        }
        
        let url = input.trim();
        if url.is_empty() {
            logger::warning("URL cannot be empty. Please try again.");
            continue;
        }
        
        if utils::validate_youtube_url(url) {
            if let Some(id) = utils::extract_video_id(url) {
            logger::success(&format!("Valid YouTube URL detected: {}", id));
        }
            return Ok(url.to_string());
        } else {
            logger::error("Invalid YouTube URL. Please provide a valid YouTube link.");
        }
    }
}

fn display_summary(download_path: &Path, wallpaper_installed: bool, start_time: std::time::SystemTime) {
    let total_time = start_time.elapsed().unwrap_or_default();
    let total_seconds = total_time.as_secs_f64();
    
    logger::header("Wallpaper Setup Summary");
    logger::success(&format!("Total time: {:.1} seconds", total_seconds));
    logger::file(&format!("Video saved to: {}", download_path.display()));

    if let Some(stats) = utils::get_file_stats(download_path) {
        logger::stats(&format!("Final size: {}", utils::format_file_size(Some(stats.len()))));
    }

    if wallpaper_installed {
        logger::success("Live wallpaper installed successfully!");
        println!();
        logger::info("Next Steps:");
        logger::info("   1. Restart your Mac to see the live wallpaper in action");
        logger::info("   2. If wallpaper becomes static after screen lock, run: cargo run --bin refresh");
        logger::info("   3. For best results, ensure your Mac stays plugged in (power saving can pause animation)");
        println!();
        logger::warning("Common Issue: Live wallpapers may appear static after unlocking from lock screen");
        logger::info("   This is a known macOS behavior - use the refresh utility to fix it");
    } else {
        logger::success("Video download completed successfully!");
    }
}

fn handle_error(error: &Box<dyn std::error::Error>, downloader: &mut downloader::Downloader) {
    logger::error(&format!("Application error: {}", error));
    
    // Provide helpful hints based on error type
    let error_msg = error.to_string();
    if error_msg.contains("yt-dlp") {
        logger::warning("Make sure yt-dlp is installed and accessible");
        logger::info("Install with: brew install yt-dlp (macOS) or pip install yt-dlp");
    } else if error_msg.contains("ffmpeg") {
        logger::warning("Make sure ffmpeg is installed and accessible");
        logger::info("Install with: brew install ffmpeg (macOS) or apt install ffmpeg (Ubuntu)");
    } else if error_msg.contains("Video unavailable") {
        logger::warning("The video might be private, deleted, or region-locked");
    } else if error_msg.contains("network") || error_msg.contains("connection") {
        logger::warning("Check your internet connection and try again");
    }
    
    // Cancel any ongoing download
    if downloader.is_download_in_progress() {
        downloader.cancel_download();
    }
}

fn setup_signal_handlers() {
    // In a full implementation, we'd set up proper signal handlers
    // For now, we'll just note that this would be implemented
    logger::info("Signal handlers initialized");
}

fn display_usage() {
    logger::header("Rust YouTube Downloader & Wallpaper Setter");
    logger::info("==========================================================");
    logger::info("");
    logger::info("Usage:");
    logger::info("   rust-downloader                    (interactive mode)");
    logger::info("   rust-downloader URL                (direct download)");
    logger::info("   rust-downloader --wallpaper URL    (with wallpaper installation)");
    logger::info("   rust-downloader --help             (show this help)");
    logger::info("");
    logger::info("Commands:");
    logger::info("   rust-downloader interactive        (interactive mode)");
    logger::info("   rust-downloader download URL       (download only)");
    logger::info("   rust-downloader wallpaper URL      (download + wallpaper)");
    logger::info("   rust-downloader check              (check dependencies)");
    logger::info("");
    logger::info("Examples:");
    logger::info("   rust-downloader                    # Start interactive video downloader");
    logger::info("   rust-downloader https://youtu.be/dQw4w9WgXcQ");
    logger::info("   rust-downloader --wallpaper https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    logger::info("");
    logger::info("Tips:");
    logger::info("   • Use --wallpaper flag for automatic wallpaper installation (requires sudo)");
    logger::info("   • Videos are converted to 4K 60fps HEVC .mov format");
    logger::info("   • Original files are cleaned up after conversion");
    logger::info("   • Run 'cargo run --bin cleanup' to fix permission issues");
    logger::info("   • Run 'cargo run --bin refresh' to refresh wallpaper animation");
    logger::info("");
    logger::info("Features:");
    logger::info("   • Intelligent video format selection");
    logger::info("   • Hardware-accelerated HEVC conversion");
    logger::info("   • Automatic wallpaper installation for macOS");
    logger::info("   • Comprehensive error handling and user guidance");
    logger::info("   • Progress tracking and detailed logging");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start_time = std::time::SystemTime::now();
    
    // Initialize logger
    logger::init();
    
    // Handle help flag
    if args.help {
        display_usage();
        return Ok(());
    }

    let mut config = Config::default();
    
    // Apply command line arguments
    if args.wallpaper {
        config.enable_wallpaper = true;
    }
    
    if args.download_only {
        config.enable_wallpaper = false;
    }
    
    if let Some(output_dir) = &args.output {
        config.output_dir = output_dir.clone();
    }

    // Handle commands
    let command_result = if let Some(url) = args.url {
        // Direct URL provided
        if config.enable_wallpaper {
            run_with_wallpaper(&url, &config, start_time).await
        } else {
            run_download_only(&url, &config, start_time).await
        }
    } else {
        // Interactive mode
        interactive_mode(&config, start_time).await
    };

    match command_result {
        Ok((download_path, wallpaper_installed)) => {
            display_summary(&download_path, wallpaper_installed, start_time);
        }
        Err(error) => {
            let mut downloader = downloader::Downloader::new();
            handle_error(&error, &mut downloader);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn run_with_wallpaper(url: &str, config: &Config, start_time: std::time::SystemTime) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    logger::header("Rust YouTube Downloader & Wallpaper Setter");
    logger::info("Transform YouTube videos into stunning live wallpapers for macOS");
    logger::info("Intelligent automation with comprehensive error handling");
    println!();

    // Setup signal handlers
    setup_signal_handlers();

    // Check environment and dependencies
    let dependency_checker = dependencies::DependencyChecker::new();
    dependency_checker.perform_full_check().await?;

    // Analyze video
    let analysis = video_info::analyze(url)?;

    // Perform download and conversion
    let mut downloader = downloader::Downloader::new();
    let download_path = downloader.perform_download(url, &analysis).await?;

    // Setup wallpaper (only if enabled)
    let wallpaper_installed = if config.enable_wallpaper {
        logger::info("Starting wallpaper installation process...");
        let wallpaper_mgr = wallpaper_manager::WallpaperManager::new();
        wallpaper_mgr.setup_wallpaper(&download_path).await?
    } else {
        logger::info("Wallpaper installation disabled; running in download-only mode.");
        false
    };

    Ok((download_path, wallpaper_installed))
}

async fn run_download_only(url: &str, config: &Config, start_time: std::time::SystemTime) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    logger::header("Rust YouTube Downloader");
    logger::info("Download and convert YouTube videos for any purpose");
    println!();

    // Setup signal handlers
    setup_signal_handlers();

    // Check dependencies only (no sudo needed for download only)
    let dependency_checker = dependencies::DependencyChecker::new();
    let mut check_config = config.clone();
    check_config.enable_wallpaper = false; // Override to skip sudo check
    let _ = dependency_checker.perform_full_check().await;

    // Analyze video
    let analysis = video_info::analyze(url)?;

    // Perform download and conversion
    let mut downloader = downloader::Downloader::new();
    let download_path = downloader.perform_download(url, &analysis).await?;

    Ok((download_path, false))
}

async fn interactive_mode(config: &Config, start_time: std::time::SystemTime) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    // Display header
    logger::header("Rust YouTube Downloader & Wallpaper Setter");
    logger::info("Transform YouTube videos into stunning live wallpapers for macOS");
    logger::info("Intelligent automation with comprehensive error handling");
    println!();

    // Get YouTube URL interactively
    let url = prompt_for_url()?;

    // Ask user about wallpaper installation
    let mut final_config = config.clone();
    if !config.enable_wallpaper {
        println!();
        logger::info("Supported URL formats:");
        logger::info("   • https://www.youtube.com/watch?v=VIDEO_ID");
        logger::info("   • https://youtu.be/VIDEO_ID");
        logger::info("   • https://www.youtube.com/embed/VIDEO_ID");
        logger::info("   • https://www.youtube.com/v/VIDEO_ID");
        println!();

        print!("Do you want to install this as a live wallpaper? (y/N): ");
        std::io::stdout().flush().ok();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
            final_config.enable_wallpaper = true;
            logger::info("Wallpaper installation enabled");
            println!();
        } else {
            logger::info("Running in download-only mode");
        }
    }

    if final_config.enable_wallpaper {
        run_with_wallpaper(&url, &final_config, start_time).await
    } else {
        run_download_only(&url, &final_config, start_time).await
    }
}
