import React, { useState } from 'react'
import { useAppStore } from '@/store/appStore'

const DownloadPage: React.FC = () => {
  const [url, setUrl] = useState('')
  const [enableWallpaper, setEnableWallpaper] = useState(true)
  const [outputDirectory, setOutputDirectory] = useState('')
  const [quality, setQuality] = useState('best')
  
  const { isDownloading, downloadProgress, currentStep, addLog } = useAppStore()

  const handleDownload = async () => {
    if (!url.trim()) {
      addLog({
        level: 'error',
        message: 'Please enter a YouTube URL',
        source: 'download'
      })
      return
    }

    // Simulate download process
    addLog({
      level: 'info',
      message: `Starting download for: ${url}`,
      source: 'download'
    })

    // Simulate progress updates
    let progress = 0
    const interval = setInterval(() => {
      progress += 10
      if (progress <= 100) {
        // Update progress here would go through the store
        if (progress === 100) {
          clearInterval(interval)
          addLog({
            level: 'success',
            message: 'Download completed successfully!',
            source: 'download'
          })
        }
      }
    }, 1000)
  }

  const isValidUrl = (url: string) => {
    return url.includes('youtube.com') || url.includes('youtu.be')
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* Header */}
      <div className="text-center">
        <h1 className="text-3xl font-bold text-slate-900 dark:text-white mb-4">
          Download YouTube Videos
        </h1>
        <p className="text-slate-600 dark:text-slate-300">
          Transform any YouTube video into a stunning live wallpaper
        </p>
      </div>

      {/* Download Form */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
        <div className="space-y-6">
          {/* URL Input */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              YouTube URL
            </label>
            <div className="flex gap-3">
              <input
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://www.youtube.com/watch?v=..."
                className="flex-1 px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white placeholder-slate-500 dark:placeholder-slate-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                disabled={isDownloading}
              />
              <button
                onClick={handleDownload}
                disabled={isDownloading || !isValidUrl(url)}
                className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-400 text-white font-semibold rounded-lg transition-colors disabled:cursor-not-allowed"
              >
                {isDownloading ? '⏳ Downloading...' : '⬇️ Download'}
              </button>
            </div>
            {!isValidUrl(url) && url && (
              <p className="text-sm text-red-600 dark:text-red-400 mt-1">
                Please enter a valid YouTube URL
              </p>
            )}
          </div>

          {/* Options */}
          <div className="grid md:grid-cols-2 gap-6">
            {/* Quality Selection */}
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Quality Preference
              </label>
              <select
                value={quality}
                onChange={(e) => setQuality(e.target.value)}
                className="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                disabled={isDownloading}
              >
                <option value="best">Best Available (Recommended)</option>
                <option value="4k">4K (3840x2160)</option>
                <option value="1080p">1080p (1920x1080)</option>
                <option value="720p">720p (1280x720)</option>
                <option value="480p">480p (854x480)</option>
              </select>
            </div>

            {/* Output Directory */}
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Output Directory (Optional)
              </label>
              <input
                type="text"
                value={outputDirectory}
                onChange={(e) => setOutputDirectory(e.target.value)}
                placeholder="./outputs"
                className="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white placeholder-slate-500 dark:placeholder-slate-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                disabled={isDownloading}
              />
            </div>
          </div>

          {/* Wallpaper Options */}
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <div className="flex items-center gap-3">
              <input
                type="checkbox"
                id="wallpaper"
                checked={enableWallpaper}
                onChange={(e) => setEnableWallpaper(e.target.checked)}
                className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                disabled={isDownloading}
              />
              <label htmlFor="wallpaper" className="text-sm font-medium text-blue-900 dark:text-blue-100">
                🖼️ Install as Live Wallpaper (macOS only)
              </label>
            </div>
            <p className="text-xs text-blue-700 dark:text-blue-300 mt-1 ml-7">
              Automatically converts and installs the video as a live wallpaper. Requires administrator privileges.
            </p>
          </div>
        </div>
      </div>

      {/* Progress Section */}
      {isDownloading && (
        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold text-slate-900 dark:text-white">
                Download Progress
              </h3>
              <span className="text-sm text-slate-500 dark:text-slate-400">
                {currentStep}
              </span>
            </div>
            
            {/* Progress Bar */}
            <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-3">
              <div 
                className="bg-blue-600 h-3 rounded-full transition-all duration-300"
                style={{ width: `${downloadProgress?.percentage || 0}%` }}
              ></div>
            </div>
            
            {/* Progress Details */}
            {downloadProgress && (
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <span className="text-slate-500 dark:text-slate-400">Progress:</span>
                  <div className="font-semibold text-slate-900 dark:text-white">
                    {downloadProgress.percentage}%
                  </div>
                </div>
                <div>
                  <span className="text-slate-500 dark:text-slate-400">Speed:</span>
                  <div className="font-semibold text-slate-900 dark:text-white">
                    {downloadProgress.speed}
                  </div>
                </div>
                <div>
                  <span className="text-slate-500 dark:text-slate-400">ETA:</span>
                  <div className="font-semibold text-slate-900 dark:text-white">
                    {downloadProgress.eta}
                  </div>
                </div>
                <div>
                  <span className="text-slate-500 dark:text-slate-400">Downloaded:</span>
                  <div className="font-semibold text-slate-900 dark:text-white">
                    {downloadProgress.downloaded}
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Tips */}
      <div className="bg-amber-50 dark:bg-amber-900/20 rounded-xl p-6">
        <h3 className="text-lg font-semibold text-amber-900 dark:text-amber-100 mb-3">
          💡 Pro Tips
        </h3>
        <ul className="space-y-2 text-sm text-amber-800 dark:text-amber-200">
          <li>• Use high-quality videos (1080p or higher) for best wallpaper results</li>
          <li>• Keep your Mac plugged in for uninterrupted wallpaper animation</li>
          <li>• Videos are automatically converted to optimized .mov format</li>
          <li>• If wallpaper becomes static, use the refresh utility to fix it</li>
          <li>• Ensure sufficient free storage space (videos can be several GB)</li>
        </ul>
      </div>
    </div>
  )
}

export default DownloadPage

