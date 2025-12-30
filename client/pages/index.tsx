import { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import { useForm } from 'react-hook-form'
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'
import toast, { Toaster } from 'react-hot-toast'
import { FiDownload, FiYoutube, FiLink, FiCheckCircle, FiAlertCircle, FiClock, FiRefreshCw, FiFolder, FiSettings } from 'react-icons/fi'
import Settings from '../components/Settings'

// Form validation schema
const schema = yup.object({
  url: yup
    .string()
    .required('YouTube URL is required')
    .matches(
      /^(https?:\/\/)?(www\.)?(youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)[a-zA-Z0-9_-]{11}/,
      'Please enter a valid YouTube URL'
    ),
})

interface FormData {
  url: string
  outputPath?: string
}

interface VideoInfo {
  title: string
  thumbnail: string
  duration: string
  channel: string
}

type DownloadStatus = 'idle' | 'validating' | 'queued' | 'downloading' | 'completed' | 'error'

interface SettingsData {
  downloadPath: string
  maxConcurrentDownloads: number
  preferredQuality: string
  autoDownload: boolean
  notifications: boolean
  theme: string
}

// Default settings - must be consistent between server and client to avoid hydration mismatch
const defaultSettings: SettingsData = {
  downloadPath: '~/Downloads',
  maxConcurrentDownloads: 3,
  preferredQuality: 'highest',
  autoDownload: false,
  notifications: true,
  theme: 'dark',
}

export default function Home() {
  const [url, setUrl] = useState<string>('')
  const [status, setStatus] = useState<DownloadStatus>('idle')
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null)
  const [progress, setProgress] = useState<number>(0)
  const [settings, setSettings] = useState<SettingsData>(defaultSettings)
  const [showSettings, setShowSettings] = useState(false)
  const [isClient, setIsClient] = useState(false)

  // Load settings from localStorage after hydration to avoid mismatch
  useEffect(() => {
    setIsClient(true)
    const saved = localStorage.getItem('downloaderSettings')
    if (saved) {
      try {
        const parsedSettings = JSON.parse(saved)
        setSettings(prev => ({ ...prev, ...parsedSettings }))
      } catch (e) {
        console.error('Failed to parse saved settings:', e)
      }
    }
  }, [])

  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
    watch,
  } = useForm<FormData>({
    resolver: yupResolver(schema),
    defaultValues: { url: '' },
  })

  const watchedUrl = watch('url')

  // Validate YouTube URL and fetch video info (mock for now)
  const validateUrl = async (url: string): Promise<VideoInfo | null> => {
    setStatus('validating')
    
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // Mock video info - in real app, this would call your backend
    const mockVideoInfo: VideoInfo = {
      title: 'Sample Video Title',
      thumbnail: `https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg`,
      duration: '3:32',
      channel: 'Example Channel'
    }
    
    setVideoInfo(mockVideoInfo)
    return mockVideoInfo
  }

  const handleDownload = async (data: FormData) => {
    if (!data.url) return

    // Validate download path before proceeding
    const path = settings.downloadPath || '~/Downloads'
    if (path && path.trim().length > 0) {
      // Check for obviously invalid paths
      const invalidChars = /[<>:"|?*\x00-\x1f]/
      const expandedPath = path.startsWith('~') 
        ? path.replace('~', process.env.HOME || process.env.USERPROFILE || '~')
        : path
      
      if (invalidChars.test(expandedPath)) {
        toast.error('Download path contains invalid characters')
        setStatus('error')
        return
      }
      
      // Check for relative path issues
      if (expandedPath === '.' || expandedPath === '..' || expandedPath === '') {
        toast.error('Please select a valid download path in Settings')
        setStatus('error')
        setShowSettings(true)
        return
      }
    }

    try {
      setStatus('validating')
      await validateUrl(data.url)
      
      setStatus('queued')
      toast.success('Download queued successfully!')
      
      // Simulate download progress
      setProgress(0)
      const progressInterval = setInterval(() => {
        setProgress(prev => {
          if (prev >= 100) {
            clearInterval(progressInterval)
            setStatus('completed')
            toast.success('Download completed!')
            return 100
          }
          return prev + 10
        })
      }, 500)

      // Make actual API call
      const res = await fetch('/api/download', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url: data.url, outputDir: settings.downloadPath }),
      })

      if (!res.ok) {
        const errorData = await res.json()
        throw new Error(errorData.message || 'Download failed')
      }
      
    } catch (err: any) {
      setStatus('error')
      toast.error('Download failed: ' + (err.message || String(err)))
    }
  }

  const getStatusIcon = () => {
    switch (status) {
      case 'validating':
        return <FiRefreshCw className="animate-spin" />
      case 'queued':
        return <FiClock />
      case 'downloading':
        return <FiDownload className="animate-pulse" />
      case 'completed':
        return <FiCheckCircle className="text-green-400" />
      case 'error':
        return <FiAlertCircle className="text-red-400" />
      default:
        return <FiLink />
    }
  }

  const getStatusText = () => {
    switch (status) {
      case 'validating':
        return 'Validating URL...'
      case 'queued':
        return 'Download queued'
      case 'downloading':
        return 'Downloading...'
      case 'completed':
        return 'Download completed!'
      case 'error':
        return 'Download failed'
      default:
        return 'Ready to download'
    }
  }

  const getStatusColor = () => {
    switch (status) {
      case 'validating':
        return 'status-loading'
      case 'queued':
        return 'status-badge text-yellow-400 border-yellow-500/30 bg-yellow-500/20'
      case 'downloading':
        return 'status-loading'
      case 'completed':
        return 'status-success'
      case 'error':
        return 'status-error'
      default:
        return 'status-badge text-gray-400 border-gray-500/30 bg-gray-500/20'
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-dark-900 via-slate-900 to-dark-800">
      <Toaster
        position="top-right"
        toastOptions={{
          duration: 4000,
          style: {
            background: 'rgba(15, 23, 42, 0.9)',
            color: '#fff',
            border: '1px solid rgba(255, 255, 255, 0.1)',
          },
        }}
      />
      
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="text-center mb-12"
        >
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center justify-center">
              <FiYoutube className="text-6xl text-red-500 mr-4" />
              <h1 className="text-5xl font-bold bg-gradient-to-r from-primary-400 to-purple-400 bg-clip-text text-transparent">
                YouTube Downloader
              </h1>
            </div>
            <button
              onClick={() => setShowSettings(true)}
              className="btn-secondary flex items-center"
            >
              <FiSettings className="mr-2" />
              Settings
            </button>
          </div>
          <p className="text-xl text-gray-300 max-w-2xl mx-auto">
            Download YouTube videos in high quality with our modern, easy-to-use interface
          </p>
        </motion.div>

        {/* Main Content */}
        <div className="max-w-4xl mx-auto">
          {/* Download Form */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.2 }}
            className="glass-card p-8 mb-8"
          >
            <form onSubmit={handleSubmit(handleDownload)} className="space-y-6">
              <div>
                <label htmlFor="url" className="block text-sm font-medium mb-2 text-gray-300">
                  YouTube URL
                </label>
                <div className="relative">
                  <input
                    {...register('url')}
                    type="text"
                    placeholder="https://youtube.com/watch?v=..."
                    className={`input-field pr-12 ${errors.url ? 'border-red-500 focus:border-red-500 focus:ring-red-500/20' : ''}`}
                  />
                  <FiYoutube className="absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-400" />
                </div>
                {errors.url && (
                  <motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    className="text-red-400 text-sm mt-2 flex items-center"
                  >
                    <FiAlertCircle className="mr-1" />
                    {errors.url.message}
                  </motion.p>
                )}
              </div>

              {/* Download Path Display */}
              <div className="flex items-center justify-between p-3 rounded-lg bg-white/5 border border-white/10">
                <div className="flex items-center space-x-2 text-gray-300">
                  <FiFolder className="text-primary-400" />
                  <span className="text-sm">Download to:</span>
                </div>
                <div className="flex items-center space-x-2">
                  <span className="text-sm text-gray-400 font-mono truncate max-w-[200px]">
                    {settings.downloadPath}
                  </span>
                  <button
                    type="button"
                    onClick={() => setShowSettings(true)}
                    className="text-xs text-primary-400 hover:text-primary-300 underline"
                  >
                    Change
                  </button>
                </div>
              </div>

              <motion.button
                type="submit"
                disabled={status === 'validating' || status === 'downloading'}
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
                className="btn-primary w-full flex items-center justify-center space-x-2 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {status === 'validating' ? (
                  <FiRefreshCw className="animate-spin" />
                ) : (
                  <FiDownload />
                )}
                <span>
                  {status === 'validating' ? 'Validating...' : 'Download Video'}
                </span>
              </motion.button>
            </form>

            {/* Status Display */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="mt-6 flex items-center justify-between p-4 rounded-lg bg-white/5 border border-white/10"
            >
              <div className="flex items-center space-x-3">
                <div className={`p-2 rounded-full ${getStatusColor().includes('success') ? 'bg-green-500/20' : 
                  getStatusColor().includes('error') ? 'bg-red-500/20' : 
                  getStatusColor().includes('loading') ? 'bg-blue-500/20' : 'bg-gray-500/20'}`}>
                  {getStatusIcon()}
                </div>
                <span className={`status-badge ${getStatusColor()}`}>
                  {getStatusText()}
                </span>
              </div>
              
              {(status === 'downloading' || status === 'completed') && (
                <div className="flex items-center space-x-3">
                  <div className="w-32 bg-gray-700 rounded-full h-2">
                    <motion.div
                      className="bg-gradient-to-r from-primary-500 to-purple-500 h-2 rounded-full"
                      initial={{ width: 0 }}
                      animate={{ width: `${progress}%` }}
                      transition={{ duration: 0.3 }}
                    />
                  </div>
                  <span className="text-sm text-gray-400 min-w-[3rem]">{progress}%</span>
                </div>
              )}
            </motion.div>
          </motion.div>

          {/* Features */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="grid md:grid-cols-3 gap-6"
          >
            <div className="glass-card p-6 text-center">
              <FiDownload className="text-3xl text-primary-400 mx-auto mb-4" />
              <h4 className="font-semibold mb-2">High Quality</h4>
              <p className="text-gray-400 text-sm">Download videos in the best available quality</p>
            </div>
            <div className="glass-card p-6 text-center">
              <FiClock className="text-3xl text-purple-400 mx-auto mb-4" />
              <h4 className="font-semibold mb-2">Fast & Reliable</h4>
              <p className="text-gray-400 text-sm">Quick downloads with progress tracking</p>
            </div>
            <div className="glass-card p-6 text-center">
              <FiCheckCircle className="text-3xl text-green-400 mx-auto mb-4" />
              <h4 className="font-semibold mb-2">Easy to Use</h4>
              <p className="text-gray-400 text-sm">Simple interface, just paste and download</p>
            </div>
          </motion.div>
        </div>
      </div>

      <Settings
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
        currentSettings={settings}
        onSettingsChange={setSettings}
      />
    </div>
  )
}
