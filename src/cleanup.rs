use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::Write;
use rust_downloader::{logger, utils, Config};
use std::os::unix::fs::MetadataExt;

pub struct CleanupUtility {
    output_dir: PathBuf,
    backup_dir: PathBuf,
    problematic_files: Vec<PathBuf>,
}

impl CleanupUtility {
    pub fn new() -> Self {
        let config = Config::default();
        let output_dir = config.output_dir.clone();
        let backup_dir = output_dir.join(config.wallpaper_settings.backup_dir);
        
        Self {
            output_dir,
            backup_dir,
            problematic_files: Vec::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        logger::header(" Rust File Cleanup Utility");
        logger::info("Fix permission issues with video files and backups");
        logger::info("═══════════════════════════════════════════════════");

        // Check if running with appropriate permissions
        #[cfg(target_os = "macos")]
        {
            use libc::{geteuid};
            unsafe {
                if geteuid() == 0 {
                    logger::warning("  Running as root - will fix ownership for original user");
                }
            }
        }

        // Scan for problematic files
        self.scan_for_problematic_files().await;

        if self.problematic_files.is_empty() {
            logger::success(" No files with permission issues found!");
            logger::info(" All your video files should be easily deletable");
            return Ok(());
        }

        // Display found files
        self.display_problematic_files();

        // Get user choice
        let action = self.get_user_choice().await?;

        match action.as_str() {
            "fix" => self.fix_permissions().await?,
            "delete" => self.delete_files().await?,
            "exit" => {
                logger::info(" Cleanup cancelled");
            }
            _ => {}
        }

        Ok(())
    }

    async fn scan_for_problematic_files(&mut self) {
        logger::info(" Scanning for files with permission issues...");

        let dirs_to_scan = vec![&self.output_dir, &self.backup_dir];

        for dir in &dirs_to_scan {
            if dir.exists() {
                logger::info(&format!(" Scanning: {}", dir.display()));
                let extensions = vec!["mov", "mp4"];
                let files = utils::find_files_with_permission_issues(dir, &extensions);
                self.problematic_files.extend(files);
            }
        }

        logger::info(&format!(" Found {} files with permission issues", self.problematic_files.len()));
    }

    fn display_problematic_files(&self) {
        logger::warning("  Files requiring sudo to delete:");
        logger::info("");

        for (index, file_path) in self.problematic_files.iter().enumerate() {
            match fs::metadata(file_path) {
                Ok(stats) => {
                    let size = utils::format_file_size(Some(stats.len()));
                    let relative_path = std::path::Path::new(".")
                        .join(file_path.strip_prefix(&std::env::current_dir().unwrap_or(PathBuf::from("."))).unwrap_or(file_path));
                    
                    println!("   {}. {}", index + 1, relative_path.display());
                    println!("      Size: {} | Owner: {}", size, stats.uid());
                }
                Err(_) => {
                    println!("   {}. {} (error reading stats)", index + 1, file_path.display());
                }
            }
        }

        println!();
    }

    async fn get_user_choice(&self) -> Result<String, Box<dyn std::error::Error>> {
        logger::info(" What would you like to do?");
        logger::info(" 1. Fix permissions (make files deletable without sudo)");
        logger::info(" 2. Delete all problematic files");
        logger::info(" 3. Exit without changes");
        logger::info("");

        print!("Enter your choice (1/2/3): ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let choice = input.trim();
        Ok(match choice {
            "1" => "fix".to_string(),
            "2" => "delete".to_string(),
            "3" | _ => "exit".to_string(),
        })
    }

    async fn fix_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(" Fixing file permissions...");

        let mut success_count = 0;
        let mut fail_count = 0;

        for file_path in &self.problematic_files {
            logger::info(&format!(" Fixing: {}", file_path.file_name().unwrap().to_string_lossy()));

            match utils::fix_file_permissions(file_path) {
                Ok(true) => {
                    logger::success(&format!(" Fixed: {}", file_path.file_name().unwrap().to_string_lossy()));
                    success_count += 1;
                }
                Ok(false) => {
                    logger::warning(&format!("  Failed to fix: {}", file_path.file_name().unwrap().to_string_lossy()));
                    fail_count += 1;
                }
                Err(error) => {
                    logger::error(&format!(" Error fixing {}: {}", 
                        file_path.file_name().unwrap().to_string_lossy(), 
                        error));
                    fail_count += 1;
                }
            }
        }

        logger::info("");
        logger::info(" Permission Fix Summary:");
        logger::success(&format!(" Successfully fixed: {} files", success_count));
        if fail_count > 0 {
            logger::warning(&format!("  Failed to fix: {} files", fail_count));
            logger::info(" You may need to run this utility with sudo for remaining files");
        }

        Ok(())
    }

    async fn delete_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let confirmed = self.confirm_deletion().await?;

        if !confirmed {
            logger::info(" Deletion cancelled");
            return Ok(());
        }

        logger::info("  Deleting files...");

        let mut success_count = 0;
        let mut fail_count = 0;

        for file_path in &self.problematic_files {
            logger::info(&format!("  Deleting: {}", file_path.file_name().unwrap().to_string_lossy()));

            // Try to fix permissions first, then delete
            if let Err(error) = utils::fix_file_permissions(file_path) {
                logger::warning(&format!("  Could not fix permissions: {}", error));
            }

            match fs::remove_file(file_path) {
                Ok(_) => {
                    logger::success(&format!("Deleted: {}", file_path.file_name().unwrap().to_string_lossy()));
                    success_count += 1;
                }
                Err(error) => {
                    logger::error(&format!(" Failed to delete {}: {}", 
                        file_path.file_name().unwrap().to_string_lossy(), 
                        error));
                    fail_count += 1;
                }
            }
        }

        logger::info("");
        logger::info("📊 Deletion Summary:");
        logger::success(&format!(" Successfully deleted: {} files", success_count));
        if fail_count > 0 {
            logger::warning(&format!(" Failed to delete: {} files", fail_count));
            logger::info(" You may need to run this utility with sudo for remaining files");
        }

        Ok(())
    }

    async fn confirm_deletion(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::warning("  This will permanently delete all listed files!");
        logger::info("");

        print!(" Are you sure you want to delete these files? (y/N): ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    
    let mut cleanup = CleanupUtility::new();
    
    match cleanup.run().await {
        Ok(_) => {
            logger::success(" Cleanup completed successfully!");
        }
        Err(error) => {
            logger::error(&format!(" Cleanup failed: {}", error));
            std::process::exit(1);
        }
    }

    Ok(())
}
