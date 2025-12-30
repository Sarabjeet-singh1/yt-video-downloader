import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
  message: string
  download_url?: string
  video_id?: string
  error?: string
}

// Your Render backend URL - replace with your actual Render service URL
const RENDER_BACKEND_URL = process.env.RENDER_BACKEND_URL || 'https://your-backend.onrender.com'

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

  try {
    // Forward the request to your Render backend
    const response = await fetch(`${RENDER_BACKEND_URL}/download`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ url, output_dir: outputDir }),
    })

    const data = await response.json()

    if (!response.ok) {
      return res.status(response.status).json({ 
        message: data.message || 'Download failed',
        error: data.error 
      })
    }

    return res.status(200).json({
      message: 'Download started successfully',
      download_url: data.download_url,
      video_id: data.video_id
    })
  } catch (err: unknown) {
    const errorMessage = err instanceof Error ? err.message : String(err)
    return res.status(500).json({ 
      message: 'Failed to connect to backend',
      error: errorMessage 
    })
  }
}

