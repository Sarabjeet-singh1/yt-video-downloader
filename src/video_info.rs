use serde_json::Value;
use std::process::Command;
use crate::logger;
use crate::config::Config;
use crate::utils;

#[derive(Debug, Clone)]
pub struct VideoFormat {
    pub format_id: String,
    pub ext: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub fps: Option<f64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub filesize: Option<u64>,
    pub tbr: Option<f64>,
    pub abr: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub format_id: String,
    pub ext: String,
    pub acodec: Option<String>,
    pub filesize: Option<u64>,
    pub abr: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub title: String,
    pub uploader: Option<String>,
    pub duration: Option<u64>,
    pub view_count: Option<u64>,
    pub upload_date: Option<String>,
    pub description: Option<String>,
    pub formats: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct SelectedFormats {
    pub info: VideoInfo,
    pub video_format: VideoFormat,
    pub audio_format: Option<AudioFormat>,
}

fn run_yt_dlp_dump(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("yt-dlp")
        .args(["--dump-json", "--no-warnings", url])
        .output()?;

    if !output.status.success() {
        return Err(format!("yt-dlp failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn display_video_info(info: &VideoInfo) {
    logger::header("Video Information");
    
    logger::video(&format!("Title: {}", info.title));
    if let Some(uploader) = &info.uploader {
        logger::video(&format!("Uploader: {}", uploader));
    }
    logger::video(&format!("Duration: {}", utils::format_duration(info.duration)));
    if let Some(views) = info.view_count {
        logger::video(&format!("Views: {}", utils::format_number(Some(views))));
    }
    if let Some(date) = &info.upload_date {
        logger::video(&format!("Upload Date: {}", utils::format_date(date)));
    }
    
    if let Some(desc) = &info.description {
        let short_desc = if desc.len() > 100 {
            format!("{}...", &desc[..100])
        } else {
            desc.clone()
        };
        logger::video(&format!("Description: {}", short_desc));
    }
}

fn analyze_formats(formats: &[Value]) -> (Vec<VideoFormat>, Vec<AudioFormat>, Vec<VideoFormat>) {
    let config = Config::default();
    
    // Filter video formats
    let video_formats: Vec<VideoFormat> = formats
        .iter()
        .filter_map(|f| {
            let vcodec = f.get("vcodec")?.as_str()?;
            if vcodec == "none" {
                return None;
            }
            
            let height = f.get("height").and_then(|h| h.as_u64()).map(|h| h as u32);
            let ext = f.get("ext")?.as_str()?;
            
            if !config.video_preferences.preferred_formats.contains(&ext) {
                return None;
            }
            
            Some(VideoFormat {
                format_id: f.get("format_id")?.as_str()?.to_string(),
                ext: ext.to_string(),
                height,
                width: f.get("width").and_then(|w| w.as_u64()).map(|w| w as u32),
                fps: f.get("fps").and_then(|fps| fps.as_f64()),
                vcodec: Some(vcodec.to_string()),
                acodec: f.get("acodec").and_then(|ac| ac.as_str()).map(|s| s.to_string()),
                filesize: f.get("filesize").and_then(|fs| fs.as_u64()),
                tbr: f.get("tbr").and_then(|tbr| tbr.as_f64()),
                abr: f.get("abr").and_then(|abr| abr.as_f64()),
            })
        })
        .collect();

    // Filter audio formats
    let audio_formats: Vec<AudioFormat> = formats
        .iter()
        .filter_map(|f| {
            let acodec = f.get("acodec")?.as_str()?;
            if acodec == "none" {
                return None;
            }
            
            // Only pure audio formats (no video)
            if let Some(vcodec) = f.get("vcodec") {
                if vcodec.as_str()? != "none" {
                    return None;
                }
            }
            
            let ext = f.get("ext")?.as_str()?;
            if !config.audio_preferences.preferred_formats.contains(&ext) {
                return None;
            }
            
            Some(AudioFormat {
                format_id: f.get("format_id")?.as_str()?.to_string(),
                ext: ext.to_string(),
                acodec: Some(acodec.to_string()),
                filesize: f.get("filesize").and_then(|fs| fs.as_u64()),
                abr: f.get("abr").and_then(|abr| abr.as_u64()),
            })
        })
        .collect();

    // Combined formats (video + audio)
    let combined_formats: Vec<VideoFormat> = formats
        .iter()
        .filter_map(|f| {
            let vcodec = f.get("vcodec")?.as_str()?;
            let acodec = f.get("acodec")?.as_str()?;
            
            if vcodec == "none" || acodec == "none" {
                return None;
            }
            
            let height = f.get("height").and_then(|h| h.as_u64()).map(|h| h as u32);
            let ext = f.get("ext")?.as_str()?;
            
            if !config.video_preferences.preferred_formats.contains(&ext) {
                return None;
            }
            
            Some(VideoFormat {
                format_id: f.get("format_id")?.as_str()?.to_string(),
                ext: ext.to_string(),
                height,
                width: f.get("width").and_then(|w| w.as_u64()).map(|w| w as u32),
                fps: f.get("fps").and_then(|fps| fps.as_f64()),
                vcodec: Some(vcodec.to_string()),
                acodec: Some(acodec.to_string()),
                filesize: f.get("filesize").and_then(|fs| fs.as_u64()),
                tbr: f.get("tbr").and_then(|tbr| tbr.as_f64()),
                abr: f.get("abr").and_then(|abr| abr.as_f64()),
            })
        })
        .collect();

    logger::stats(&format!("Found {} video formats, {} audio formats, and {} combined formats", 
        video_formats.len(), audio_formats.len(), combined_formats.len()));

    (video_formats, audio_formats, combined_formats)
}

fn find_best_video_format(video_formats: &[VideoFormat]) -> Result<VideoFormat, Box<dyn std::error::Error>> {
    if video_formats.is_empty() {
        return Err("No suitable video formats found".into());
    }
    
    let config = Config::default();
    
    // Group by resolution
    let mut resolutions: Vec<u32> = video_formats
        .iter()
        .filter_map(|f| f.height)
        .collect();
    resolutions.sort_by(|a, b| b.cmp(a));
    
    let max_resolution = resolutions.get(0).cloned()
        .map(|res| std::cmp::min(res, config.video_preferences.max_resolution))
        .unwrap_or(0);
    
    logger::stats(&format!("Available resolutions: {}p", resolutions.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("p, ")));
    logger::stats(&format!("Selected resolution: {}p", max_resolution));
    
    // Filter by max resolution
    let mut candidate_formats: Vec<&VideoFormat> = video_formats
        .iter()
        .filter(|f| f.height == Some(max_resolution))
        .collect();
    
    if candidate_formats.is_empty() {
        // Fallback to highest available resolution
        candidate_formats = video_formats
            .iter()
            .filter(|f| f.height.is_some())
            .collect();
    }
    
    // Sort by preferences
    candidate_formats.sort_by(|a, b| {
        // Prefer specific formats
        let a_format_score = config.video_preferences.preferred_formats
            .iter().position(|&f| f == a.ext).unwrap_or(usize::MAX);
        let b_format_score = config.video_preferences.preferred_formats
            .iter().position(|&f| f == b.ext).unwrap_or(usize::MAX);
        
        if a_format_score != b_format_score {
            return a_format_score.cmp(&b_format_score);
        }
        
        // Prefer higher fps if enabled
        if config.video_preferences.prefer_high_fps {
            let a_fps = a.fps.unwrap_or(30.0);
            let b_fps = b.fps.unwrap_or(30.0);
            if (a_fps - b_fps).abs() > 0.1 {
                return b_fps.partial_cmp(&a_fps).unwrap_or(std::cmp::Ordering::Equal);
            }
        }
        
        // Prefer better codecs
        let a_codec_score = config.video_preferences.preferred_codecs
            .iter().position(|&f| a.vcodec.as_ref().map_or(false, |c| c.contains(&f))).unwrap_or(usize::MAX);
        let b_codec_score = config.video_preferences.preferred_codecs
            .iter().position(|&f| b.vcodec.as_ref().map_or(false, |c| c.contains(&f))).unwrap_or(usize::MAX);
        
        if a_codec_score != b_codec_score {
            return a_codec_score.cmp(&b_codec_score);
        }
        
        // Prefer larger file size (usually better quality)
        b.filesize.cmp(&a.filesize)
    });
    
    Ok(candidate_formats[0].clone())
}

fn find_best_audio_format(audio_formats: &[AudioFormat]) -> Result<AudioFormat, Box<dyn std::error::Error>> {
    if audio_formats.is_empty() {
        return Err("No suitable audio formats found".into());
    }
    
    let config = Config::default();
    
    let mut sorted_formats = audio_formats.to_vec();
    sorted_formats.sort_by(|a, b| {
        // Prefer specific formats
        let a_format_score = config.audio_preferences.preferred_formats
            .iter().position(|&f| f == a.ext).unwrap_or(usize::MAX);
        let b_format_score = config.audio_preferences.preferred_formats
            .iter().position(|&f| f == b.ext).unwrap_or(usize::MAX);
        
        if a_format_score != b_format_score {
            return a_format_score.cmp(&b_format_score);
        }
        
        // Prefer higher bitrate
        let a_bitrate = a.abr.unwrap_or(0);
        let b_bitrate = b.abr.unwrap_or(0);
        if a_bitrate != b_bitrate {
            return b_bitrate.cmp(&a_bitrate);
        }
        
        // Prefer better codecs
        let a_codec_score = config.audio_preferences.preferred_codecs
            .iter().position(|&f| a.acodec.as_ref().map_or(false, |c| c.contains(&f))).unwrap_or(usize::MAX);
        let b_codec_score = config.audio_preferences.preferred_codecs
            .iter().position(|&f| b.acodec.as_ref().map_or(false, |c| c.contains(&f))).unwrap_or(usize::MAX);
        
        if a_codec_score != b_codec_score {
            return a_codec_score.cmp(&b_codec_score);
        }
        
        std::cmp::Ordering::Equal
    });
    
    Ok(sorted_formats[0].clone())
}

pub fn display_selected_formats(video_format: &VideoFormat, audio_format: &Option<AudioFormat>) {
    logger::header("Selected Formats");
    
    let video_info = vec![
        format!("{}p", video_format.height.unwrap_or(0)),
        format!("{}fps", video_format.fps.unwrap_or(30.0) as u32),
        video_format.ext.clone(),
        format!("({})", video_format.vcodec.as_ref().unwrap_or(&"unknown".to_string())),
        utils::format_file_size(video_format.filesize)
    ].join(" ");
    logger::video(&format!("Video: {}", video_info));

    if let Some(audio) = audio_format {
        let audio_info = vec![
            format!("{}kbps", audio.abr.unwrap_or(0)),
            audio.ext.clone(),
            format!("({})", audio.acodec.as_ref().unwrap_or(&"unknown".to_string())),
            utils::format_file_size(audio.filesize)
        ].join(" ");
        logger::audio(&format!("Audio: {}", audio_info));
    } else {
        // No separate audio format; check if video has embedded audio
        if video_format.acodec.as_ref().map_or(false, |ac| ac != "none") {
            logger::audio("Audio: embedded in video format");
        } else {
            logger::audio("Audio: no separate audio stream found (video may be silent)");
        }
    }
}

pub fn analyze(url: &str) -> Result<SelectedFormats, Box<dyn std::error::Error>> {
    logger::search("Retrieving video information...");
    let dumped = run_yt_dlp_dump(url)?;
    let info_value: Value = serde_json::from_str(&dumped)?;

    logger::success("Video information retrieved successfully");

    // Parse video info
    let video_info = VideoInfo {
        title: info_value.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
        uploader: info_value.get("uploader").and_then(|v| v.as_str()).map(|s| s.to_string()),
        duration: info_value.get("duration").and_then(|v| v.as_u64()),
        view_count: info_value.get("view_count").and_then(|v| v.as_u64()),
        upload_date: info_value.get("upload_date").and_then(|v| v.as_str()).map(|s| s.to_string()),
        description: info_value.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
        formats: info_value.get("formats").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
    };

    // Display basic info
    display_video_info(&video_info);
    
    // Analyze formats
    let (video_formats, audio_formats, combined_formats) = analyze_formats(&video_info.formats);

    // Find best video format
    let best_video = find_best_video_format(&video_formats)?;
    let mut best_audio: Option<AudioFormat> = None;

    // Pick audio if available; otherwise try to use a combined format
    if !audio_formats.is_empty() {
        best_audio = Some(find_best_audio_format(&audio_formats)?);
    } else {
        // Try to find a combined format matching the chosen resolution
        let combined_candidate = combined_formats
            .iter()
            .find(|c| c.height == best_video.height)
            .cloned();
            
        if let Some(candidate) = combined_candidate {
            logger::info("No separate audio formats found; using combined AV format");
            return Ok(SelectedFormats { 
                info: video_info, 
                video_format: candidate, 
                audio_format: None 
            });
        } else {
            // Try one more fallback: pick any audio-only format (regardless of ext)
            let fallback_audio = video_info.formats
                .iter()
                .filter_map(|f| {
                    f.get("acodec").and_then(|ac| ac.as_str()).and_then(|acodec| {
                        if acodec != "none" {
                            f.get("vcodec").and_then(|vc| {
                                if vc.as_str() == Some("none") {
                                    Some(AudioFormat {
                                        format_id: f.get("format_id")?.as_str()?.to_string(),
                                        ext: f.get("ext")?.as_str()?.to_string(),
                                        acodec: Some(acodec.to_string()),
                                        filesize: f.get("filesize").and_then(|fs| fs.as_u64()),
                                        abr: f.get("abr").and_then(|abr| abr.as_u64()),
                                    })
                                } else {
                                    None
                                }
                            })
                        } else {
                            None
                        }
                    })
                })
                .max_by(|a, b| a.abr.cmp(&b.abr));
                
            if let Some(fallback) = fallback_audio {
                logger::info("Found an audio-only fallback; will download and merge audio with video");
                best_audio = Some(fallback);
            } else {
                // As a last resort, allow using the video format as-is
                if best_video.acodec.as_ref().map_or(false, |ac| ac != "none") {
                    logger::info("Using selected video format which includes embedded audio");
                } else {
                    logger::warning("No separate audio formats found; proceeding with video-only download (no audio)");
                }
            }
        }
    }

    // Display selected formats
    display_selected_formats(&best_video, &best_audio);

    logger::stats(&format!("Selected resolution: {}p", best_video.height.unwrap_or(0)));

    Ok(SelectedFormats { 
        info: video_info, 
        video_format: best_video, 
        audio_format: best_audio 
    })
}
