use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::io::Write;
use crate::logger;
use crate::Config;
use crate::utils;

pub struct VideoManager {
    customer_dir: PathBuf,
    target_dir: PathBuf,
    backup_dir: PathBuf,
    retry_attempts: u32,
    retry_interval: Duration,
}

impl VideoManager {
    pub fn new() -> Self {
        let config = Config::default();
        let customer_dir = PathBuf::from(config.video_settings.customer_dir);
        let target_dir = customer_dir.join(config.video_settings.target_sub_dir);
        let backup_dir = config.output_dir.join(config.video_settings.backup_dir);
        
        Self {
            customer_dir,
            target_dir,
            backup_dir,
            retry_attempts: config.video_settings.max_retry_attempts,
            retry_interval: Duration::from_millis(config.video_settings.retry_interval),
        }
    }

    async fn check_customer_directory(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.customer_dir.exists() {
            logger::error(" Customer directory not found");
            logger::info("This usually means macOS video system is not initialized");
            return Ok(false);
        }

        if !self.target_dir.exists() {
            logger::warning("  4KSDR240FPS directory not found, creating...");
            fs::create_dir_all(&self.target_dir)?;
        }

        // Test write permissions
        let test_file = self.target_dir.join(".test_write");
        match fs::write(&test_file, "test") {
            Ok(_) => {
                let _ = fs::remove_file(&test_file);
                logger::success(" Customer directory is accessible");
                Ok(true)
            }
            Err(_) => {
                logger::error(" No write permissions to Customer directory");
                logger::warning(" This application requires administrator privileges");
                logger::info(" Please restart with: sudo rust-downloader \"YOUR_VIDEO_URL\"");
                Ok(false)
            }
        }
    }

    fn get_existing_videos(&self) -> Vec<VideoFile> {
        let mut videos = Vec::new();
        
        if !self.target_dir.exists() {
            return videos;
        }

        if let Ok(entries) = fs::read_dir(&self.target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("mov") ||
                   path.extension().and_then(|e| e.to_str()) == Some("mp4") {
                    
                    if let Ok(metadata) = fs::metadata(&path) {
                        videos.push(VideoFile {
                            name: path.file_name().unwrap().to_string_lossy().to_string(),
                            path: path.clone(),
                            size: metadata.len(),
                            created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                        });
                    }
                }
            }
        }

        // Sort by most recently modified
        videos.sort_by(|a, b| b.modified.cmp(&a.modified));
        videos
    }

    fn is_target_directory_empty(&self) -> bool {
        self.get_existing_videos().is_empty()
    }

    async fn open_video_settings(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::info("🔧 Opening System Preferences > video...");

        // Use AppleScript to open video settings
        let script = r#"tell application "System Preferences"
    activate
    set current pane to pane "com.apple.preference.desktopscreeneffect"
end tell"#;

        let output = Command::new("osascript")
            .args(["-e", script])
            .output()?;

        if output.status.success() {
            logger::success(" System Preferences opened");
            Ok(true)
        } else {
            logger::warning("Could not open System Preferences automatically");
            logger::info("Please manually open: System Preferences > video");
            Ok(false)
        }
    }

    async fn open_finder_at_video_dir(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::info(" Opening Finder at video directory...");

        let output = Command::new("open")
            .arg(&self.target_dir)
            .output()?;

        if output.status.success() {
            logger::success(" Finder opened at video directory");
            Ok(true)
        } else {
            logger::warning("Could not open Finder automatically");
            logger::info(&format!("Please manually open: {}", self.target_dir.display()));
            Ok(false)
        }
    }

    async fn wait_for_video_setup(&self) -> Result<VideoFile, Box<dyn std::error::Error>> {
        logger::info(" Waiting for you to download a landscape video...");
        logger::info(" Steps:");
        logger::info("   1. In System Preferences > Wallpaper");
        logger::info("   2. Scroll to \"Landscape\" section");
        logger::info("   3. Download any landscape video (e.g., \"Sonoma Horizon\")");
        logger::info("   4. This tool will detect it automatically");
        
        let mut attempts = 0;
        
        while attempts < self.retry_attempts {
            let videos = self.get_existing_videos();
            
            if !videos.is_empty() {
                logger::success(&format!(" Detected video: {}", videos[0].name));
                return Ok(videos[0].clone());
            }
            
            // Show progress dots
            print!(".");
            std::io::stdout().flush().ok();
            tokio::time::sleep(self.retry_interval).await;
            attempts += 1;
        }
        
        println!(); // New line after dots
        Err("Timeout waiting for video setup. Please download a landscape video and try again.".into())
    }

    async fn create_backup(&self, video_file: &VideoFile) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        utils::ensure_directory_exists(&self.backup_dir)?;

        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let backup_name = format!("{}_backup_{}.mov", 
            video_file.name.trim_end_matches(".mov").trim_end_matches(".mp4"), 
            timestamp);
        let backup_path = self.backup_dir.join(&backup_name);

        fs::copy(&video_file.path, &backup_path)?;

        // Fix permissions for the backup file
        logger::info("🔧 Fixing backup file permissions...");
        let permission_fixed = utils::fix_file_permissions(&backup_path)?;

        if permission_fixed {
            logger::success(&format!(" Backup created with proper permissions: {}", backup_name));
        } else {
            logger::success(&format!(" Backup created: {}", backup_name));
            logger::warning("  Backup file may require sudo to delete - run cleanup utility if needed");
        }

        Ok(Some(backup_path))
    }

    async fn install_video(&self, video_path: &Path, target_video_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let target_path = self.target_dir.join(target_video_name);

        logger::info(&format!(" Installing video: {}", target_video_name));

        // Copy video to target location
        fs::copy(video_path, &target_path)?;

        // Verify installation
        if target_path.exists() {
            if let Ok(stats) = fs::metadata(&target_path) {
                logger::success(" video installed successfully");
                logger::stats(&format!(" Size: {}", utils::format_file_size(Some(stats.len()))));

                // Refresh video system to ensure animation works
                self.refresh_video_system().await?;

                return Ok(true);
            }
        }
        Err("Installation verification failed".into())
    }

    async fn refresh_video_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Refreshing video system to ensure animation works...");

        // Method 1: Restart the video daemon
        self.restart_video_daemon().await?;

        // Method 2: Force refresh through AppleScript
        self.force_video_refresh().await?;

        logger::success(" video system refreshed");
        logger::info(" If video appears static after screen lock, run: cargo run --bin refresh");

        Ok(())
    }

    async fn restart_video_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Restarting video daemon...");

        let commands = [
            vec!["sudo", "launchctl", "unload", "/System/Library/LaunchDaemons/com.apple.idleassetsd.plist"],
            vec!["sudo", "launchctl", "load", "/System/Library/LaunchDaemons/com.apple.idleassetsd.plist"],
        ];

        for command in &commands {
            let output = Command::new(command[0])
                .args(&command[1..])
                .output()?;
            
            if !output.status.success() {
                logger::warning("  Could not restart daemon (this is normal on some macOS versions)");
                break;
            }
        }

        logger::success(" video daemon restarted");
        Ok(())
    }

    async fn force_video_refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Forcing video refresh...");

        // Method 1: Desktop refresh via AppleScript
        let script = r#"tell application "System Events"
    tell every desktop
        set picture rotation to 0
        delay 0.5
        set picture rotation to 1
        delay 0.5
        set picture rotation to 0
    end tell
end tell"#;

        let output = Command::new("osascript")
            .args(["-e", script])
            .output()?;

        if !output.status.success() {
            // Method 2: Touch video files as fallback
            let touch_command = format!("find \"{}\" -name \"*.mov\" -exec touch {{}} \\; 2>/dev/null", 
                self.target_dir.display());
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&touch_command)
                .output()?;
            
            logger::warning("  Could not force video refresh");
        } else {
            logger::success(" video refresh triggered");
        }

        Ok(())
    }

    async fn select_video_from_list(&self, videos: &[VideoFile]) -> Result<Option<VideoFile>, Box<dyn std::error::Error>> {
        logger::video("  Multiple videos found in directory");
        logger::info(" Opening Finder to help you identify the current video...");

        // Open Finder to help user identify current video
        self.open_finder_at_video_dir().await?;

        println!();
        logger::info(" Available videos:");
        println!();

        for (i, video) in videos.iter().enumerate() {
            let created_date = chrono::DateTime::<chrono::Local>::from(video.created).format("%Y-%m-%d %H:%M");
            let size = utils::format_file_size(Some(video.size));

            println!("  {}. {}", i + 1, video.name);
            println!("      Created: {}", created_date);
            println!("      Size: {}", size);
            println!();
        }

        logger::info(" Instructions:");
        logger::info("   1. Check which video is currently active in System Preferences");
        logger::info("   2. Find the matching file in the Finder window that opened");
        logger::info("   3. Enter the number corresponding to that video");
        println!();

        // Simple prompt for user input
        print!(" Select video to replace (1-{}) or 'c' to cancel: ", videos.len());
        std::io::stdout().flush().ok();

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            let trimmed_input = input.trim();
            
            if trimmed_input.to_lowercase() == "c" || trimmed_input.to_lowercase() == "cancel" {
                return Ok(None);
            }
            
            match trimmed_input.parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= videos.len() => {
                    return Ok(Some(videos[choice - 1].clone()));
                }
                _ => {
                    logger::warning(&format!(" Invalid choice. Please enter a number between 1 and {}, or 'c' to cancel.", videos.len()));
                }
            }
        }
    }

    async fn get_user_confirmation(&self, selected_video: &VideoFile, new_video_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        logger::warning(&format!("  About to replace: {}", selected_video.name));
        logger::info(&format!(" Current size: {}", utils::format_file_size(Some(selected_video.size))));

        if let Ok(new_stats) = fs::metadata(new_video_path) {
            logger::info(&format!(" New video size: {}", utils::format_file_size(Some(new_stats.len()))));
        }

        print!("\n Proceed with replacement? (y/N): ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
    }

    pub async fn setup_video(&self, video_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        logger::header("  Video Installation");
        
        // Check directory access
        let has_access = self.check_customer_directory().await?;
        if !has_access {
            return Err("Cannot access video directory. Please check permissions.".into());
        }
        
        // Check if directory is empty
        if self.is_target_directory_empty() {
            logger::warning(" Video directory is empty");
            logger::info(" You need to download a landscape video first");
            
            // Open System Preferences
            self.open_video_settings().await?;
            
            // Wait for user to setup video
            let video_file = self.wait_for_video_setup().await?;
            
            // Create backup
            self.create_backup(&video_file).await?;
            
            // Install new video
            let success = self.install_video(video_path, &video_file.name).await?;
            return Ok(success);
        } else {
            // Directory has existing videos
            let existing_videos = self.get_existing_videos();

            if existing_videos.is_empty() {
                logger::warning("No .mov/.mp4 files found in video directory");
                logger::info("You need to download a landscape video first");

                // Open System Preferences
                self.open_video_settings().await?;

                // Wait for user to setup video
                let video_file = self.wait_for_video_setup().await?;

                // Create backup
                self.create_backup(&video_file).await?;

                // Install new video
                let success = self.install_video(video_path, &video_file.name).await?;
                return Ok(success);
            } else if existing_videos.len() == 1 {
                // Single video found - use existing logic
                let target_video = &existing_videos[0];

                // Get user confirmation
                let confirmed = self.get_user_confirmation(target_video, video_path).await?;
                if !confirmed {
                    logger::info(" Video installation cancelled by user");
                    return Ok(false);
                }

                // Create backup
                self.create_backup(target_video).await?;

                // Install new video
                let success = self.install_video(video_path, &target_video.name).await?;
                return Ok(success);
            } else {
                // Multiple videos found - let user choose
                logger::info(&format!(" Found {} videos in directory", existing_videos.len()));

                let selected_video = self.select_video_from_list(&existing_videos).await?;
                if let Some(video) = selected_video {
                    // Get user confirmation for the selected video
                    let confirmed = self.get_user_confirmation(&video, video_path).await?;
                    if !confirmed {
                        logger::info(" Video installation cancelled by user");
                        return Ok(false);
                    }

                    // Create backup
                    self.create_backup(&video).await?;

                    // Install new video
                    let success = self.install_video(video_path, &video.name).await?;
                    return Ok(success);
                } else {
                    logger::info(" Video installation cancelled by user");
                    return Ok(false);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoFile {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub created: std::time::SystemTime,
    pub modified: std::time::SystemTime,
}
