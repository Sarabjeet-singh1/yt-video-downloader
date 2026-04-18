use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use rust_downloader::{logger, Config, video_info, downloader, video_manager, dependencies, utils};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Mp4,
    Mov,
}

#[derive(Parser, Debug)]
#[command(name = "rust-downloader")]
#[command(about = "Race into the future with stunning live video! Transform online videos into dynamic local files with precision and speed.", long_about = None)]
struct Args {
    /// Video URL to download (YouTube, Instagram, X/Twitter). If omitted, you'll be prompted to paste one.
    url: Option<String>,
    
    /// Disable video installation (download only mode)
    #[arg(long)]
    download_only: bool,
    
    /// Enable video installation (requires sudo)
    #[arg(long)]
    video: bool,

    /// Output format (mp4 or mov)
    #[arg(long, value_enum)]
    format: Option<OutputFormat>,
    
    /// Custom output directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive mode - prompts for URL and walks through setup
    Interactive,
    
    /// Download video only (no video installation)
    Download {
        /// Video URL to download
        url: String,
        
        /// Custom output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Download and install as video
    Video {
        /// Video URL to download and install
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
        print!("Enter the video URL (YouTube/Instagram/X): ");
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
        
        if utils::validate_supported_url(url) {
            let platform = utils::detect_platform(url);
            if let Some(id) = utils::extract_video_id(url) {
                logger::success(&format!("Valid {} URL detected: {}", platform, id));
            } else {
                logger::success(&format!("Valid {} URL detected", platform));
            }
            return Ok(url.to_string());
        } else {
            logger::error("Invalid URL. Please provide a valid YouTube, Instagram, or X/Twitter link.");
        }
    }
}

fn prompt_for_output_format() -> Result<OutputFormat, Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    println!();
    logger::info("Choose output format:");
    logger::info("   1. mp4 (faster, no conversion)");
    logger::info("   2. mov (HEVC optimized for macOS live wallpaper)");

    loop {
        print!("Select format [1/2] (default: 2): ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "" | "2" | "mov" => return Ok(OutputFormat::Mov),
            "1" | "mp4" => return Ok(OutputFormat::Mp4),
            _ => logger::warning("Invalid choice. Enter 1/mp4 or 2/mov."),
        }
    }
}

fn apply_output_format(config: &mut Config, format: OutputFormat) {
    match format {
        OutputFormat::Mp4 => {
            config.download_settings.convert_to_mov = false;
            logger::info("Output format set to mp4");
        }
        OutputFormat::Mov => {
            config.download_settings.convert_to_mov = true;
            logger::info("Output format set to mov");
        }
    }
}

fn display_summary(download_path: &PathBuf, video_installed: bool, start_time: std::time::SystemTime) {
    let total_time = start_time.elapsed().unwrap_or_default();
    let total_seconds = total_time.as_secs_f64();
    
    logger::header("Video Setup Summary");
    logger::success(&format!("Total time: {:.1} seconds", total_seconds));
    logger::file(&format!("Video saved to: {}", download_path.display()));

    if let Some(stats) = utils::get_file_stats(download_path) {
        logger::stats(&format!("Final size: {}", utils::format_file_size(Some(stats.len()))));
    }

    if video_installed {
        logger::success("Live video installed successfully!");
        println!();
        logger::info("Next Steps:");
        logger::info("   1. Restart your Mac to see the live video in action");
        logger::info("   2. If it becomes static after screen lock, run: cargo run --bin refresh");
        logger::info("   3. For best results, ensure your Mac stays plugged in (power saving can pause animation)");
        println!();
        logger::warning("Common Issue: Live videos may appear static after unlocking from lock screen");
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

#[allow(dead_code)]
fn display_usage() {
    logger::header("Rust Video Downloader ");
    logger::info("==========================================================");
    logger::info("");
    logger::info("Usage:");
    logger::info("   rust-downloader                    (interactive mode)");
    logger::info("   rust-downloader URL                (direct download)");
    logger::info("   rust-downloader --video URL         (with video installation)");
    logger::info("   rust-downloader --format mp4 URL    (download mp4 output)");
    logger::info("   rust-downloader --format mov URL    (download mov output)");
    logger::info("   rust-downloader --help              (show this help)");
    logger::info("");
    logger::info("Commands:");
    logger::info("   rust-downloader interactive         (interactive mode)");
    logger::info("   rust-downloader download URL        (download only)");
    logger::info("   rust-downloader video URL           (download + video)");
    logger::info("   rust-downloader check               (check dependencies)");
    logger::info("");
    logger::info("Examples:");
    logger::info("   rust-downloader                     # Start interactive video downloader");
    logger::info("   rust-downloader https://youtu.be/dQw4w9WgXcQ");
    logger::info("   rust-downloader --video https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    logger::info("   rust-downloader https://www.instagram.com/reel/REEL_ID/");
    logger::info("   rust-downloader https://x.com/username/status/1234567890");
    logger::info("   rust-downloader --format mp4 https://youtu.be/dQw4w9WgXcQ");
    logger::info("");
    logger::info("Tips:");
    logger::info("   • Use --video flag for automatic video installation (requires sudo)");
    logger::info("   • Use --format mp4 or --format mov to choose output format");
    logger::info("   • .mov uses HEVC conversion for macOS wallpaper compatibility");
    logger::info("   • Original files are cleaned up after conversion");
    logger::info("   • Run 'cargo run --bin cleanup' to fix permission issues");
    logger::info("   • Run 'cargo run --bin refresh' to refresh video animation");
    logger::info("");
    logger::info("Features:");
    logger::info("   • Intelligent video format selection");
    logger::info("   • Hardware-accelerated HEVC conversion");
    logger::info("   • Automatic video installation for macOS");
    logger::info("   • Comprehensive error handling and user guidance");
    logger::info("   • Progress tracking and detailed logging");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start_time = std::time::SystemTime::now();
    
    // Initialize logger
    logger::init();

    let mut config = Config::default();
    
    // Apply command line arguments
    if args.video {
        config.enable_video = true;
    }

    if args.download_only {
        config.enable_video = false;
    }

    if let Some(format) = args.format {
        apply_output_format(&mut config, format);
    }
    
    if let Some(output_dir) = &args.output {
        config.output_dir = Config::expand_tilde(output_dir.to_str().unwrap_or(""));
    }

    // Ensure output directory exists
    config.ensure_output_dir_exists()?;

    // Handle commands
    let command_result = if let Some(url) = args.url {
        // Direct URL provided
        if config.enable_video{
            run_with_video(&url, &config, start_time).await
        } else {
            run_download_only(&url, &config, start_time).await
        }
    } else {
        // Interactive mode
        interactive_mode(&config, start_time, args.format).await
    };

    match command_result {
        Ok((download_path, video_installed)) => {
            display_summary(&download_path, video_installed, start_time);
        }
        Err(error) => {
            let mut downloader = downloader::Downloader::new();
            handle_error(&error, &mut downloader);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn run_with_video(url: &str, config: &Config, _start_time: std::time::SystemTime) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    logger::header("Rust Video Downloader ");
    logger::info("Transform online videos for any purpose");
    logger::info("Intelligent automation with comprehensive error handling");
    println!();

    // Setup signal handlers
    setup_signal_handlers();

    // Check environment and dependencies
    let dependency_checker = dependencies::DependencyChecker::new();
    dependency_checker.perform_full_check(config).await?;

    // Analyze video
    let analysis = video_info::analyze(url)?;

    // Perform download and conversion
    let mut downloader = downloader::Downloader::new();
    let download_path = downloader.perform_download(url, &analysis, config).await?;

    // Setup video (only if enabled)
    let video_installed = if config.enable_video {
        logger::info("Starting video installation process...");
        let video_mgr = video_manager::VideoManager::new();
        video_mgr.setup_video(&download_path).await?
    } else {
        logger::info("Video installation disabled; running in download-only mode.");
        false
    };

    Ok((download_path, video_installed))
}

async fn run_download_only(url: &str, config: &Config, _start_time: std::time::SystemTime) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    logger::header("Rust Video Downloader");
    logger::info("Download and convert online videos for any purpose");
    println!();

    // Setup signal handlers
    setup_signal_handlers();

    // Check dependencies only (no sudo needed for download only)
    let dependency_checker = dependencies::DependencyChecker::new();
    let mut check_config = config.clone();
    check_config.enable_video = false; // Override to skip sudo check
    let _ = dependency_checker.perform_full_check(&check_config).await;

    // Analyze video
    let analysis = video_info::analyze(url)?;

    // Perform download and conversion
    let mut downloader = downloader::Downloader::new();
    let download_path = downloader.perform_download(url, &analysis, config).await?;

    Ok((download_path, false))
}

async fn interactive_mode(config: &Config, start_time: std::time::SystemTime, cli_format: Option<OutputFormat>) -> Result<(PathBuf, bool), Box<dyn std::error::Error>> {
    // Display header
    logger::header("Rust Video Downloader ");
    logger::info("Transform online videos into your local machine");
    logger::info("Intelligent automation with comprehensive error handling");
    println!();

    // Get URL interactively
    let url = prompt_for_url()?;

    // Keep interactive mode download-focused by default.
    // Wallpaper installation is only used when explicitly enabled via --video.
    let mut final_config = config.clone();
    if !config.enable_video {
        final_config.enable_video = false;
        logger::info("Running in download-only mode");
        logger::info("Tip: pass --video only if you want wallpaper installation.");
    }

    if let Some(format) = cli_format {
        apply_output_format(&mut final_config, format);
    } else {
        let selected = prompt_for_output_format()?;
        apply_output_format(&mut final_config, selected);
    }

    if final_config.enable_video {
        run_with_video(&url, &final_config, start_time).await
    } else {
        run_download_only(&url, &final_config, start_time).await
    }
}
