#!/usr/bin/env node
// Actual-port Seatbelt proof. Drives real sandboxed Node clients under the same
// profile text the runtime driver uses and checks the observed connections:
//   owned loopback ports         -> allowed
//   unrelated loopback listener  -> denied (EPERM)
//   non-loopback destination     -> denied (EPERM)
//   twin profile (no outbound)   -> every outbound connection denied
// Exit status 0 only when every observation matches.

import { spawn } from 'node:child_process'
import fs from 'node:fs/promises'
import net from 'node:net'
import os from 'node:os'
import path from 'node:path'
import { SANDBOX_EXEC, profileText, requireSandbox } from './sandbox.mjs'

requireSandbox()
const NON_LOOPBACK = { host: '192.0.2.1', port: 443 } // TEST-NET-1, never routed

function listen() {
  return new Promise((resolve, reject) => {
    const server = net.createServer((socket) => {
      socket.on('error', () => {})
      socket.end('pong')
    })
    server.once('error', reject)
    server.listen(0, '127.0.0.1', () => resolve(server))
  })
}

const clientSource = `
import net from 'node:net'
const targets = JSON.parse(process.argv[2])
const attempt = ({ host, port }) => new Promise((resolve) => {
  const socket = net.connect({ host, port })
  const timer = setTimeout(() => { socket.destroy(); resolve('timeout') }, 3000)
  socket.once('connect', () => { clearTimeout(timer); socket.destroy(); resolve('connected') })
  socket.once('error', (error) => { clearTimeout(timer); resolve(error.code ?? String(error)) })
})
const out = {}
for (const [name, target] of Object.entries(targets)) out[name] = await attempt(target)
// A bind on loopback must also keep working inside the sandbox (twins need it).
out.bind = await new Promise((resolve) => {
  const server = net.createServer()
  server.once('error', (error) => resolve(error.code ?? String(error)))
  server.listen(0, '127.0.0.1', () => server.close(() => resolve('bound')))
})
console.log(JSON.stringify(out))
`

async function sandboxedClient(root, outboundPorts, targets) {
  const profile = path.join(root, `probe-${outboundPorts.join('-') || 'none'}.sb`)
  await fs.writeFile(profile, profileText(outboundPorts))
  const script = path.join(root, 'client.mjs')
  await fs.writeFile(script, clientSource)
  return new Promise((resolve, reject) => {
    const child = spawn(SANDBOX_EXEC, ['-f', profile, process.execPath, script, JSON.stringify(targets)], { stdio: ['ignore', 'pipe', 'pipe'] })
    const out = []
    const err = []
    child.stdout.on('data', (part) => out.push(part))
    child.stderr.on('data', (part) => err.push(part))
    child.on('error', reject)
    child.on('close', (code) => {
      if (code !== 0) return reject(new Error(`sandboxed client exited ${code}: ${Buffer.concat(err)}`))
      resolve({ profile: path.basename(profile), observed: JSON.parse(Buffer.concat(out).toString()) })
    })
  })
}

const root = await fs.mkdtemp(path.join(os.tmpdir(), 'givn-opencode-isolation-'))
const owned = [await listen(), await listen()]
const unrelated = await listen()
const [ownedA, ownedB] = owned.map((server) => server.address().port)
const targets = {
  owned_a: { host: '127.0.0.1', port: ownedA },
  owned_b: { host: '127.0.0.1', port: ownedB },
  unrelated_loopback: { host: '127.0.0.1', port: unrelated.address().port },
  non_loopback: NON_LOOPBACK,
}
let failures = 0
function expect(label, observed, expected) {
  const ok = Object.entries(expected).every(([key, value]) => observed[key] === value)
  if (!ok) failures += 1
  console.log(JSON.stringify({ probe: label, ok, expected, observed }))
}
try {
  const driver = await sandboxedClient(root, [ownedA, ownedB], targets)
  expect('driver-profile-owned-ports', driver.observed, { owned_a: 'connected', owned_b: 'connected', unrelated_loopback: 'EPERM', non_loopback: 'EPERM', bind: 'bound' })
  const twin = await sandboxedClient(root, [], targets)
  expect('twin-profile-no-outbound', twin.observed, { owned_a: 'EPERM', owned_b: 'EPERM', unrelated_loopback: 'EPERM', non_loopback: 'EPERM', bind: 'bound' })
} finally {
  for (const server of [...owned, unrelated]) server.close()
  await fs.rm(root, { recursive: true, force: true })
}
console.log(JSON.stringify({ probe: 'summary', ports: { owned: [ownedA, ownedB], unrelated: targets.unrelated_loopback.port }, failures }))
process.exit(failures === 0 ? 0 : 1)
