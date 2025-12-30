import React from 'react'

const Header: React.FC = () => {
  return (
    <header className="bg-white/80 dark:bg-slate-800/80 backdrop-blur-xl border-b border-slate-200 dark:border-slate-700 px-4 py-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg flex items-center justify-center">
              <span className="text-white text-sm font-bold">YT</span>
            </div>
            <div>
              <h1 className="text-lg font-semibold text-slate-900 dark:text-white">
                YouTube Downloader
              </h1>
              <p className="text-xs text-slate-500 dark:text-slate-400">
                Transform videos into live wallpapers
              </p>
            </div>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <button className="px-3 py-1 rounded-md text-sm bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">
            Light
          </button>
          <button className="px-3 py-1 rounded-md text-sm hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors">
            Dark
          </button>
          <button className="px-3 py-1 rounded-md text-sm hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors">
            System
          </button>
        </div>
      </div>
    </header>
  )
}

export default Header

