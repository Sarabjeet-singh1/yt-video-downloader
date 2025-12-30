import React from 'react'

interface HomePageProps {
  onNavigate: (path: string) => void
}

const HomePage: React.FC<HomePageProps> = ({ onNavigate }) => {
  return (
    <div className="space-y-6">
      {/* Hero Section */}
      <div className="text-center py-12 px-4">
        <div className="max-w-3xl mx-auto">
          <h1 className="text-4xl md:text-5xl font-bold text-slate-900 dark:text-white mb-6">
            Transform YouTube Videos into
            <span className="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              {' '}Live Wallpapers
            </span>
          </h1>
          <p className="text-xl text-slate-600 dark:text-slate-300 mb-8 leading-relaxed">
            Download and convert YouTube videos into stunning 4K 60fps live wallpapers for macOS.
            Built with Rust for maximum performance and reliability.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <button
              onClick={() => onNavigate('/download')}
              className="px-8 py-4 bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white font-semibold rounded-lg transition-all duration-200 transform hover:scale-105 shadow-lg"
            >
              🚀 Start Downloading
            </button>
            <button
              onClick={() => onNavigate('/settings')}
              className="px-8 py-4 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 text-slate-900 dark:text-white font-semibold rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition-all duration-200"
            >
              ⚙️ Configure Settings
            </button>
          </div>
        </div>
      </div>

      {/* Features Grid */}
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 px-4">
        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">🎥</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            Smart Video Analysis
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Automatically selects the best available video format and quality for optimal wallpaper performance.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">⚡</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            High-Performance Conversion
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Hardware-accelerated HEVC conversion to 4K 60fps .mov format for smooth macOS wallpaper animation.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">🖼️</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            Automatic Wallpaper Installation
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Seamlessly installs converted videos as live wallpapers with automatic backup and recovery.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">🛡️</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            Robust Error Handling
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Comprehensive error detection and user guidance with automatic retry mechanisms.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">📊</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            Real-time Progress Tracking
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Detailed download and conversion progress with speed, ETA, and stage indicators.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-xl p-6 shadow-sm border border-slate-200 dark:border-slate-700">
          <div className="text-3xl mb-4">🧹</div>
          <h3 className="text-lg font-semibold text-slate-900 dark:text-white mb-2">
            Automatic Cleanup
          </h3>
          <p className="text-slate-600 dark:text-slate-300 text-sm">
            Automatically removes temporary files and optimizes storage usage after conversion.
          </p>
        </div>
      </div>

      {/* Quick Start Guide */}
      <div className="bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-xl p-8 mx-4">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-6 text-center">
          Quick Start Guide
        </h2>
        <div className="grid md:grid-cols-4 gap-6">
          <div className="text-center">
            <div className="w-12 h-12 bg-blue-600 text-white rounded-full flex items-center justify-center mx-auto mb-3 text-lg font-bold">
              1
            </div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
              Install Dependencies
            </h3>
            <p className="text-sm text-slate-600 dark:text-slate-300">
              Ensure yt-dlp and ffmpeg are installed on your system
            </p>
          </div>
          <div className="text-center">
            <div className="w-12 h-12 bg-blue-600 text-white rounded-full flex items-center justify-center mx-auto mb-3 text-lg font-bold">
              2
            </div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
              Paste YouTube URL
            </h3>
            <p className="text-sm text-slate-600 dark:text-slate-300">
              Copy and paste any YouTube video URL into the downloader
            </p>
          </div>
          <div className="text-center">
            <div className="w-12 h-12 bg-blue-600 text-white rounded-full flex items-center justify-center mx-auto mb-3 text-lg font-bold">
              3
            </div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
              Choose Options
            </h3>
            <p className="text-sm text-slate-600 dark:text-slate-300">
              Select wallpaper installation and quality preferences
            </p>
          </div>
          <div className="text-center">
            <div className="w-12 h-12 bg-blue-600 text-white rounded-full flex items-center justify-center mx-auto mb-3 text-lg font-bold">
              4
            </div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
              Enjoy!
            </h3>
            <p className="text-sm text-slate-600 dark:text-slate-300">
              Watch your video transform into a stunning live wallpaper
            </p>
          </div>
        </div>
      </div>

      {/* System Requirements */}
      <div className="bg-white dark:bg-slate-800 rounded-xl p-6 mx-4 shadow-sm border border-slate-200 dark:border-slate-700">
        <h2 className="text-xl font-bold text-slate-900 dark:text-white mb-4">
          System Requirements
        </h2>
        <div className="grid md:grid-cols-2 gap-6">
          <div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">📱 Operating System</h3>
            <ul className="text-sm text-slate-600 dark:text-slate-300 space-y-1">
              <li>• macOS 10.15 (Catalina) or later</li>
              <li>• macOS Big Sur or later recommended for best performance</li>
            </ul>
          </div>
          <div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">🛠️ Dependencies</h3>
            <ul className="text-sm text-slate-600 dark:text-slate-300 space-y-1">
              <li>• Rust 1.70+ (for building)</li>
              <li>• yt-dlp (video downloading)</li>
              <li>• ffmpeg (video conversion)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">💾 Hardware</h3>
            <ul className="text-sm text-slate-600 dark:text-slate-300 space-y-1">
              <li>• 4GB RAM minimum, 8GB recommended</li>
              <li>• 10GB free storage for temporary files</li>
              <li>• SSD storage recommended for faster I/O</li>
            </ul>
          </div>
          <div>
            <h3 className="font-semibold text-slate-900 dark:text-white mb-2">🌐 Network</h3>
            <ul className="text-sm text-slate-600 dark:text-slate-300 space-y-1">
              <li>• Stable internet connection</li>
              <li>• No proxy or VPN required</li>
              <li>• Supports all YouTube video qualities</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}

export default HomePage

