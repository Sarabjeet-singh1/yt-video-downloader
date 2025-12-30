import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { UIState, DownloadProgress, LogEntry, Theme } from '@/types'

interface AppStore extends UIState {
  // Theme management
  theme: Theme
  setTheme: (theme: Theme) => void
  
  // UI state management
  setLoading: (loading: boolean) => void
  setDownloading: (downloading: boolean) => void
  setDownloadProgress: (progress: DownloadProgress | undefined) => void
  setCurrentStep: (step: string) => void
  setShowAdvanced: (show: boolean) => void
  toggleSidebar: () => void
  
  // Log management
  addLog: (entry: Omit<LogEntry, 'timestamp'>) => void
  clearLogs: () => void
  
  // Download management
  startDownload: (url: string, enableWallpaper: boolean) => void
  completeDownload: (success: boolean, error?: string) => void
  
  // Config management
  updateConfig: (updates: Partial<AppStore>) => void
}

const useAppStore = create<AppStore>()(
  persist(
    (set, get) => ({
      // Initial state
      isLoading: false,
      isDownloading: false,
      logs: [],
      currentStep: '',
      showAdvanced: false,
      sidebarOpen: false,
      theme: 'system',
      
      // Theme management
      setTheme: (theme: Theme) => set({ theme }),
      
      // UI state management
      setLoading: (loading: boolean) => set({ isLoading: loading }),
      setDownloading: (downloading: boolean) => set({ isDownloading: downloading }),
      setDownloadProgress: (progress: DownloadProgress | undefined) => set({ downloadProgress: progress }),
      setCurrentStep: (step: string) => set({ currentStep: step }),
      setShowAdvanced: (show: boolean) => set({ showAdvanced: show }),
      toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      // Log management
      addLog: (entry: Omit<LogEntry, 'timestamp'>) => set((state) => ({
        logs: [
          ...state.logs,
          {
            ...entry,
            timestamp: new Date().toISOString(),
          }
        ].slice(-100) // Keep only last 100 logs
      })),
      
      clearLogs: () => set({ logs: [] }),
      
      // Download management
      startDownload: (url: string, enableWallpaper: boolean) => {
        set({
          isDownloading: true,
          downloadProgress: {
            percentage: 0,
            speed: '0 B/s',
            eta: 'Calculating...',
            downloaded: '0 B',
            total: '0 B',
            status: 'downloading',
            stage: 'Initializing...'
          },
          currentStep: 'Starting download...',
          logs: []
        })
        
        get().addLog({
          level: 'info',
          message: `Starting download for: ${url}`,
          source: 'app'
        })
        
        if (enableWallpaper) {
          get().addLog({
            level: 'info',
            message: 'Wallpaper installation enabled',
            source: 'app'
          })
        }
      },
      
      completeDownload: (success: boolean, error?: string) => {
        if (success) {
          set({
            isDownloading: false,
            downloadProgress: {
              ...get().downloadProgress!,
              status: 'completed',
              stage: 'Download completed successfully!'
            },
            currentStep: 'Completed!'
          })
          
          get().addLog({
            level: 'success',
            message: 'Download completed successfully',
            source: 'app'
          })
        } else {
          set({
            isDownloading: false,
            downloadProgress: {
              ...get().downloadProgress!,
              status: 'error',
              stage: error || 'Download failed'
            },
            currentStep: 'Failed'
          })
          
          get().addLog({
            level: 'error',
            message: error || 'Download failed',
            source: 'app'
          })
        }
      },
      
      // Config management
      updateConfig: (updates: Partial<AppStore>) => set(updates)
    }),
    {
      name: 'youtube-downloader-app',
      partialize: (state) => ({
        theme: state.theme,
        showAdvanced: state.showAdvanced
      })
    }
  )
)

export { useAppStore }
export default useAppStore

