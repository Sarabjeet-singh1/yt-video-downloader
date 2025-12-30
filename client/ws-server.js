const WebSocket = require('ws')

function getPort() {
  const envPort = process.env.YD_WS_PORT ? Number(process.env.YD_WS_PORT) : 4001
  return envPort
}

let port = getPort()
let wss
let server

try {
  server = require('net').createServer()
  server.listen(port, () => {
    port = server.address().port
    server.close(() => startWsServer(port))
  })
  server.on('error', () => {
    // Port in use, try random port
    startWsServer(0)
  })
} catch (e) {
  startWsServer(0)
}

function startWsServer(listenPort) {
  wss = new WebSocket.Server({ port: listenPort })
  const clients = new Set()

  wss.on('connection', (ws) => {
    clients.add(ws)
    ws.send(JSON.stringify({ type: 'connected', message: 'Connected to YouTube downloader WS' }))

    ws.on('close', () => clients.delete(ws))
    ws.on('error', () => clients.delete(ws))
  })

  function broadcast(msg) {
    const data = typeof msg === 'string' || Buffer.isBuffer(msg) ? msg.toString() : JSON.stringify(msg)
    for (const c of clients) {
      if (c.readyState === WebSocket.OPEN) {
        try { c.send(data) } catch (e) { /* ignore */ }
      }
    }
  }

  console.log(`[ws-server] started on ws://localhost:${listenPort} (clients: 0)`)

  module.exports = { broadcast, port: listenPort, clients, wss }
}

