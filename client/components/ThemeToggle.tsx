import { useState, useEffect } from 'react'
import { FiMoon, FiSun } from 'react-icons/fi'
import { motion } from 'framer-motion'

export default function ThemeToggle() {
  const [isDark, setIsDark] = useState(true)

  useEffect(() => {
    const savedTheme = localStorage.getItem('theme')
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    
    if (savedTheme) {
      setIsDark(savedTheme === 'dark')
    } else {
      setIsDark(prefersDark)
    }
  }, [])

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark')
      localStorage.setItem('theme', 'dark')
    } else {
      document.documentElement.classList.remove('dark')
      localStorage.setItem('theme', 'light')
    }
  }, [isDark])

  return (
    <motion.button
      onClick={() => setIsDark(!isDark)}
      className="p-3 rounded-lg bg-white/10 hover:bg-white/20 border border-white/20 transition-all duration-200"
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
    >
      <motion.div
        initial={false}
        animate={{ rotate: isDark ? 0 : 180 }}
        transition={{ duration: 0.3 }}
      >
        {isDark ? (
          <FiMoon className="text-yellow-400" />
        ) : (
          <FiSun className="text-yellow-500" />
        )}
      </motion.div>
    </motion.button>
  )
}
