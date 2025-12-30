
import { useState, useRef, ChangeEvent, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { FiSettings, FiDownload, FiFolder, FiFolderPlus, FiCpu, FiSave, FiRefreshCw, FiCheck, FiAlertCircle, FiHome, FiTrash2 } from 'react-icons/fi'

interface SettingsProps {
  isOpen: boolean
  onClose: () => void
}

interface SettingsData {
  downloadPath: string
  maxConcurrentDownloads: number
  preferredQuality: string
  autoDownload: boolean
  notifications: boolean
  theme: string
}

export default function Settings({ isOpen, onClose, currentSettings, onSettingsChange }: SettingsProps & { currentSettings: SettingsData, onSettingsChange: (settings: SettingsData) => void }) {
  const [settings, setSettings] = useState<SettingsData>(currentSettings)
  const [isSaving, setIsSaving] = useState(false)
  const [pathSelected, setPathSelected] = useState(false)
  const [pathError, setPathError] = useState<string | null>(null)
  const [pathValid, setPathValid] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  // Validate path on mount and when it changes
  useEffect(() => {
    validatePath(settings.downloadPath)
  }, [settings.downloadPath])

  const validatePath = async (path: string) => {
    if (!path || path.trim().length === 0) {
      setPathError('Path cannot be empty')
      setPathValid(false)
      return false
    }

    // Check if path starts with ~ and expand it for validation
    const expandedPath = path.startsWith('~') 
      ? path.replace('~', process.env.HOME || process.env.USERPROFILE || '~')
      : path

    // Check for invalid characters
    const invalidChars = /[<>:"|?*\x00-\x1f]/
    if (invalidChars.test(expandedPath)) {
      setPathError('Path contains invalid characters')
      setPathValid(false)
      return false
    }

    // In browser, we can't fully validate the path, but we can check basic format
    if (expandedPath.startsWith('/') || expandedPath.includes('/') || expandedPath.includes('\\')) {
      setPathError(null)
      setPathValid(true)
      return true
    }

    // Relative path - check it's not just dots
    if (expandedPath === '.' || expandedPath === '..') {
      setPathError('Please enter a valid directory path')
      setPathValid(false)
      return false
    }

    setPathError(null)
    setPathValid(true)
    return true
  }

  const handleFolderSelect = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (files && files.length > 0) {
      const firstFile = files[0] as any
      
      // Use the actual file path if available (Node.js/Electron environment)
      let finalPath: string
      if (firstFile.path) {
        // Node.js/Electron path
        if (firstFile.path.includes('/')) {
          // macOS/Linux path
          finalPath = firstFile.path.split('/').slice(0, -1).join('/')
        } else if (firstFile.path.includes('\\')) {
          // Windows path
          finalPath = firstFile.path.split('\\').slice(0, -1).join('\\')
        } else {
          finalPath = firstFile.path
        }
      } else if (firstFile.webkitRelativePath) {
        // Browser path from webkitRelativePath
        finalPath = firstFile.webkitRelativePath.split('/')[0]
      } else {
        // Fallback - show alert that we need manual path entry
        alert('Browser folder picker limitation: Please enter the full path manually or use ~ for home directory')
        return
      }
      
      setSettings({ ...settings, downloadPath: finalPath })
      setPathSelected(true)
      
      // Reset the input so the same folder can be selected again
      e.target.value = ''
    }
  }

  const setToHome = () => {
    setSettings({ ...settings, downloadPath: '~/Downloads' })
    setPathSelected(true)
  }

  const clearPath = () => {
    setSettings({ ...settings, downloadPath: '' })
    setPathSelected(false)
    setPathError(null)
    setPathValid(false)
  }

  const handleSave = async () => {
    setIsSaving(true)
    // Simulate saving settings
    await new Promise(resolve => setTimeout(resolve, 500))
    localStorage.setItem('downloaderSettings', JSON.stringify(settings))
    onSettingsChange(settings)
    setIsSaving(false)
    setPathSelected(false)
    onClose()
  }

  const openFolderPicker = () => {
    fileInputRef.current?.click()
  }

  if (!isOpen) return null

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.9, opacity: 0 }}
        className="bg-dark-800 border border-white/10 rounded-xl shadow-2xl w-full max-w-lg"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="p-6 border-b border-white/10">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold flex items-center">
              <FiSettings className="mr-2 text-primary-400" />
              Settings
            </h2>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-white transition-colors"
            >
              ✕
            </button>
          </div>
        </div>

        <div className="p-6 space-y-6">
          {/* Hidden file input for folder selection */}
          <input
            type="file"
            ref={fileInputRef}
            onChange={handleFolderSelect}
            {...{ webkitdirectory: '', directory: '', multiple: true }}
            style={{ display: 'none' }}
          />

          {/* Download Path */}
          <div>
            <label className="block text-sm font-medium mb-2 text-gray-300">
              <FiFolder className="inline mr-2" />
              Download Path
            </label>
            
            <div className="flex space-x-2 mb-2">
              <div className="relative flex-1">
                <input
                  type="text"
                  value={settings.downloadPath}
                  onChange={(e) => {
                    setSettings({ ...settings, downloadPath: e.target.value })
                    setPathSelected(false)
                  }}
                  className={`input-field flex-1 pr-10 ${pathError ? 'border-red-500 focus:border-red-500' : pathValid && settings.downloadPath ? 'border-green-500/50' : ''}`}
                  placeholder="/path/to/folder or ~/Downloads"
                />
                {settings.downloadPath && (
                  <button
                    type="button"
                    onClick={clearPath}
                    className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-red-400 transition-colors"
                    title="Clear path"
                  >
                    <FiTrash2 size={14} />
                  </button>
                )}
              </div>
              <button
                type="button"
                onClick={openFolderPicker}
                className="btn-secondary flex items-center whitespace-nowrap"
                title="Select folder"
              >
                <FiFolderPlus className="mr-2" />
                Browse
              </button>
            </div>

            {/* Quick actions */}
            <div className="flex items-center space-x-2 mb-3">
              <button
                type="button"
                onClick={setToHome}
                className="text-xs text-primary-400 hover:text-primary-300 flex items-center transition-colors"
              >
                <FiHome className="mr-1" />
                Use ~/Downloads
              </button>
              <span className="text-gray-600">•</span>
              <span className="text-xs text-gray-500">
                Use ~ for home directory
              </span>
            </div>

            {/* Path validation feedback */}
            <AnimatePresence>
              {pathError && (
                <motion.div
                  initial={{ opacity: 0, y: -5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="text-red-400 text-sm mt-2 flex items-center"
                >
                  <FiAlertCircle className="mr-1" />
                  {pathError}
                </motion.div>
              )}
            </AnimatePresence>
            
            <AnimatePresence>
              {pathValid && settings.downloadPath && !pathError && (
                <motion.p
                  initial={{ opacity: 0, y: -5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="text-green-400 text-sm mt-2 flex items-center"
                >
                  <FiCheck className="mr-1" />
                  Valid path
                </motion.p>
              )}
            </AnimatePresence>

            <AnimatePresence>
              {pathSelected && (
                <motion.p
                  initial={{ opacity: 0, y: -5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="text-green-400 text-sm mt-2 flex items-center"
                >
                  <FiCheck className="mr-1" />
                  Folder selected successfully
                </motion.p>
              )}
            </AnimatePresence>

            <p className="text-gray-500 text-xs mt-2">
              Videos will be downloaded to this location
            </p>
          </div>

          {/* Quality Selection */}
          <div>
            <label className="block text-sm font-medium mb-2 text-gray-300">
              <FiDownload className="inline mr-2" />
              Preferred Quality
            </label>
            <select
              value={settings.preferredQuality}
              onChange={(e) => setSettings({ ...settings, preferredQuality: e.target.value })}
              className="input-field"
            >
              <option value="highest">Highest Available</option>
              <option value="1080p">1080p</option>
              <option value="720p">720p</option>
              <option value="480p">480p</option>
              <option value="audio">Audio Only (MP3)</option>
            </select>
          </div>

          {/* Concurrent Downloads */}
          <div>
            <label className="block text-sm font-medium mb-2 text-gray-300">
              <FiCpu className="inline mr-2" />
              Max Concurrent Downloads: {settings.maxConcurrentDownloads}
            </label>
            <input
              type="range"
              min="1"
              max="10"
              value={settings.maxConcurrentDownloads}
              onChange={(e) => setSettings({ ...settings, maxConcurrentDownloads: parseInt(e.target.value) })}
              className="w-full accent-primary-500"
            />
          </div>

          {/* Toggles */}
          <div className="space-y-4">
            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-gray-300">Auto-download when URL is pasted</span>
              <input
                type="checkbox"
                checked={settings.autoDownload}
                onChange={(e) => setSettings({ ...settings, autoDownload: e.target.checked })}
                className="w-5 h-5 rounded border-gray-600 bg-gray-700 text-primary-500 focus:ring-primary-500"
              />
            </label>

            <label className="flex items-center justify-between cursor-pointer">
              <span className="text-gray-300">Enable notifications</span>
              <input
                type="checkbox"
                checked={settings.notifications}
                onChange={(e) => setSettings({ ...settings, notifications: e.target.checked })}
                className="w-5 h-5 rounded border-gray-600 bg-gray-700 text-primary-500 focus:ring-primary-500"
              />
            </label>
          </div>
        </div>

        <div className="p-6 border-t border-white/10 flex justify-end space-x-3">
          <button
            onClick={onClose}
            className="btn-secondary"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="btn-primary flex items-center"
          >
            {isSaving ? (
              <>
                <FiRefreshCw className="animate-spin mr-2" />
                Saving...
              </>
            ) : (
              <>
                <FiSave className="mr-2" />
                Save Settings
              </>
            )}
          </button>
        </div>
      </motion.div>
    </motion.div>
  )
}

