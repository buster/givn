#!/usr/bin/env node
// Stand-in process tree for the supervisor probes. Like the real runtime
// driver it is the leader of the supervised process group and owns a twin, a
// server, a shell descendant and one stubborn descendant that ignores SIGTERM,
// so the supervisor must escalate to SIGKILL. Every member prints one JSON
// readiness line naming its role, PID and (where it listens) owned port.

import { spawn } from 'node:child_process'

const listener = (role, ignoreTerm) => `
import net from 'node:net'
${ignoreTerm ? "process.on('SIGTERM', () => {})" : "process.on('SIGTERM', () => process.exit(0))"}
const server = net.createServer((socket) => { socket.on('error', () => {}); socket.end('${role}-alive') })
server.listen(0, '127.0.0.1', () => console.log(JSON.stringify({ role: '${role}', pid: process.pid, port: server.address().port })))
setInterval(() => {}, 1000)
`

const children = []
function start(role, source) {
  const child = spawn(process.execPath, ['--input-type=module', '-e', source], { stdio: ['ignore', 'pipe', 'inherit'] })
  children.push(child)
  child.stdout.on('data', (part) => process.stdout.write(part))
  return child
}

start('twin', listener('twin', false))
start('server', listener('server', false))
start('stubborn', listener('stubborn', true))
const shell = spawn('/bin/sh', ['-c', 'sleep 300 & echo "{\\"role\\":\\"shell\\",\\"pid\\":$$,\\"sleep\\":$!}"; wait'], { stdio: ['ignore', 'pipe', 'inherit'] })
shell.stdout.on('data', (part) => process.stdout.write(part))
console.log(JSON.stringify({ role: 'driver', pid: process.pid }))

// Like the real driver: on SIGTERM abort outstanding work and exit; members
// that ignore SIGTERM are left for the supervisor's SIGKILL escalation.
process.on('SIGTERM', () => process.exit(143))
process.on('SIGINT', () => process.exit(130))
setInterval(() => {}, 1000)
