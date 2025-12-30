use serde_json::Value;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use crate::utils;
use crate::logger;
use crate::config::Config;
use crate::video_info::{SelectedFormats, VideoFormat, AudioFormat};

pub struct Downloader {
    is_downloading: bool,
    current_process: Option<std::process::Child>,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            is_downloading: false,
            current_process: None,
        }
    }

    fn create_output_filename(&self, info: &crate::video_info::VideoInfo, video_format: &VideoFormat) -> String {
        let quality = format!("{}p_{}fps", video_format.height.unwrap_or(0), video_format.fps.unwrap_or(30.0) as u32);
        utils::create_safe_filename(
            &info.title,
            &quality,
            self.get_extension(),
            Config::default().file_naming.max_title_length,
        )
    }

    fn get_extension(&self) -> &'static str {
        Config::default().download_settings.merge_output_format
    }

    fn check_existing_video(&self, output_path: &Path) -> (bool, Option<PathBuf>, bool) {
        // First check for .mov version (final format)
        let mov_path = output_path.with_extension("mov");
        if mov_path.exists() {
            if let Ok(stats) = fs::metadata(&mov_path) {
                logger::success(&format!("📁 Final .mov video already exists: {}", mov_path.file_name().unwrap().to_string_lossy()));
                logger::stats(&format!("📊 Size: {}", utils::format_file_size(Some(stats.len()))));
                return (true, Some(mov_path), false);
            }
        }

        // Then check for original format (needs conversion)
        if output_path.exists() {
            if let Ok(stats) = fs::metadata(output_path) {
                logger::success(&format!("📁 Source video exists: {}", output_path.file_name().unwrap().to_string_lossy()));
                logger::stats(&format!("📊 Size: {}", utils::format_file_size(Some(stats.len()))));
                logger::info("🔄 Will convert to .mov format for wallpaper compatibility");
                return (true, Some(output_path.to_path_buf()), true);
            }
        }

        (false, None, false)
    }

    fn check_video_quality(&self, video_format: &VideoFormat) {
        let resolution = video_format.height.unwrap_or(0);
        let min_recommended = Config::default().video_settings.min_recommended_resolution;

        if resolution < min_recommended as u32 {
            logger::warning(" Video quality warning!");
            logger::warning(&format!("Selected: {}p ({}x{})", resolution, video_format.width.unwrap_or(0), resolution));
            logger::warning(&format!("Recommended: {}p for best wallpaper quality", min_recommended));
            logger::info("Consider finding a higher quality version for better results");
        } else {
            logger::success(&format!("Excellent quality: {}p", resolution));
        }
    }

    async fn get_video_duration(&self, input_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let output = Command::new("ffprobe")
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                input_path.to_str().unwrap()
            ])
            .output()?;

        if !output.status.success() {
            return Err(format!("ffprobe failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        let info: Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
        let duration = info.get("format")
            .and_then(|f| f.get("duration"))
            .and_then(|d| d.as_str())
            .and_then(|d| d.parse::<f64>().ok())
            .ok_or("Failed to parse video duration")?;

        Ok(duration)
    }

    async fn extend_video(&self, input_path: &Path, min_duration: f64) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let original_duration = self.get_video_duration(input_path).await?;
        let output_path = input_path.with_extension("extended.mp4");

        // Calculate how many loops we need
        let loops_needed = (min_duration / original_duration).ceil() as i32;

        logger::info(&format!("Creating extended version by looping the video..."));
        logger::info(&format!("Original: {} → Target: {} ({} loops)", 
            utils::format_time(original_duration), 
            utils::format_time(min_duration), 
            loops_needed));

        // Use FFmpeg to loop the video
        let args = [
            "-stream_loop", "-1", // Loop indefinitely
            "-i", input_path.to_str().unwrap(),
            "-t", &min_duration.to_string(), // Stop at minimum duration
            "-c", "copy", // Copy streams without re-encoding for speed
            "-avoid_negative_ts", "make_zero",
            "-fflags", "+genpts", // Generate presentation timestamps
            "-y", // Overwrite output file
            output_path.to_str().unwrap(),
        ];

        let mut child = Command::new("ffmpeg")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        // Monitor progress
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("time=") {
                    // Progress monitoring could be added here
                }
            }
        }

        let status = child.wait()?;

        if status.success() {
            if output_path.exists() {
                if let Ok(stats) = fs::metadata(&output_path) {
                    logger::success(&format!("Video extended successfully: {}", utils::format_file_size(Some(stats.len()))));
                    logger::info(&format!("Extended duration: {}", utils::format_time(min_duration)));
                    return Ok(output_path);
                }
            }
            return Err("Extended video file not found after processing".into());
        } else {
            return Err(format!("Video extension failed with code {:?}", status.code()).into());
        }
    }

    fn fix_file_permissions(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        logger::info(&format!("🔧 Fixing file permissions for: {}", file_path.file_name().unwrap().to_string_lossy()));

        let success = utils::fix_file_permissions(file_path)?;

        if success {
            logger::success("File permissions fixed successfully");
        } else {
            logger::warning("Failed to fix file permissions completely");
            logger::info("You may need to run the cleanup utility later");
        }

        Ok(())
    }

    async fn cleanup_source_file(&self, source_path: &Path, converted_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Verify the converted file exists and has reasonable size
        if !converted_path.exists() {
            logger::warning("Converted file not found, keeping source file");
            return Ok(());
        }

        let source_stats = fs::metadata(source_path)?;
        let converted_stats = fs::metadata(converted_path)?;

        // Basic sanity check - converted file should be at least 10% of source size
        if converted_stats.len() < source_stats.len() / 10 {
            logger::warning(" Converted file seems too small, keeping source file for safety");
            return Ok(());
        }

        // Only clean up MP4 files (not other formats)
        if source_path.extension().and_then(|e| e.to_str()) == Some("mp4") {
            logger::info(&format!("Cleaning up source MP4 file: {}", source_path.file_name().unwrap().to_string_lossy()));

            match fs::remove_file(source_path) {
                Ok(_) => {
                    logger::success("Source MP4 file cleaned up successfully");
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        logger::info(" Fixing permissions before cleanup...");
                        match utils::fix_file_permissions(source_path) {
                            Ok(true) => {
                                match fs::remove_file(source_path) {
                                    Ok(_) => logger::success("Source MP4 file cleaned up after permission fix"),
                                    Err(second_e) => {
                                        logger::warning(&format!("  Could not delete MP4 file: {}", second_e));
                                        logger::info(" You may need to manually delete the MP4 file later");
                                    }
                                }
                            }
                            Ok(false) => {
                                logger::warning(&format!(" Could not delete MP4 file: {}", e));
                                logger::info("You may need to manually delete the MP4 file later");
                            }
                            Err(perm_e) => {
                                logger::warning(&format!("Permission fix failed: {}", perm_e));
                            }
                        }
                    } else {
                        logger::warning(&format!("Failed to clean up source file: {}", e));
                        logger::info(" Source file will be kept for safety");
                    }
                }
            }
        }

        Ok(())
    }

    async fn convert_with_hevc(&self, input_path: &Path, output_path: &Path, mut use_fallback: bool, mut reencode_audio: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config = Config::default();
        let max_attempts = config.conversion_settings.max_attempts;

        for attempt in 1..=max_attempts {
            if attempt > 1 {
                logger::info(&format!("Conversion attempt {}/{}", attempt, max_attempts));
            }

            if use_fallback {
                logger::convert("Converting to HEVC .mov format (software encoding)...");
                logger::warning(" Hardware acceleration not available, using software encoding");
            } else {
                logger::convert("Converting to HEVC .mov format with hardware acceleration...");
                logger::info("Using Apple VideoToolbox for optimal performance");
            }

            logger::info(" Conversion settings:");
            logger::info("   • Codec: HEVC (H.265) 10-bit");
            logger::info("   • Resolution: 4K (3840x2160)");
            logger::info("   • Frame Rate: 60fps");
            logger::info("   • Bitrate: 50 Mbps");

            let video_codec = if use_fallback { "libx265" } else { "hevc_videotoolbox" };
            let pixel_format = "yuv420p10le";

            // Prepare arguments
            let mut args = vec![
                "-y",
                "-i", input_path.to_str().unwrap(),
                "-c:v", video_codec,
                "-tag:v", "hvc1", // Ensure proper HEVC tag for QuickTime compatibility
                "-movflags", "+faststart",
                "-pix_fmt", pixel_format,
                "-r", "60", // Force 60fps for smooth wallpaper
                "-vf", "scale=3840:2160:flags=lanczos", // Ensure 4K resolution
                "-b:v", "50M", // High bitrate for quality (50 Mbps)
                "-maxrate", "60M",
                "-bufsize", "100M"
            ];

            // Add audio codec
            if reencode_audio {
                args.extend_from_slice(&["-c:a", "aac"]);
            } else {
                args.extend_from_slice(&["-c:a", "copy"]);
            }

            // Add profile settings for software encoding
            if use_fallback {
                args.extend_from_slice(&["-profile:v", "main10", "-level", "5.1", "-preset", "medium"]);
            }

            args.push("-y"); // Overwrite output file
            args.push(output_path.to_str().unwrap());

            // Run ffmpeg
            let mut child = Command::new("ffmpeg")
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            let start_time = SystemTime::now();
            let stderr = child.stderr.take().unwrap();
            let reader = BufReader::new(stderr);

            // Collect stderr for error reporting
            let mut stderr_output = String::new();

            // Parse progress
            let mut video_duration = None;
            for line in reader.lines() {
                if let Ok(line) = line {
                    stderr_output.push_str(&line);
                    stderr_output.push('\n');

                    // Extract video duration from initial output
                    if video_duration.is_none() && line.contains("Duration:") {
                        if let Some(duration_match) = line.split("Duration: ").nth(1) {
                            let time_part = duration_match.split(',').next().unwrap_or("");
                            let parts: Vec<&str> = time_part.split(':').collect();
                            if parts.len() >= 3 {
                                let hours = parts[0].parse::<f64>().unwrap_or(0.0);
                                let minutes = parts[1].parse::<f64>().unwrap_or(0.0);
                                let seconds = parts[2].parse::<f64>().unwrap_or(0.0);
                                video_duration = Some(hours * 3600.0 + minutes * 60.0 + seconds);
                            }
                        }
                    }

                    // Extract progress information
                    if let Some(progress_data) = utils::parse_progress(&line) {
                        let (percentage, _, _, eta) = progress_data;
                        let progress_bar = utils::create_progress_bar(percentage, 20);

                        if let Some(_duration) = video_duration {
                            let elapsed = start_time.elapsed()?.as_secs_f64();
                            let eta_text = if percentage > 5.0 {
                                let estimated_total = elapsed / (percentage / 100.0);
                                let eta_secs = (estimated_total - elapsed).max(0.0);
                                format!(" | ETA: {}", utils::format_time(eta_secs))
                            } else {
                                String::new()
                            };

                            logger::progress(&format!("Converting {} | {} ETA: {}{}", progress_bar, eta, eta, eta_text));
                        }
                    }
                }
            }

            let status = child.wait()?;

            if status.success() {
                let conversion_time = start_time.elapsed()?.as_secs_f64();
                logger::success(&format!("HEVC conversion completed in {:.1}s: {}",
                    conversion_time,
                    output_path.file_name().unwrap().to_string_lossy()));

                // Verify output file
                if output_path.exists() {
                    if let Ok(stats) = fs::metadata(output_path) {
                        logger::stats(&format!("HEVC .mov size: {}", utils::format_file_size(Some(stats.len()))));
                        logger::info("Video optimized for macOS live wallpaper with 4K 60fps HEVC");

                        // Fix file permissions and ownership
                        self.fix_file_permissions(output_path)?;

                        return Ok(output_path.to_path_buf());
                    }
                }
                return Err("Conversion completed but output file not found".into());
            } else {
                logger::warning(&format!(" Conversion attempt {} failed with exit code {:?}", attempt, status.code()));

                // Log FFmpeg stderr output for diagnostics
                if !stderr_output.is_empty() {
                    logger::error("FFmpeg error output:");
                    for line in stderr_output.lines().take(10) { // Limit to first 10 lines
                        logger::error(&format!("  {}", line));
                    }
                    if stderr_output.lines().count() > 10 {
                        logger::error("  ... (truncated)");
                    }
                }

                // Determine next attempt settings
                if !use_fallback && attempt < max_attempts {
                    use_fallback = true;
                    logger::info("Next attempt: using software encoding...");
                } else if !reencode_audio && attempt < max_attempts {
                    reencode_audio = true;
                    logger::info("Next attempt: re-encoding audio...");
                } else if attempt >= max_attempts {
                    return Err(format!("FFmpeg HEVC conversion failed after {} attempts with code {:?}. Last error output:\n{}",
                        attempt, status.code(), stderr_output).into());
                }
            }
        }
        
        unreachable!("Should have returned from within the loop")
    }

    async fn convert_to_mov(&self, input_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let output_path = input_path.with_extension("mov");

        if output_path.exists() {
            logger::success(&format!("HEVC .mov version already exists: {}", output_path.file_name().unwrap().to_string_lossy()));
            return Ok(output_path);
        }

        // Check video duration and extend if needed
        let duration = self.get_video_duration(input_path).await?;
        let min_duration = Config::default().video_settings.min_recommended_duration as f64;

        let mut processed_input_path = input_path.to_path_buf();

        if duration < min_duration {
            logger::info(&format!(" Video duration: {} ({:.1}s)", utils::format_time(duration), duration));
            logger::info(" Extending video to minimum 3 minutes for better experience...");
            processed_input_path = self.extend_video(input_path, min_duration).await?;
        } else {
            logger::info(&format!("  Video duration: {}", utils::format_time(duration)));
        }

        // Try hardware-accelerated HEVC first, fallback to software if needed
        let converted_path = self.convert_with_hevc(&processed_input_path, &output_path, false, false).await?;

        // Clean up temporary extended file if created
        if processed_input_path != *input_path {
            if let Err(e) = fs::remove_file(&processed_input_path) {
                logger::warning(&format!("  Could not clean up temporary file: {}", e));
            } else {
                logger::info("  Cleaned up temporary extended video file");
            }
        }

        // Clean up original MP4 file after successful conversion
        self.cleanup_source_file(input_path, &converted_path).await?;

        Ok(converted_path)
    }

    fn setup_cleanup_handlers(&mut self) {
        let cleanup = || {
            logger::warning("Cleaning up download process...");
            std::process::exit(0);
        };

        // Note: In a full implementation, we'd set up proper signal handlers
        // For now, we'll just note that this would be implemented
        let _ = cleanup;
    }

    fn parse_download_progress(&self, line: &str) -> Option<(f64, String, String, String)> {
        utils::parse_progress(line)
    }

    async fn download_video(&mut self, url: &str, video_format: &VideoFormat, audio_format: &Option<AudioFormat>, output_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        logger::header("Starting Download");
        logger::download(&format!("Output: {}", output_path.display()));
        
        // Ensure unique filename
        let final_output_path = utils::get_unique_filename(output_path)?;
        if final_output_path != *output_path {
            logger::warning(&format!("File exists, using: {}", final_output_path.display()));
        }
        
        // Build yt-dlp arguments
        let format_arg = if let Some(audio) = audio_format {
            format!("{}+{}", video_format.format_id, audio.format_id)
        } else {
            video_format.format_id.clone()
        };

        let mut args = vec![
            "-f", &format_arg,
            "-o", final_output_path.to_str().unwrap(),
            "--merge-output-format", self.get_extension(),
            "--progress",
            "--newline"
        ];
        
        // Add optional settings
        let config = Config::default();
        if config.download_settings.embed_subtitles {
            args.push("--embed-subs");
        }
        
        if config.download_settings.embed_thumbnail {
            args.push("--embed-thumbnail");
        }
        
        args.push(url);
        
        logger::info(&format!("Command: yt-dlp {}", args.join(" ")));
        
        // Start download process
        let child = Command::new("yt-dlp")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
            
        self.is_downloading = true;
        self.current_process = Some(child);
        
        // Handle stdout (progress)
        let stdout = self.current_process.as_mut().unwrap().stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    self.parse_download_progress(&line);
                }
            }
        }
        
        // Handle stderr (errors and additional info)
        let stderr = self.current_process.as_mut().unwrap().stderr.take().unwrap();
        let stderr_reader = BufReader::new(stderr);

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                if !line.trim().is_empty() && !line.contains("WARNING") {
                    logger::warning(&line);
                }
            }
        }
        
        // Handle process completion
        let status = self.current_process.as_mut().unwrap().wait()?;
        self.is_downloading = false;
        self.current_process = None;
        
        if status.success() {
            logger::success("Download completed successfully!");
            
            // Check if file exists and show stats
            if let Some(stats) = utils::get_file_stats(&final_output_path) {
                logger::file(&format!("Final file: {}", final_output_path.display()));
                logger::stats(&format!("File size: {}", utils::format_file_size(Some(stats.len()))));
                // Note: birthtime not available in Rust std::fs::Metadata
            }
            
            Ok(final_output_path)
        } else {
            Err(format!("Download failed with exit code {:?}", status.code()).into())
        }
    }

    async fn download_with_retry(&mut self, url: &str, video_format: &VideoFormat, audio_format: &Option<AudioFormat>, output_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config = Config::default();
        let mut _last_error = None;

        for attempt in 1..=config.download_settings.retry_attempts {
            if attempt > 1 {
                logger::warning(&format!("Retry attempt {}/{}", attempt, config.download_settings.retry_attempts));
                // Wait a bit before retrying
                tokio::time::sleep(Duration::from_secs(2)).await;
            }

            match self.download_video(url, video_format, audio_format, output_path).await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    let error_msg = error.to_string();
                    logger::error(&format!("Attempt {} failed: {}", attempt, error_msg));

                    if attempt == config.download_settings.retry_attempts {
                        return Err(format!("Download failed after {} attempts. Last error: {}",
                            config.download_settings.retry_attempts,
                            error_msg).into());
                    }
                    _last_error = Some(error);
                }
            }
        }

        unreachable!()
    }

    pub fn is_download_in_progress(&self) -> bool {
        self.is_downloading
    }

    pub fn cancel_download(&mut self) -> bool {
        if let Some(mut process) = self.current_process.take() {
            logger::warning("Cancelling download...");
            let _ = process.kill();
            self.is_downloading = false;
            return true;
        }
        false
    }

    pub async fn perform_download(&mut self, url: &str, analysis: &SelectedFormats) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Setup cleanup handlers
        self.setup_cleanup_handlers();

        // Check video quality and warn if needed
        self.check_video_quality(&analysis.video_format);

        // Create output filename
        let output_filename = self.create_output_filename(&analysis.info, &analysis.video_format);
        let output_path = utils::get_output_path(&output_filename);

        // Check if video already exists
        let (exists, existing_path, needs_conversion) = self.check_existing_video(&output_path);
        let final_path;

        if exists && !needs_conversion {
            // .mov file already exists, we're done
            final_path = existing_path.unwrap();
            logger::info(" Using existing .mov video, no processing needed");
            return Ok(final_path);
        } else if exists && needs_conversion {
            // Source file exists but needs conversion
            final_path = existing_path.unwrap();
            logger::info(" Skipping download, using existing video for conversion");
        } else {
            // Need to download
            final_path = self.download_with_retry(
                url,
                &analysis.video_format,
                &analysis.audio_format,
                &output_path
            ).await?;
            logger::success(&format!("Video downloaded successfully: {}", final_path.file_name().unwrap().to_string_lossy()));
        }

        // Convert to .mov format for wallpaper compatibility
        let config = Config::default();
        if config.download_settings.convert_to_mov {
            let mov_path = self.convert_to_mov(&final_path).await?;
            return Ok(mov_path);
        }

        Ok(final_path)
    }
}
