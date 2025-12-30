import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { FiDownload, FiClock, FiCheckCircle, FiAlertCircle, FiTrash2, FiExternalLink } from 'react-icons/fi'

interface DownloadItem {
  id: string
  title: string
  thumbnail: string
  url: string
  status: 'completed' | 'downloading' | 'error' | 'queued'
  progress: number
  timestamp: Date
  fileSize?: string
}

interface DownloadHistoryProps {
  downloads: DownloadItem[]
  onRemove: (id: string) => void
  onRetry: (id: string) => void
}

export default function DownloadHistory({ downloads, onRemove, onRetry }: DownloadHistoryProps) {
  const [expandedItems, setExpandedItems] = useState<Set<string>>(new Set())

  const toggleExpanded = (id: string) => {
    const newExpanded = new Set(expandedItems)
    if (newExpanded.has(id)) {
      newExpanded.delete(id)
    } else {
      newExpanded.add(id)
    }
    setExpandedItems(newExpanded)
  }

  const getStatusIcon = (status: DownloadItem['status']) => {
    switch (status) {
      case 'completed':
        return <FiCheckCircle className="text-green-400" />
      case 'downloading':
        return <FiDownload className="text-blue-400 animate-pulse" />
      case 'error':
        return <FiAlertCircle className="text-red-400" />
      case 'queued':
        return <FiClock className="text-yellow-400" />
    }
  }

  const getStatusColor = (status: DownloadItem['status']) => {
    switch (status) {
      case 'completed':
        return 'bg-green-500/20 text-green-400 border-green-500/30'
      case 'downloading':
        return 'bg-blue-500/20 text-blue-400 border-blue-500/30'
      case 'error':
        return 'bg-red-500/20 text-red-400 border-red-500/30'
      case 'queued':
        return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30'
    }
  }

  if (downloads.length === 0) {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="glass-card p-8 text-center"
      >
        <FiDownload className="text-4xl text-gray-500 mx-auto mb-4" />
        <h3 className="text-lg font-medium text-gray-400 mb-2">No downloads yet</h3>
        <p className="text-gray-500">Your download history will appear here</p>
      </motion.div>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      className="glass-card p-6"
    >
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-xl font-semibold flex items-center">
          <FiDownload className="mr-2 text-primary-400" />
          Download History
        </h3>
        <span className="text-sm text-gray-400">{downloads.length} items</span>
      </div>

      <div className="space-y-4 max-h-96 overflow-y-auto">
        <AnimatePresence>
          {downloads.map((download, index) => (
            <motion.div
              key={download.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ delay: index * 0.1 }}
              className="bg-white/5 border border-white/10 rounded-lg p-4 hover:bg-white/10 transition-all duration-200"
            >
              <div className="flex items-start space-x-4">
                <img
                  src={download.thumbnail}
                  alt={download.title}
                  className="w-16 h-12 object-cover rounded"
                />
                
                <div className="flex-1 min-w-0">
                  <div className="flex items-start justify-between">
                    <h4 className="font-medium text-sm line-clamp-2 pr-4">{download.title}</h4>
                    <div className="flex items-center space-x-2">
                      <span className={`status-badge text-xs ${getStatusColor(download.status)}`}>
                        {getStatusIcon(download.status)}
                      </span>
                      <button
                        onClick={() => toggleExpanded(download.id)}
                        className="p-1 hover:bg-white/10 rounded"
                      >
                        <FiExternalLink className="text-gray-400" />
                      </button>
                      <button
                        onClick={() => onRemove(download.id)}
                        className="p-1 hover:bg-red-500/20 rounded text-red-400"
                      >
                        <FiTrash2 />
                      </button>
                    </div>
                  </div>

                  {download.status === 'downloading' && (
                    <div className="mt-2">
                      <div className="flex items-center justify-between text-xs text-gray-400 mb-1">
                        <span>Progress</span>
                        <span>{download.progress}%</span>
                      </div>
                      <div className="w-full bg-gray-700 rounded-full h-1">
                        <motion.div
                          className="bg-gradient-to-r from-primary-500 to-purple-500 h-1 rounded-full"
                          initial={{ width: 0 }}
                          animate={{ width: `${download.progress}%` }}
                          transition={{ duration: 0.3 }}
                        />
                      </div>
                    </div>
                  )}

                  <div className="flex items-center justify-between mt-2">
                    <span className="text-xs text-gray-500">
                      {download.timestamp.toLocaleTimeString()}
                    </span>
                    {download.fileSize && (
                      <span className="text-xs text-gray-400">{download.fileSize}</span>
                    )}
                  </div>

                  {expandedItems.has(download.id) && (
                    <motion.div
                      initial={{ opacity: 0, height: 0 }}
                      animate={{ opacity: 1, height: 'auto' }}
                      exit={{ opacity: 0, height: 0 }}
                      className="mt-3 pt-3 border-t border-white/10"
                    >
                      <p className="text-xs text-gray-400 break-all mb-2">{download.url}</p>
                      {download.status === 'error' && (
                        <button
                          onClick={() => onRetry(download.id)}
                          className="text-xs text-primary-400 hover:text-primary-300 underline"
                        >
                          Retry Download
                        </button>
                      )}
                    </motion.div>
                  )}
                </div>
              </div>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </motion.div>
  )
}
