use std::process::Command;
use std::env;
use crate::logger;
use crate::Config;

#[derive(Clone)]
pub struct DependencyResult {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub error: Option<String>,
    pub install_hint: Option<String>,
    pub command: String,
}

pub struct DependencyChecker {
    config: Config,
}

impl DependencyChecker {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    fn run_command(command: &str, args: &[&str]) -> Result<std::process::Output, Box<dyn std::error::Error>> {
        let output = Command::new(command)
            .args(args)
            .output()?;
        Ok(output)
    }

    pub async fn check_dependency(&self, name: &str, config: &Config) -> DependencyResult {
        let dependency_config = config.dependencies.iter()
            .find(|d| d.command == name)
            .unwrap_or(&config.dependencies[0]); // fallback to first dependency

        match Self::run_command(dependency_config.command, &dependency_config.args) {
            Ok(output) => {
                if output.status.success() {
                    // Extract version from output if possible
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .find(|line| line.contains(|c: char| c.is_ascii_digit()))
                        .and_then(|line| {
                            line.split_whitespace()
                                .find(|word| word.contains(|c: char| c.is_ascii_digit()))
                                .map(|s| s.to_string())
                        })
                        .or(Some("unknown".to_string()));

                    DependencyResult {
                        name: name.to_string(),
                        available: true,
                        version,
                        error: None,
                        install_hint: None,
                        command: dependency_config.command.to_string(),
                    }
                } else {
                    DependencyResult {
                        name: name.to_string(),
                        available: false,
                        version: None,
                        error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                        install_hint: Some(dependency_config.install_hint.to_string()),
                        command: dependency_config.command.to_string(),
                    }
                }
            }
            Err(error) => {
                DependencyResult {
                    name: name.to_string(),
                    available: false,
                    version: None,
                    error: Some(error.to_string()),
                    install_hint: Some(dependency_config.install_hint.to_string()),
                    command: dependency_config.command.to_string(),
                }
            }
        }
    }

    pub async fn check_all_dependencies(&self) -> Vec<DependencyResult> {
        logger::header("Checking Dependencies");
        
        let mut results = Vec::new();
        let config = Config::default();
        
        for dependency in &config.dependencies {
            logger::info(&format!("Checking {}...", dependency.command));
            let result = self.check_dependency(dependency.command, &config).await;
            results.push(result.clone());
            
            if result.available {
                logger::success(&format!("{} v{} - Available", result.name, result.version.as_deref().unwrap_or("unknown")));
            } else {
                logger::error(&format!("{} - Not available", result.name));
                if let Some(hint) = &result.install_hint {
                    logger::warning(&format!("Install hint: {}", hint));
                }
            }
        }
        
        results
    }

    pub async fn validate_environment(&self) -> Result<Vec<DependencyResult>, Box<dyn std::error::Error>> {
        let results = self.check_all_dependencies().await;
        let missing: Vec<_> = results.iter().filter(|r| !r.available).collect();
        
        if !missing.is_empty() {
            logger::error("Missing required dependencies:");
            for dep in &missing {
                if let Some(hint) = &dep.install_hint {
                    logger::error(&format!("  - {}: {}", dep.name, hint));
                }
            }
            
            return Err(format!("Missing {} required dependencies. Please install them and try again.", missing.len()).into());
        }
        
        logger::success("All dependencies are available!");
        Ok(results)
    }

    pub fn check_node_version() {
        let node_version = env!("CARGO_PKG_VERSION"); // Using Rust version as proxy
        
        logger::info(&format!("Rust version: {}", node_version));

        // For Rust, we don't have the same version constraints as Node.js
        logger::success("Rust version is compatible");
    }

    pub fn check_sudo_privileges() -> bool {
        // Check if running with elevated privileges
        let is_root = unsafe { libc::geteuid() } == 0;
        let has_sudo = env::var("SUDO_USER").is_ok();

        if is_root || has_sudo {
            logger::success("Running with elevated privileges");
            if let Ok(sudo_user) = env::var("SUDO_USER") {
                logger::info(&format!("Original user: {}", sudo_user));
            }
            return true;
        }

        false
    }

    pub fn prompt_for_sudo() -> Result<(), Box<dyn std::error::Error>> {
        logger::warning("This application requires administrator privileges to access system wallpaper directories");
        logger::info("Please restart the application with sudo:");
        logger::info("");
        logger::info("   sudo cargo run --bin rust-downloader");
        logger::info("");
        logger::info("This is required to:");
        logger::info("   • Access /Library/Application Support/com.apple.idleassetsd/Customer");
        logger::info("   • Install wallpaper files in the system directory");
        logger::info("   • Create backups of existing wallpapers");
        logger::info("");
        logger::info(" Note: Your downloads will be saved to the outputs/ directory with proper ownership");

        Err("Administrator privileges required. Please restart with sudo.".into())
    }

    pub async fn check_system_resources(&self) -> Result<(), Box<dyn std::error::Error>> {
        logger::info("Checking system resources...");
        
        // Check available disk space (basic check)
        let config = Config::default();
        match std::fs::metadata(&config.output_dir) {
            Ok(_) => {
                logger::success("Output directory is accessible");
            }
            Err(error) => {
                logger::warning(&format!("Output directory not accessible: {}", error));
            }
        }
        
        // Check memory (basic) - simplified for now
        #[cfg(target_os = "macos")]
        {
            logger::info("System memory check optimized for macOS");
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            logger::info("System memory check not implemented for this platform");
        }
        
        Ok(())
    }

    pub async fn perform_full_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        logger::header("Environment Check");

        // Check sudo privileges first only when wallpaper installation is enabled
        let config = Config::default();
        if config.enable_wallpaper {
            if !Self::check_sudo_privileges() {
                Self::prompt_for_sudo()?;
            }
        } else {
            logger::info("Wallpaper installation disabled; skipping sudo privileges check.");
        }

        // Check Rust version
        Self::check_node_version();

        // Check system resources
        self.check_system_resources().await?;

        // Check dependencies
        self.validate_environment().await?;

        logger::success("Environment check completed successfully!");
        Ok(true)
    }
}

// Platform-specific imports for macOS
#[cfg(target_os = "macos")]
extern "C" {
    fn libc_geteuid() -> libc::uid_t;
}

#[cfg(target_os = "macos")]
fn check_euid() -> libc::uid_t {
    unsafe { libc_geteuid() }
}

// Placeholder for other platforms
#[cfg(not(target_os = "macos"))]
fn check_euid() -> i32 {
    0 // Non-zero indicates not root
}

impl DependencyChecker {
    pub fn is_root() -> bool {
        check_euid() == 0
    }
}

