# FFmpeg HEVC Conversion Fix Plan

## Tasks
- [ ] Add comprehensive error logging to capture FFmpeg stderr output for better diagnostics (IN PROGRESS)
- [ ] Implement more conservative fallback settings: reduce resolution/bitrate if initial attempts fail
- [ ] Add FFmpeg version and codec availability checks before conversion
- [ ] Add input video validation to ensure compatibility before processing
- [ ] Modify the conversion logic to try simpler settings first before aggressive ones
- [ ] Update src/config.rs with configurable fallback settings
- [ ] Update src/utils.rs with utility functions for FFmpeg validation
- [ ] Update src/downloader.rs with improved conversion logic and error handling

## Followup Steps
- [ ] Test conversion with various input videos
- [ ] Verify FFmpeg installation and codec support
- [ ] Run the application to ensure fixes work
