#!/usr/bin/env node

import http from 'node:http'
import fs from 'node:fs/promises'
import crypto from 'node:crypto'

const portFile = process.argv[2]
const nonce = crypto.randomUUID()
const server = http.createServer(async (request, response) => {
  const chunks = []
  for await (const chunk of request) chunks.push(chunk)
  const body = Buffer.concat(chunks).toString('utf8')
  await fs.appendFile(process.env.GIVN_PROVIDER_LOG, `${JSON.stringify({ method: request.method, url: request.url, body })}\n`)

  if (request.method === 'GET' && request.url === '/api.json') {
    response.writeHead(200, { 'content-type': 'application/json' })
    response.end(JSON.stringify({ models: {} }))
    return
  }
  if (request.url?.startsWith('/api/plugin')) {
    response.writeHead(200, { 'content-type': 'application/json' })
    response.end(JSON.stringify({ plugins: [] }))
    return
  }
  if (request.method !== 'POST' || !request.url?.endsWith('/chat/completions')) {
    response.writeHead(404)
    response.end('unexpected fixture request')
    return
  }

  let parsed
  try { parsed = JSON.parse(body) } catch {
    response.writeHead(400)
    response.end('malformed JSON')
    return
  }
  const tools = parsed.tools ?? []
  const shell = tools.find((tool) => ['shell', 'bash'].includes(tool.function?.name ?? tool.name))
  const hasToolResult = (parsed.messages ?? []).some((message) => message.role === 'tool' || message.tool_call_id)
  const payload = hasToolResult
        ? { choices: [{ index: 0, message: { role: 'assistant', content: 'Attribution fixture completed.' }, finish_reason: 'stop' }] }
      : shell
        ? { choices: [{ index: 0, message: { role: 'assistant', content: null, tool_calls: [{ id: 'fixture-call-1', type: 'function', function: { name: shell.function?.name ?? shell.name, arguments: JSON.stringify({ command: process.env.GIVN_FIXTURE_COMMAND }) } }] }, finish_reason: 'tool_calls' }] }
      : { choices: [{ index: 0, message: { role: 'assistant', content: 'Attribution fixture completed.' }, finish_reason: 'stop' }] }
  const model = parsed.model ?? 'model-a'
  if (parsed.stream) {
    response.writeHead(200, { 'content-type': 'text/event-stream', 'cache-control': 'no-cache', connection: 'keep-alive' })
    const choice = payload.choices[0]
    const deltas = choice.message.tool_calls
      ? [
          { role: 'assistant', tool_calls: choice.message.tool_calls.map((call, index) => ({ index, id: call.id, type: call.type, function: call.function })) },
        ]
      : [{ role: 'assistant', content: choice.message.content ?? '' }]
    for (const delta of deltas) {
      response.write(`data: ${JSON.stringify({ id: 'fixture-response', object: 'chat.completion.chunk', created: 0, model, choices: [{ index: 0, delta, finish_reason: null }] })}\n\n`)
    }
    response.write(`data: ${JSON.stringify({ id: 'fixture-response', object: 'chat.completion.chunk', created: 0, model, choices: [{ index: 0, delta: {}, finish_reason: choice.finish_reason }] })}\n\n`)
    response.end('data: [DONE]\n\n')
    return
  }
  response.writeHead(200, { 'content-type': 'application/json' })
  response.end(JSON.stringify({ id: 'fixture-response', object: 'chat.completion', created: 0, model, ...payload, usage: { prompt_tokens: 1, completion_tokens: 1, total_tokens: 2 } }))
})

server.on('clientError', (_error, socket) => socket.destroy())
server.listen(0, '127.0.0.1', async () => {
  const address = server.address()
  await fs.writeFile(portFile, JSON.stringify({ nonce: process.env.GIVN_DRIVER_NONCE, providerNonce: nonce, port: address.port, pid: process.pid }))
})

for (const signal of ['SIGINT', 'SIGTERM']) {
  process.on(signal, () => server.close(() => process.exit(0)))
}
