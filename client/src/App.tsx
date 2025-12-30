import { useState } from 'react'
import Header from './components/layout/Header'
import Sidebar from './components/layout/Sidebar'
import HomePage from './pages/HomePage'
import DownloadPage from './pages/DownloadPage'
import SettingsPage from './pages/SettingsPage'
import LoadingScreen from './components/ui/LoadingScreen'
import { useAppStore } from './store/appStore'
import './index.css'

function App() {
  const [currentPath, setCurrentPath] = useState('/')
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const { isLoading } = useAppStore()

  const handleNavigate = (path: string) => {
    setCurrentPath(path)
    // Close sidebar on mobile after navigation
    if (sidebarOpen) {
      setSidebarOpen(false)
    }
  }

  // Show loading screen while app initializes
  if (isLoading) {
    return <LoadingScreen />
  }

  const renderCurrentPage = () => {
    switch (currentPath) {
      case '/download':
        return <DownloadPage />
      case '/settings':
        return <SettingsPage onNavigate={handleNavigate} />
      case '/':
      default:
        return <HomePage onNavigate={handleNavigate} />
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 transition-colors duration-300">
      <div className="flex h-screen overflow-hidden">
        {/* Mobile sidebar overlay */}
        {sidebarOpen && (
          <div 
            className="fixed inset-0 z-40 bg-black/50 lg:hidden"
            onClick={() => setSidebarOpen(false)}
          />
        )}

        {/* Sidebar */}
        <div className={`fixed inset-y-0 left-0 z-50 w-80 transform transition-transform duration-300 ease-in-out lg:translate-x-0 lg:static lg:inset-0 ${
          sidebarOpen ? 'translate-x-0' : '-translate-x-full'
        }`}>
          <Sidebar 
            isOpen={sidebarOpen}
            onClose={() => setSidebarOpen(false)}
            currentPath={currentPath}
            onNavigate={handleNavigate}
          />
        </div>

        {/* Main content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <Header />
          
          <main className="flex-1 overflow-y-auto">
            <div className="container mx-auto px-4 py-6 max-w-7xl">
              {renderCurrentPage()}
            </div>
          </main>
        </div>
      </div>
    </div>
  )
}

export default App

