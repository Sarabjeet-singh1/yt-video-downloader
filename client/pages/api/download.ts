import type { NextApiRequest, NextApiResponse } from 'next'
import { spawn } from 'child_process'
import fs from 'fs'
import path from 'path'

// Initialize WS server module (starts on import)
// eslint-disable-next-line @typescript-eslint/no-var-requires
const wsServer = require('../../ws-server')

type Data = {
  message: string
  pid?: number
  logPath?: string
  outputDir?: string
}

function findBinary(): string | null {
  const repoRoot = path.resolve(process.cwd(), '..')
  const candidates = [
    path.join(repoRoot, 'target', 'release', 'rust_downloader'),
    path.join(repoRoot, 'target', 'release', 'rust-downloader'),
    path.join(repoRoot, 'target', 'debug', 'rust_downloader'),
    path.join(repoRoot, 'target', 'debug', 'rust-downloader'),
  ]

  for (const p of candidates) {
    if (fs.existsSync(p) && fs.statSync(p).isFile()) return p
  }
  return null
}

function expandPath(p: string): string {
  if (p.startsWith('~/')) {
    const home = process.env.HOME || process.env.USERPROFILE || ''
    return p.replace('~', home)
  }
  return p
}

function validateAndCreateDirectory(dirPath: string): { valid: boolean; message: string; resolvedPath?: string } {
  // Expand ~ to home directory
  const expandedPath = expandPath(dirPath)
  
  // Check if path is empty
  if (!expandedPath || expandedPath.trim().length === 0) {
    return { valid: false, message: 'Output path cannot be empty' }
  }

  // Check for invalid characters in path
  const invalidChars = /[<>:"|?*\x00-\x1f]/
  if (invalidChars.test(expandedPath)) {
    return { valid: false, message: 'Path contains invalid characters' }
  }

  try {
    // Check if directory exists, create if not
    if (!fs.existsSync(expandedPath)) {
      fs.mkdirSync(expandedPath, { recursive: true })
      return { valid: true, message: 'Directory created successfully', resolvedPath: expandedPath }
    }

    // Check if it's actually a directory
    const stats = fs.statSync(expandedPath)
    if (!stats.isDirectory()) {
      return { valid: false, message: 'Path exists but is not a directory' }
    }

    // Check if directory is writable
    try {
      const testFile = path.join(expandedPath, '.write_test_' + Date.now())
      fs.writeFileSync(testFile, 'test')
      fs.unlinkSync(testFile)
      return { valid: true, message: 'Directory is writable', resolvedPath: expandedPath }
    } catch (writeErr) {
      return { valid: false, message: 'Directory is not writable' }
    }
  } catch (err: unknown) {
    const errorMessage = err instanceof Error ? err.message : String(err)
    return { valid: false, message: `Failed to access directory: ${errorMessage}` }
  }
}

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  if (req.method !== 'POST') {
    res.setHeader('Allow', ['POST'])
    return res.status(405).json({ message: 'Method not allowed. Use POST.' })
  }

  const { url, outputDir } = req.body ?? {}
  
  // Validate URL
  if (!url || typeof url !== 'string') {
    return res.status(400).json({ message: 'Missing or invalid `url` in request body' })
  }

  // Basic URL validation
  const urlPattern = /^(https?:\/\/)?(www\.)?(youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)[a-zA-Z0-9_-]{11}/
  if (!urlPattern.test(url)) {
    return res.status(400).json({ message: 'Invalid YouTube URL format' })
  }

  // Validate or create output directory
  let finalOutputDir: string
  if (outputDir && typeof outputDir === 'string' && outputDir.trim().length > 0) {
    const validation = validateAndCreateDirectory(outputDir.trim())
    if (!validation.valid) {
      return res.status(400).json({ message: `Invalid output directory: ${validation.message}` })
    }
    finalOutputDir = validation.resolvedPath!
  } else {
    // Use default Downloads folder
    const defaultPath = expandPath('~/Downloads')
    const validation = validateAndCreateDirectory(defaultPath)
    if (!validation.valid) {
      return res.status(500).json({ message: `Failed to use default directory: ${validation.message}` })
    }
    finalOutputDir = validation.resolvedPath!
  }

  const bin = findBinary()
  if (!bin) {
    return res.status(500).json({ message: 'Rust binary not found. Build the project first with: cargo build --release' })
  }

  try {
    const outputsDir = path.join(path.resolve(process.cwd(), '..'), 'outputs')
    if (!fs.existsSync(outputsDir)) fs.mkdirSync(outputsDir, { recursive: true })

    const ts = Date.now()
    const logPath = path.join(outputsDir, `download-${ts}.log`)
    const outStream = fs.createWriteStream(logPath, { flags: 'a' })

    // Add diagnostic header to the log
    outStream.write(`Starting download for ${url} at ${new Date().toISOString()}\n`)
    outStream.write(`Binary: ${bin}\n`)
    outStream.write(`Output directory: ${finalOutputDir}\n\n`)

    // Call the binary with the URL and output directory
    const args = [url, '--output', finalOutputDir]
    outStream.write(`Command: ${bin} ${args.join(' ')}\n\n`)
    
    const child = spawn(bin, args, {
      detached: true,
      stdio: ['ignore', 'pipe', 'pipe']
    })

    child.stdout?.on('data', (chunk) => {
      outStream.write(chunk)
      try { wsServer.broadcast(chunk.toString()) } catch (e) { /* ignore */ }
    })
    child.stderr?.on('data', (chunk) => {
      outStream.write(chunk)
      try { wsServer.broadcast(chunk.toString()) } catch (e) { /* ignore */ }
    })
    child.on('error', (err) => {
      outStream.write(`\nSpawn error: ${String(err)}\n`)
      try { wsServer.broadcast(`ERROR: ${String(err)}`) } catch (e) { /* ignore */ }
      outStream.end()
    })
    child.on('close', (code) => {
      outStream.write(`\nProcess exited with code ${code}\n`)
      outStream.end()
    })

    // Detach so the process can continue after the request finishes
    child.unref()

    return res.status(202).json({ 
      message: 'Download started successfully', 
      pid: child.pid ?? undefined, 
      logPath,
      outputDir: finalOutputDir
    })
  } catch (err: unknown) {
    const errorMessage = err instanceof Error ? err.message : String(err)
    return res.status(500).json({ message: `Failed to start download: ${errorMessage}` })
  }
}

