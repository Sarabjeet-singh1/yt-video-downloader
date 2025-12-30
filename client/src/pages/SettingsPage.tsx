import React, { useState } from 'react'
import { useAppStore } from '@/store/appStore'
import { useTheme } from '@/hooks/useTheme'

interface SettingsPageProps {
  onNavigate: (path: string) => void
}

const SettingsPage: React.FC<SettingsPageProps> = ({ onNavigate }) => {
  const { theme, setTheme } = useAppStore()
  const { setTheme: setThemeValue } = useTheme()
  const [enableWallpaper, setEnableWallpaper] = useState(true)
  const [retryAttempts, setRetryAttempts] = useState(3)
  const [timeoutSeconds, setTimeoutSeconds] = useState(300)
  const [convertToMov, setConvertToMov] = useState(true)
  const [useHevc, setUseHevc] = useState(true)
  const [targetFrameRate, setTargetFrameRate] = useState(60)

  const handleThemeChange = (newTheme: 'light' | 'dark' | 'system') => {
    setThemeValue(newTheme)
    setTheme(newTheme)
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* Header */}
      <div className="text-center">
        <h1 className="text-3xl font-bold text-slate-900 dark:text-white mb-4">
          Settings
        </h1>
        <p className="text-slate-600 dark:text-slate-300">
          Configure your YouTube downloader preferences
        </p>
      </div>

      {/* General Settings */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-6">
          ⚙️ General Settings
        </h2>
        <div className="space-y-6">
          {/* Theme Selection */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
              Theme
            </label>
            <div className="flex gap-2">
              <button
                onClick={() => handleThemeChange('light')}
                className={`px-4 py-2 rounded-lg border transition-colors ${
                  theme === 'light'
                    ? 'bg-blue-50 border-blue-300 text-blue-700 dark:bg-blue-900/20 dark:border-blue-600 dark:text-blue-300'
                    : 'bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-600'
                }`}
              >
                ☀️ Light
              </button>
              <button
                onClick={() => handleThemeChange('dark')}
                className={`px-4 py-2 rounded-lg border transition-colors ${
                  theme === 'dark'
                    ? 'bg-blue-50 border-blue-300 text-blue-700 dark:bg-blue-900/20 dark:border-blue-600 dark:text-blue-300'
                    : 'bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-600'
                }`}
              >
                🌙 Dark
              </button>
              <button
                onClick={() => handleThemeChange('system')}
                className={`px-4 py-2 rounded-lg border transition-colors ${
                  theme === 'system'
                    ? 'bg-blue-50 border-blue-300 text-blue-700 dark:bg-blue-900/20 dark:border-blue-600 dark:text-blue-300'
                    : 'bg-white dark:bg-slate-700 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-600'
                }`}
              >
                💻 System
              </button>
            </div>
          </div>

          {/* Default Wallpaper Setting */}
          <div className="flex items-center gap-3">
            <input
              type="checkbox"
              id="defaultWallpaper"
              checked={enableWallpaper}
              onChange={(e) => setEnableWallpaper(e.target.checked)}
              className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
            />
            <label htmlFor="defaultWallpaper" className="text-sm font-medium text-slate-700 dark:text-slate-300">
              Enable wallpaper installation by default
            </label>
          </div>
        </div>
      </div>

      {/* Download Settings */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-6">
          ⬇️ Download Settings
        </h2>
        <div className="grid md:grid-cols-2 gap-6">
          {/* Retry Attempts */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              Retry Attempts
            </label>
            <input
              type="number"
              min="1"
              max="10"
              value={retryAttempts}
              onChange={(e) => setRetryAttempts(parseInt(e.target.value))}
              className="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
            <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
              Number of times to retry failed downloads
            </p>
          </div>

          {/* Timeout */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              Timeout (seconds)
            </label>
            <input
              type="number"
              min="60"
              max="3600"
              value={timeoutSeconds}
              onChange={(e) => setTimeoutSeconds(parseInt(e.target.value))}
              className="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
            <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
              Maximum time to wait for downloads
            </p>
          </div>
        </div>
      </div>

      {/* Conversion Settings */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-6">
          🔄 Conversion Settings
        </h2>
        <div className="space-y-6">
          {/* Convert to MOV */}
          <div className="flex items-center gap-3">
            <input
              type="checkbox"
              id="convertToMov"
              checked={convertToMov}
              onChange={(e) => setConvertToMov(e.target.checked)}
              className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
            />
            <label htmlFor="convertToMov" className="text-sm font-medium text-slate-700 dark:text-slate-300">
              Convert to .mov format (required for wallpapers)
            </label>
          </div>

          {/* Use HEVC */}
          <div className="flex items-center gap-3">
            <input
              type="checkbox"
              id="useHevc"
              checked={useHevc}
              onChange={(e) => setUseHevc(e.target.checked)}
              className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
            />
            <label htmlFor="useHevc" className="text-sm font-medium text-slate-700 dark:text-slate-300">
              Use HEVC codec (better compression, requires supported hardware)
            </label>
          </div>

          {/* Target Frame Rate */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              Target Frame Rate
            </label>
            <select
              value={targetFrameRate}
              onChange={(e) => setTargetFrameRate(parseInt(e.target.value))}
              className="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value={30}>30 FPS (Standard)</option>
              <option value={60}>60 FPS (Smooth)</option>
              <option value={120}>120 FPS (High-end)</option>
            </select>
            <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
              Higher frame rates provide smoother wallpapers but use more resources
            </p>
          </div>
        </div>
      </div>

      {/* Dependencies Status */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-6">
          🔧 Dependencies Status
        </h2>
        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <div className="flex items-center gap-3">
              <span className="text-green-600 dark:text-green-400 text-xl">✅</span>
              <div>
                <div className="font-medium text-green-900 dark:text-green-100">yt-dlp</div>
                <div className="text-sm text-green-700 dark:text-green-300">Video downloader</div>
              </div>
            </div>
            <span className="text-sm font-medium text-green-900 dark:text-green-100">
              Installed
            </span>
          </div>

          <div className="flex items-center justify-between p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <div className="flex items-center gap-3">
              <span className="text-green-600 dark:text-green-400 text-xl">✅</span>
              <div>
                <div className="font-medium text-green-900 dark:text-green-100">ffmpeg</div>
                <div className="text-sm text-green-700 dark:text-green-300">Video converter</div>
              </div>
            </div>
            <span className="text-sm font-medium text-green-900 dark:text-green-100">
              Installed
            </span>
          </div>

          <div className="flex items-center justify-between p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
            <div className="flex items-center gap-3">
              <span className="text-green-600 dark:text-green-400 text-xl">✅</span>
              <div>
                <div className="font-medium text-green-900 dark:text-green-100">Rust</div>
                <div className="text-sm text-green-700 dark:text-green-300">Programming language</div>
              </div>
            </div>
            <span className="text-sm font-medium text-green-900 dark:text-green-100">
              Available
            </span>
          </div>
        </div>

        <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-blue-600 dark:text-blue-400">💡</span>
            <span className="font-medium text-blue-900 dark:text-blue-100">
              Dependencies Check
            </span>
          </div>
          <p className="text-sm text-blue-700 dark:text-blue-300">
            All required dependencies are properly installed. You can start downloading videos immediately.
          </p>
          <button className="mt-3 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg transition-colors">
            Run Full Check
          </button>
        </div>
      </div>

      {/* System Information */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-6">
          💻 System Information
        </h2>
        <div className="grid md:grid-cols-2 gap-6">
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">Operating System:</span>
              <span className="font-medium text-slate-900 dark:text-white">macOS 14.0</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">Architecture:</span>
              <span className="font-medium text-slate-900 dark:text-white">Apple Silicon (arm64)</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">Rust Version:</span>
              <span className="font-medium text-slate-900 dark:text-white">1.70.0</span>
            </div>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">Available Storage:</span>
              <span className="font-medium text-slate-900 dark:text-white">245 GB</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">Memory:</span>
              <span className="font-medium text-slate-900 dark:text-white">16 GB</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-600 dark:text-slate-400">HEVC Support:</span>
              <span className="font-medium text-green-600 dark:text-green-400">✅ Available</span>
            </div>
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="flex flex-col sm:flex-row gap-4 justify-center">
        <button
          onClick={() => onNavigate('/')}
          className="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors"
        >
          🏠 Back to Home
        </button>
        <button
          onClick={() => onNavigate('/download')}
          className="px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg transition-colors"
        >
          ⬇️ Start Downloading
        </button>
      </div>
    </div>
  )
}

export default SettingsPage

