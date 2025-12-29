# Rust Downloader (yt-dlp wrapper)

This is a small Rust CLI that wraps `yt-dlp` to analyze formats and download video+audio (merging when separate streams exist).

Build:

```bash
cd rust-downloader
cargo build --release
```

Run:

```bash
cargo run --release -- "https://youtu.be/VIDEO_ID"
```

Requirements: `yt-dlp` must be installed and available on PATH. `ffmpeg` is required for merging/conversion if needed by `yt-dlp`.
