use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::io::Write;
use crate::logger;
use crate::Config;
use crate::utils;

pub struct WallpaperManager {
    customer_dir: PathBuf,
    target_dir: PathBuf,
    backup_dir: PathBuf,
    retry_attempts: u32,
    retry_interval: Duration,
}

impl WallpaperManager {
    pub fn new() -> Self {
        let config = Config::default();
        let customer_dir = PathBuf::from(config.wallpaper_settings.customer_dir);
        let target_dir = customer_dir.join(config.wallpaper_settings.target_sub_dir);
        let backup_dir = config.output_dir.join(config.wallpaper_settings.backup_dir);
        
        Self {
            customer_dir,
            target_dir,
            backup_dir,
            retry_attempts: config.wallpaper_settings.max_retry_attempts,
            retry_interval: Duration::from_millis(config.wallpaper_settings.retry_interval),
        }
    }

    async fn check_customer_directory(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.customer_dir.exists() {
            logger::error(" Customer directory not found");
            logger::info("This usually means macOS wallpaper system is not initialized");
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

    fn get_existing_wallpapers(&self) -> Vec<WallpaperFile> {
        let mut wallpapers = Vec::new();
        
        if !self.target_dir.exists() {
            return wallpapers;
        }

        if let Ok(entries) = fs::read_dir(&self.target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("mov") ||
                   path.extension().and_then(|e| e.to_str()) == Some("mp4") {
                    
                    if let Ok(metadata) = fs::metadata(&path) {
                        wallpapers.push(WallpaperFile {
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
        wallpapers.sort_by(|a, b| b.modified.cmp(&a.modified));
        wallpapers
    }

    fn is_target_directory_empty(&self) -> bool {
        self.get_existing_wallpapers().is_empty()
    }

    async fn open_wallpaper_settings(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::info("🔧 Opening System Preferences > Wallpaper...");

        // Use AppleScript to open wallpaper settings
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
            logger::info("Please manually open: System Preferences > Wallpaper");
            Ok(false)
        }
    }

    async fn open_finder_at_wallpaper_dir(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::info(" Opening Finder at wallpaper directory...");

        let output = Command::new("open")
            .arg(&self.target_dir)
            .output()?;

        if output.status.success() {
            logger::success(" Finder opened at wallpaper directory");
            Ok(true)
        } else {
            logger::warning("Could not open Finder automatically");
            logger::info(&format!("Please manually open: {}", self.target_dir.display()));
            Ok(false)
        }
    }

    async fn wait_for_wallpaper_setup(&self) -> Result<WallpaperFile, Box<dyn std::error::Error>> {
        logger::info(" Waiting for you to download a landscape wallpaper...");
        logger::info(" Steps:");
        logger::info("   1. In System Preferences > Wallpaper");
        logger::info("   2. Scroll to \"Landscape\" section");
        logger::info("   3. Download any landscape wallpaper (e.g., \"Sonoma Horizon\")");
        logger::info("   4. This tool will detect it automatically");
        
        let mut attempts = 0;
        
        while attempts < self.retry_attempts {
            let wallpapers = self.get_existing_wallpapers();
            
            if !wallpapers.is_empty() {
                logger::success(&format!(" Detected wallpaper: {}", wallpapers[0].name));
                return Ok(wallpapers[0].clone());
            }
            
            // Show progress dots
            print!(".");
            std::io::stdout().flush().ok();
            tokio::time::sleep(self.retry_interval).await;
            attempts += 1;
        }
        
        println!(); // New line after dots
        Err("Timeout waiting for wallpaper setup. Please download a landscape wallpaper and try again.".into())
    }

    async fn create_backup(&self, wallpaper_file: &WallpaperFile) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        utils::ensure_directory_exists(&self.backup_dir)?;

        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let backup_name = format!("{}_backup_{}.mov", 
            wallpaper_file.name.trim_end_matches(".mov").trim_end_matches(".mp4"), 
            timestamp);
        let backup_path = self.backup_dir.join(&backup_name);

        fs::copy(&wallpaper_file.path, &backup_path)?;

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

    async fn install_wallpaper(&self, video_path: &Path, target_wallpaper_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let target_path = self.target_dir.join(target_wallpaper_name);

        logger::info(&format!(" Installing wallpaper: {}", target_wallpaper_name));

        // Copy video to target location
        fs::copy(video_path, &target_path)?;

        // Verify installation
        if target_path.exists() {
            if let Ok(stats) = fs::metadata(&target_path) {
                logger::success(" Wallpaper installed successfully");
                logger::stats(&format!(" Size: {}", utils::format_file_size(Some(stats.len()))));

                // Refresh wallpaper system to ensure animation works
                self.refresh_wallpaper_system().await?;

                return Ok(true);
            }
        }
        Err("Installation verification failed".into())
    }

    async fn refresh_wallpaper_system(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Refreshing wallpaper system to ensure animation works...");

        // Method 1: Restart the wallpaper daemon
        self.restart_wallpaper_daemon().await?;

        // Method 2: Force refresh through AppleScript
        self.force_wallpaper_refresh().await?;

        logger::success(" Wallpaper system refreshed");
        logger::info(" If wallpaper appears static after screen lock, run: cargo run --bin refresh");

        Ok(())
    }

    async fn restart_wallpaper_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Restarting wallpaper daemon...");

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

        logger::success(" Wallpaper daemon restarted");
        Ok(())
    }

    async fn force_wallpaper_refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Forcing wallpaper refresh...");

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
            // Method 2: Touch wallpaper files as fallback
            let touch_command = format!("find \"{}\" -name \"*.mov\" -exec touch {{}} \\; 2>/dev/null", 
                self.target_dir.display());
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&touch_command)
                .output()?;
            
            logger::warning("  Could not force wallpaper refresh");
        } else {
            logger::success(" Wallpaper refresh triggered");
        }

        Ok(())
    }

    async fn select_wallpaper_from_list(&self, wallpapers: &[WallpaperFile]) -> Result<Option<WallpaperFile>, Box<dyn std::error::Error>> {
        logger::wallpaper("  Multiple wallpapers found in directory");
        logger::info(" Opening Finder to help you identify the current wallpaper...");

        // Open Finder to help user identify current wallpaper
        self.open_finder_at_wallpaper_dir().await?;

        println!();
        logger::info(" Available wallpapers:");
        println!();

        for (i, wallpaper) in wallpapers.iter().enumerate() {
            let created_date = chrono::DateTime::<chrono::Local>::from(wallpaper.created).format("%Y-%m-%d %H:%M");
            let size = utils::format_file_size(Some(wallpaper.size));

            println!("  {}. {}", i + 1, wallpaper.name);
            println!("      Created: {}", created_date);
            println!("      Size: {}", size);
            println!();
        }

        logger::info(" Instructions:");
        logger::info("   1. Check which wallpaper is currently active in System Preferences");
        logger::info("   2. Find the matching file in the Finder window that opened");
        logger::info("   3. Enter the number corresponding to that wallpaper");
        println!();

        // Simple prompt for user input
        print!(" Select wallpaper to replace (1-{}) or 'c' to cancel: ", wallpapers.len());
        std::io::stdout().flush().ok();

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            let trimmed_input = input.trim();
            
            if trimmed_input.to_lowercase() == "c" || trimmed_input.to_lowercase() == "cancel" {
                return Ok(None);
            }
            
            match trimmed_input.parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= wallpapers.len() => {
                    return Ok(Some(wallpapers[choice - 1].clone()));
                }
                _ => {
                    logger::warning(&format!(" Invalid choice. Please enter a number between 1 and {}, or 'c' to cancel.", wallpapers.len()));
                }
            }
        }
    }

    async fn get_user_confirmation(&self, selected_wallpaper: &WallpaperFile, new_video_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        logger::warning(&format!("  About to replace: {}", selected_wallpaper.name));
        logger::info(&format!(" Current size: {}", utils::format_file_size(Some(selected_wallpaper.size))));

        if let Ok(new_stats) = fs::metadata(new_video_path) {
            logger::info(&format!(" New video size: {}", utils::format_file_size(Some(new_stats.len()))));
        }

        print!("\n Proceed with replacement? (y/N): ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
    }

    pub async fn setup_wallpaper(&self, video_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        logger::header("  Wallpaper Installation");
        
        // Check directory access
        let has_access = self.check_customer_directory().await?;
        if !has_access {
            return Err("Cannot access wallpaper directory. Please check permissions.".into());
        }
        
        // Check if directory is empty
        if self.is_target_directory_empty() {
            logger::warning(" Wallpaper directory is empty");
            logger::info(" You need to download a landscape wallpaper first");
            
            // Open System Preferences
            self.open_wallpaper_settings().await?;
            
            // Wait for user to setup wallpaper
            let wallpaper_file = self.wait_for_wallpaper_setup().await?;
            
            // Create backup
            self.create_backup(&wallpaper_file).await?;
            
            // Install new wallpaper
            let success = self.install_wallpaper(video_path, &wallpaper_file.name).await?;
            return Ok(success);
        } else {
            // Directory has existing wallpapers
            let existing_wallpapers = self.get_existing_wallpapers();

            if existing_wallpapers.is_empty() {
                logger::warning("No .mov/.mp4 files found in wallpaper directory");
                logger::info("You need to download a landscape wallpaper first");

                // Open System Preferences
                self.open_wallpaper_settings().await?;

                // Wait for user to setup wallpaper
                let wallpaper_file = self.wait_for_wallpaper_setup().await?;

                // Create backup
                self.create_backup(&wallpaper_file).await?;

                // Install new wallpaper
                let success = self.install_wallpaper(video_path, &wallpaper_file.name).await?;
                return Ok(success);
            } else if existing_wallpapers.len() == 1 {
                // Single wallpaper found - use existing logic
                let target_wallpaper = &existing_wallpapers[0];

                // Get user confirmation
                let confirmed = self.get_user_confirmation(target_wallpaper, video_path).await?;
                if !confirmed {
                    logger::info(" Wallpaper installation cancelled by user");
                    return Ok(false);
                }

                // Create backup
                self.create_backup(target_wallpaper).await?;

                // Install new wallpaper
                let success = self.install_wallpaper(video_path, &target_wallpaper.name).await?;
                return Ok(success);
            } else {
                // Multiple wallpapers found - let user choose
                logger::info(&format!(" Found {} wallpapers in directory", existing_wallpapers.len()));

                let selected_wallpaper = self.select_wallpaper_from_list(&existing_wallpapers).await?;
                if let Some(wallpaper) = selected_wallpaper {
                    // Get user confirmation for the selected wallpaper
                    let confirmed = self.get_user_confirmation(&wallpaper, video_path).await?;
                    if !confirmed {
                        logger::info(" Wallpaper installation cancelled by user");
                        return Ok(false);
                    }

                    // Create backup
                    self.create_backup(&wallpaper).await?;

                    // Install new wallpaper
                    let success = self.install_wallpaper(video_path, &wallpaper.name).await?;
                    return Ok(success);
                } else {
                    logger::info(" Wallpaper installation cancelled by user");
                    return Ok(false);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WallpaperFile {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub created: std::time::SystemTime,
    pub modified: std::time::SystemTime,
}
