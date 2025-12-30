import React from 'react'

const LoadingScreen: React.FC = () => {
  return (
    <div className="fixed inset-0 bg-gradient-to-br from-slate-900 via-slate-800 to-slate-700 flex items-center justify-center z-50">
      <div className="text-center">
        <div className="w-12 h-12 border-3 border-blue-500 border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
        <h2 className="text-xl font-semibold text-white mb-2">
          YouTube Downloader
        </h2>
        <p className="text-slate-400">
          Loading application...
        </p>
      </div>
    </div>
  )
}

export default LoadingScreen

