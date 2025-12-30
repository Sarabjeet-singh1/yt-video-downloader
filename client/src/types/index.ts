export interface VideoInfo {
  id: string;
  title: string;
  description: string;
  duration: number;
  thumbnail: string;
  channel: string;
  views: number;
  uploadDate: string;
  availableFormats: VideoFormat[];
  recommendedFormat?: VideoFormat;
}

export interface VideoFormat {
  formatId: string;
  ext: string;
  resolution: string;
  fps: number;
  vcodec: string;
  acodec: string;
  filesize?: number;
  quality: number;
  url?: string;
}

export interface DownloadProgress {
  percentage: number;
  speed: string;
  eta: string;
  downloaded: string;
  total: string;
  status: 'downloading' | 'converting' | 'installing' | 'completed' | 'error';
  stage: string;
}

export interface DownloadRequest {
  url: string;
  outputDirectory?: string;
  format?: string;
  enableWallpaper?: boolean;
  quality?: string;
}

export interface DownloadResponse {
  success: boolean;
  downloadPath?: string;
  wallpaperInstalled?: boolean;
  error?: string;
  metadata?: VideoMetadata;
}

export interface VideoMetadata {
  title: string;
  duration: number;
  size: string;
  format: string;
  resolution: string;
  fps: number;
  quality: string;
}

export interface DependencyStatus {
  name: string;
  installed: boolean;
  version?: string;
  required: boolean;
  installHint?: string;
  error?: string;
}

export interface SystemInfo {
  os: string;
  arch: string;
  rustVersion: string;
  dependencies: DependencyStatus[];
}

export interface AppConfig {
  enableWallpaper: boolean;
  outputDir: string;
  videoPreferences: VideoPreferences;
  audioPreferences: AudioPreferences;
  downloadSettings: DownloadSettings;
  wallpaperSettings: WallpaperSettings;
}

export interface VideoPreferences {
  preferredFormats: string[];
  preferredCodecs: string[];
  maxResolution: number;
  preferHighFps: boolean;
  prefer60fps: boolean;
}

export interface AudioPreferences {
  preferredFormats: string[];
  preferredCodecs: string[];
  minBitrate: number;
  preferredBitrate: number;
}

export interface DownloadSettings {
  retryAttempts: number;
  timeoutSeconds: number;
  mergeOutputFormat: string;
  embedSubtitles: boolean;
  embedThumbnail: boolean;
  convertToMov: boolean;
  optimizeForWallpaper: boolean;
  useHevc: boolean;
  targetFrameRate: number;
  targetResolution: string;
}

export interface WallpaperSettings {
  customerDir: string;
  targetSubDir: string;
  backupDir: string;
  requiredFormat: string;
  minRecommendedResolution: number;
  minRecommendedDuration: number;
  maxRetryAttempts: number;
  retryInterval: number;
}

export interface ApiError {
  message: string;
  code: string;
  details?: Record<string, any>;
}

export interface LogEntry {
  timestamp: string;
  level: 'info' | 'success' | 'warning' | 'error';
  message: string;
  source?: string;
}

export interface UIState {
  isLoading: boolean;
  isDownloading: boolean;
  downloadProgress?: DownloadProgress;
  logs: LogEntry[];
  currentStep: string;
  showAdvanced: boolean;
  sidebarOpen: boolean;
}

export type Theme = 'light' | 'dark' | 'system';
export type ViewMode = 'grid' | 'list';
export type SortBy = 'title' | 'duration' | 'size' | 'date' | 'quality';
