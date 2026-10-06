#!/usr/bin/env node
// Deterministic loopback OpenAI-compatible provider twin for the OpenCode
// attribution E2E scenarios. It is a finite state machine selected by the
// requested model and the message content, never by a plugin-only switch.
// Requests outside the machine fail with a diagnostic response.

import http from 'node:http'
import fs from 'node:fs/promises'
import crypto from 'node:crypto'

export const SUBAGENT_PROMPT_MARKER = 'attribution subagent'
export const SUBAGENT_CHILD_PREFIX = 'You are a subagent spawned by another session.'
const FINAL_TEXT = 'Attribution fixture completed.'
const TITLE_TEXT = 'Attribution fixture'

function textOf(content) {
  if (typeof content === 'string') return content
  if (Array.isArray(content)) return content.map((part) => (typeof part === 'string' ? part : part?.text ?? '')).join('')
  return ''
}

function toolName(tool) {
  return tool?.function?.name ?? tool?.name
}

// Pure decision function: returns { kind, ... } for one chat completion body.
export function decide(body, command) {
  const model = String(body.model ?? '')
  const tools = (body.tools ?? []).map(toolName)
  const messages = body.messages ?? []
  const hasToolResult = messages.some((message) => message.role === 'tool' || message.tool_call_id)
  const shell = tools.find((name) => name === 'shell' || name === 'bash')
  const users = messages.filter((message) => message.role === 'user').map((message) => textOf(message.content))
  const firstUser = users[0] ?? ''
  const lastUser = users.at(-1) ?? ''

  if (model === 'title-model') return { kind: 'title' }
  if (hasToolResult) return { kind: 'final' }
  if (!shell && !tools.includes('subagent')) return { kind: 'text' }
  if (firstUser.startsWith(SUBAGENT_CHILD_PREFIX)) {
    if (model !== 'model-b') return { kind: 'reject', reason: `subagent child reached the provider with model ${model}, expected model-b` }
    if (!shell) return { kind: 'reject', reason: 'subagent child was not offered a shell tool' }
    return { kind: 'tool', name: shell, args: { command } }
  }
  if (tools.includes('subagent') && lastUser.includes(SUBAGENT_PROMPT_MARKER)) {
    if (model !== 'model-a') return { kind: 'reject', reason: `primary subagent request used model ${model}, expected model-a` }
    return {
      kind: 'tool',
      name: 'subagent',
      args: { agent: 'attribution-subagent', description: 'Run the attribution fixture', prompt: 'Run the attribution fixture command.' },
    }
  }
  if (shell) return { kind: 'tool', name: shell, args: { command } }
  return { kind: 'reject', reason: `unexpected request: model=${model} tools=${JSON.stringify(tools)}` }
}

function completion(decision, model) {
  if (decision.kind === 'tool') {
    return {
      message: { role: 'assistant', content: null, tool_calls: [{ id: `fixture-call-${crypto.randomUUID().slice(0, 8)}`, type: 'function', function: { name: decision.name, arguments: JSON.stringify(decision.args) } }] },
      finish: 'tool_calls',
    }
  }
  const content = decision.kind === 'title' ? TITLE_TEXT : FINAL_TEXT
  return { message: { role: 'assistant', content }, finish: 'stop', model }
}

export function createServer({ logPath, command }) {
  return http.createServer(async (request, response) => {
    const chunks = []
    for await (const chunk of request) chunks.push(chunk)
    const body = Buffer.concat(chunks).toString('utf8')
    const log = (extra) => fs.appendFile(logPath, `${JSON.stringify({ method: request.method, url: request.url, ...extra })}\n`)

    if (request.method === 'GET' && request.url === '/api.json') {
      await log({})
      response.writeHead(200, { 'content-type': 'application/json' })
      response.end(JSON.stringify({}))
      return
    }
    if (request.method !== 'POST' || !request.url?.endsWith('/chat/completions')) {
      await log({ rejected: true })
      response.writeHead(404, { 'content-type': 'text/plain' })
      response.end('unexpected fixture request')
      return
    }
    let parsed
    try { parsed = JSON.parse(body) } catch {
      await log({ rejected: true, reason: 'malformed JSON' })
      response.writeHead(400)
      response.end('malformed JSON')
      return
    }
    const decision = decide(parsed, command)
    await log({ model: parsed.model, tools: (parsed.tools ?? []).map(toolName), decision: decision.kind, tool: decision.name, reason: decision.reason })
    if (decision.kind === 'reject') {
      response.writeHead(400, { 'content-type': 'text/plain' })
      response.end(`fixture provider rejected the request: ${decision.reason}`)
      return
    }
    const model = parsed.model ?? 'model-a'
    const { message, finish } = completion(decision, model)
    if (parsed.stream) {
      response.writeHead(200, { 'content-type': 'text/event-stream', 'cache-control': 'no-cache', connection: 'keep-alive' })
      const delta = message.tool_calls
        ? { role: 'assistant', tool_calls: message.tool_calls.map((call, index) => ({ index, id: call.id, type: call.type, function: call.function })) }
        : { role: 'assistant', content: message.content }
      const chunk = (d, reason) => `data: ${JSON.stringify({ id: 'fixture-response', object: 'chat.completion.chunk', created: 0, model, choices: [{ index: 0, delta: d, finish_reason: reason }] })}\n\n`
      response.write(chunk(delta, null))
      response.write(chunk({}, finish))
      response.end('data: [DONE]\n\n')
      return
    }
    response.writeHead(200, { 'content-type': 'application/json' })
    response.end(JSON.stringify({ id: 'fixture-response', object: 'chat.completion', created: 0, model, choices: [{ index: 0, message, finish_reason: finish }], usage: { prompt_tokens: 1, completion_tokens: 1, total_tokens: 2 } }))
  })
}

// Refusing forward proxy: records every destination OpenCode tries to reach
// (catalog refresh, package registry) and answers 403. It never forwards.
export function createRefusingProxy({ logPath }) {
  const record = (entry) => fs.appendFile(logPath, `${JSON.stringify({ proxy: true, ...entry })}\n`)
  const proxy = http.createServer(async (request, response) => {
    await record({ method: request.method, target: request.url })
    response.writeHead(403, { 'content-type': 'text/plain' })
    response.end('fixture proxy refuses outbound traffic')
  })
  proxy.on('connect', async (request, socket) => {
    await record({ method: 'CONNECT', target: request.url })
    socket.end('HTTP/1.1 403 Forbidden\r\ncontent-length: 0\r\nconnection: close\r\n\r\n')
  })
  proxy.on('clientError', (_error, socket) => socket.destroy())
  return proxy
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const portFile = process.argv[2]
  const nonce = crypto.randomUUID()
  const server = createServer({ logPath: process.env.GIVN_PROVIDER_LOG, command: process.env.GIVN_FIXTURE_COMMAND })
  server.on('clientError', (_error, socket) => socket.destroy())
  const proxy = createRefusingProxy({ logPath: process.env.GIVN_PROVIDER_LOG })
  proxy.listen(0, '127.0.0.1', () => {
    server.listen(0, '127.0.0.1', async () => {
      const address = server.address()
      await fs.writeFile(portFile, JSON.stringify({ nonce: process.env.GIVN_DRIVER_NONCE, providerNonce: nonce, port: address.port, proxyPort: proxy.address().port, pid: process.pid }))
    })
  })
  for (const signal of ['SIGINT', 'SIGTERM']) {
    process.on(signal, () => {
      server.close()
      proxy.close()
      // Do not wait forever for keep-alive connections.
      setTimeout(() => process.exit(0), 500).unref()
      server.once('close', () => process.exit(0))
    })
  }
}
