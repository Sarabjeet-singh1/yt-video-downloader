import React from 'react'

interface SidebarItem {
  id: string
  label: string
  icon: string
  path: string
  description: string
}

const sidebarItems: SidebarItem[] = [
  {
    id: 'home',
    label: 'Home',
    icon: '🏠',
    path: '/',
    description: 'Overview and quick start'
  },
  {
    id: 'download',
    label: 'Download',
    icon: '⬇️',
    path: '/download',
    description: 'Download and convert videos'
  },
  {
    id: 'settings',
    label: 'Settings',
    icon: '⚙️',
    path: '/settings',
    description: 'Configure preferences'
  }
]

interface SidebarProps {
  isOpen: boolean
  onClose: () => void
  currentPath: string
  onNavigate: (path: string) => void
}

const Sidebar: React.FC<SidebarProps> = ({ onClose, currentPath, onNavigate }) => {
  return (
    <div className="flex flex-col h-full bg-white/80 dark:bg-slate-800/80 backdrop-blur-xl">
      {/* Mobile header */}
      <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700 lg:hidden">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg flex items-center justify-center">
            <span className="text-white text-sm font-bold">YT</span>
          </div>
          <span className="font-semibold text-slate-900 dark:text-white">
            Downloader
          </span>
        </div>
        <button
          onClick={onClose}
          className="p-2 rounded-lg bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors"
        >
          ✕
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4 space-y-2">
        {sidebarItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onNavigate(item.path)}
            className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg text-left transition-all duration-200 ${
              currentPath === item.path
                ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800'
                : 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
            }`}
          >
            <span className="text-lg">{item.icon}</span>
            <div className="flex-1">
              <div className={`font-medium ${currentPath === item.path ? 'text-blue-900 dark:text-blue-100' : 'text-slate-900 dark:text-white'}`}>
                {item.label}
              </div>
              <div className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                {item.description}
              </div>
            </div>
          </button>
        ))}
      </nav>

      {/* Features highlight */}
      <div className="p-4 border-t border-slate-200 dark:border-slate-700">
        <div className="bg-gradient-to-r from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-lg p-3">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-sm">⚡</span>
            <span className="text-sm font-medium text-blue-900 dark:text-blue-100">
              Pro Features
            </span>
          </div>
          <ul className="text-xs text-blue-700 dark:text-blue-300 space-y-1">
            <li className="flex items-center gap-2">
              <span>🛡️</span>
              Live wallpapers for macOS
            </li>
            <li className="flex items-center gap-2">
              <span>📁</span>
              4K 60fps conversion
            </li>
          </ul>
        </div>
      </div>
    </div>
  )
}

export default Sidebar

