use std::process::Command;
use std::time::Duration;
use rust_downloader::{logger, Config, wallpaper_manager};
use rust_downloader::utils;
use std::io::Write;

pub struct RefreshUtility {
    customer_dir: std::path::PathBuf,
    target_dir: std::path::PathBuf,
}

impl RefreshUtility {
    pub fn new() -> Self {
        let config = Config::default();
        let customer_dir = std::path::PathBuf::from(config.wallpaper_settings.customer_dir);
        let target_dir = customer_dir.join(config.wallpaper_settings.target_sub_dir);
        
        Self {
            customer_dir,
            target_dir,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        logger::header(" Wallpaper Refresh Utility");
        logger::info("Fix wallpaper animation issues and refresh the wallpaper system");
        logger::info("═══════════════════════════════════════════════════");

        // Check if we have access to wallpaper directories
        if !self.customer_dir.exists() {
            logger::error(" Customer directory not found");
            logger::info("This utility requires administrator privileges");
            logger::info("Please run with: sudo cargo run --bin refresh");
            return Err("Customer directory not accessible".into());
        }

        if !self.target_dir.exists() {
            logger::warning(" Target wallpaper directory not found");
            logger::info("You may need to set up a live wallpaper first");
            return Err("Target directory not found".into());
        }

        // Show current wallpaper status
        self.show_wallpaper_status().await?;

        // Perform refresh operations
        logger::info("Starting wallpaper refresh process...");
        
        // Method 1: Restart wallpaper daemon
        self.restart_wallpaper_daemon().await?;
        
        // Method 2: Force desktop refresh
        self.force_desktop_refresh().await?;
        
        // Method 3: Touch wallpaper files
        self.touch_wallpaper_files().await?;
        
        // Method 4: Refresh through System Events
        self.refresh_via_system_events().await?;

        logger::success(" Wallpaper refresh completed!");
        logger::info(" If your wallpaper still appears static:");
        logger::info("   1. Try locking and unlocking your screen");
        logger::info("   2. Restart your Mac");
        logger::info("   3. Check System Preferences > Desktop & Screen Saver");

        Ok(())
    }

    async fn show_wallpaper_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Current wallpaper status:");
        
        // List wallpapers in the directory
        if let Ok(entries) = std::fs::read_dir(&self.target_dir) {
            let mut wallpapers = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("mov") ||
                   path.extension().and_then(|e| e.to_str()) == Some("mp4") {
                    
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        wallpapers.push((path, metadata));
                    }
                }
            }

            if wallpapers.is_empty() {
                logger::warning(" No .mov/.mp4 wallpaper files found");
            } else {
                logger::info(&format!(" Found {} wallpaper file(s):", wallpapers.len()));
                for (path, metadata) in &wallpapers {
                    let size = crate::utils::format_file_size(Some(metadata.len()));
                    let modified = match metadata.modified() {
                        Ok(t) => {
                            let dt = chrono::DateTime::<chrono::Local>::from(t);
                            dt.format("%Y-%m-%d %H:%M").to_string()
                        }
                        Err(_) => "Unknown".to_string()
                    };
                    
                    logger::info(&format!("   {} ({} | Modified: {})", 
                        path.file_name().unwrap().to_string_lossy(), 
                        size, 
                        modified));
                }
            }
        } else {
            logger::warning("  Could not read wallpaper directory");
        }

        Ok(())
    }

    async fn restart_wallpaper_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Restarting wallpaper daemon...");

        let commands = vec![
            vec!["sudo", "launchctl", "unload", "/System/Library/LaunchDaemons/com.apple.idleassetsd.plist"],
            vec!["sudo", "launchctl", "load", "/System/Library/LaunchDaemons/com.apple.idleassetsd.plist"],
        ];

        for command in &commands {
            let output = Command::new(command[0])
                .args(&command[1..])
                .output()?;
            
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                logger::warning(&format!(" Daemon command failed: {}", error));
                break;
            }
        }

        logger::success(" Wallpaper daemon restart attempted");
        Ok(())
    }

    async fn force_desktop_refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Forcing desktop refresh...");

        // Use AppleScript to trigger desktop refresh
        let script = r#"tell application "System Events"
    -- Try to trigger a refresh by changing desktop properties
    tell every desktop
        set picture rotation to 0
        delay 0.1
        set picture rotation to 1
        delay 0.1
        set picture rotation to 0
    end tell
end tell"#;

        let output = Command::new("osascript")
            .args(["-e", script])
            .output()?;

        if output.status.success() {
            logger::success(" Desktop refresh triggered");
        } else {
            logger::warning("  Desktop refresh failed - this is normal on some macOS versions");
        }

        Ok(())
    }

    async fn touch_wallpaper_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Touching wallpaper files to trigger refresh...");

        // Find all .mov and .mp4 files and touch them
        let find_command = format!("find \"{}\" -name \"*.mov\" -o -name \"*.mp4\" -exec touch {{}} \\; 2>/dev/null", 
            self.target_dir.display());
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&find_command)
            .output()?;

        if output.status.success() {
            logger::success(" Wallpaper files touched - refresh triggered");
        } else {
            logger::warning(" Could not touch wallpaper files");
        }

        Ok(())
    }

    async fn refresh_via_system_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Refreshing via System Events...");

        // Additional refresh through killall as fallback
        let killall_output = Command::new("killall")
            .args(["Dock"])
            .output()?;

        if killall_output.status.success() {
            logger::info("Dock restarted (may help with wallpaper refresh)");
        }

        // Wait a moment for the system to settle
        tokio::time::sleep(Duration::from_millis(500)).await;

        logger::success("System events refresh completed");
        Ok(())
    }

    pub async fn quick_refresh() -> Result<(), Box<dyn std::error::Error>> {
        logger::header("⚡ Quick Wallpaper Refresh");
        logger::info("Performing rapid wallpaper refresh...");

        // Quick refresh without detailed status
        let mut refresh = RefreshUtility::new();
        
        // Just touch files and restart dock
        refresh.touch_wallpaper_files().await?;
        
        let output = Command::new("killall")
            .args(["Dock"])
            .output()?;
        
        if output.status.success() {
            logger::success("Quick refresh completed!");
        } else {
            logger::warning("  Quick refresh partially completed");
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    // Check for quick refresh flag
    if args.len() > 1 && args[1] == "--quick" {
        RefreshUtility::quick_refresh().await?;
    } else {
        let mut refresh = RefreshUtility::new();
        match refresh.run().await {
            Ok(_) => {
                logger::success(" Wallpaper refresh completed successfully!");
            }
            Err(error) => {
                logger::error(&format!(" Wallpaper refresh failed: {}", error));
                
                // Suggest alternative actions
                logger::info(" Alternative solutions:");
                logger::info("   1. Lock and unlock your screen");
                logger::info("   2. Restart your Mac");
                logger::info("   3. Go to System Preferences > Desktop & Screen Saver");
                logger::info("   4. Try selecting a different wallpaper and then back");
                
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
