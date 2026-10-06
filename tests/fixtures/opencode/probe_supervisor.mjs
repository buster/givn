#!/usr/bin/env node
// Controlled supervisor probes: SIGINT, SIGTERM, parent-lease EOF, runner exit
// and run deadline, each while an owned process group (driver, twin, server,
// shell descendant, SIGTERM-ignoring descendant) is alive. An unrelated
// sentinel process and listener, started outside the supervised group, must
// stay alive and responsive. Processes are only ever signalled by PID that this
// probe or the supervisor recorded, never by port.
//
// usage: probe_supervisor.mjs <path to opencode_supervisor> [case ...]

import { spawn } from 'node:child_process'
import fs from 'node:fs/promises'
import net from 'node:net'
import os from 'node:os'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const here = path.dirname(fileURLToPath(import.meta.url))
const supervisor = process.argv[2]
if (!supervisor) throw new Error('usage: probe_supervisor.mjs <opencode_supervisor> [case ...]')
const ALL_CASES = ['sigint', 'sigterm', 'lease-eof', 'runner-exit', 'deadline']
const cases = process.argv.length > 3 ? process.argv.slice(3) : ALL_CASES

const alive = (pid) => {
  try { process.kill(pid, 0); return true } catch (error) { return error.code === 'EPERM' }
}
const probePort = (port) => new Promise((resolve) => {
  const socket = net.connect({ host: '127.0.0.1', port })
  const chunks = []
  const timer = setTimeout(() => { socket.destroy(); resolve('timeout') }, 2000)
  socket.on('data', (part) => chunks.push(part))
  socket.on('error', (error) => { clearTimeout(timer); resolve(error.code ?? String(error)) })
  socket.on('close', () => { clearTimeout(timer); resolve(Buffer.concat(chunks).toString() || 'closed') })
})
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

async function startSentinel() {
  const source = `
import net from 'node:net'
const server = net.createServer((socket) => { socket.on('error', () => {}); socket.end('sentinel-alive') })
server.listen(0, '127.0.0.1', () => console.log(JSON.stringify({ pid: process.pid, port: server.address().port })))
setInterval(() => {}, 1000)
`
  const child = spawn(process.execPath, ['--input-type=module', '-e', source], { stdio: ['ignore', 'pipe', 'inherit'], detached: true })
  const line = await new Promise((resolve) => child.stdout.once('data', (part) => resolve(part.toString())))
  return { child, ...JSON.parse(line) }
}

function collect(stream, sink) {
  let buffer = ''
  stream.on('data', (part) => {
    buffer += part.toString()
    let index
    while ((index = buffer.indexOf('\n')) >= 0) {
      sink(buffer.slice(0, index))
      buffer = buffer.slice(index + 1)
    }
  })
}

async function runCase(name) {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'givn-opencode-probe-'))
  const sentinel = await startSentinel()
  const roles = {}
  const stderrLines = []
  const driverArgs = ['--root', root, '--', process.execPath, path.join(here, 'probe_driver.mjs')]
  const timeout = name === 'deadline' ? '2' : '60'
  const supervisorArgs = ['--timeout-seconds', timeout, ...driverArgs]
  let runner // process that holds the lease
  let supervisorPid
  const onStdout = (line) => {
    try { const record = JSON.parse(line); if (record.role) roles[record.role] = record } catch {}
  }
  const onStderr = (line) => {
    stderrLines.push(line)
    const match = line.match(/leader_pid=(\d+) pgid=(\d+)/)
    if (match && line.includes('started')) roles.group = { leader: Number(match[1]), pgid: Number(match[2]) }
  }
  let exit
  if (name === 'runner-exit') {
    // The runner owns the lease; it exits without any orderly teardown.
    const helper = `
import { spawn } from 'node:child_process'
const child = spawn(${JSON.stringify(supervisor)}, ${JSON.stringify(supervisorArgs)}, { stdio: ['pipe', 'inherit', 'inherit'] })
console.log(JSON.stringify({ role: 'supervisor', pid: child.pid }))
process.stdin.on('data', () => process.exit(0))
`
    runner = spawn(process.execPath, ['--input-type=module', '-e', helper], { stdio: ['pipe', 'pipe', 'pipe'] })
    collect(runner.stdout, onStdout)
    collect(runner.stderr, (line) => { onStderr(line); onStdout(line) })
    exit = new Promise((resolve) => runner.on('close', () => resolve({ via: 'runner stdio closed' })))
    // Supervisor stdout/stderr are inherited from the runner's pipes; wait for EOF.
  } else {
    runner = spawn(supervisor, supervisorArgs, { stdio: ['pipe', 'pipe', 'pipe'] })
    supervisorPid = runner.pid
    collect(runner.stdout, onStdout)
    collect(runner.stderr, onStderr)
    exit = new Promise((resolve) => runner.on('close', (code, signal) => resolve({ code, signal })))
  }
  const readyUntil = Date.now() + 15_000
  const wanted = ['driver', 'twin', 'server', 'shell', 'stubborn', 'group']
  while (Date.now() < readyUntil && !wanted.every((role) => roles[role]) ) await sleep(50)
  if (!wanted.every((role) => roles[role])) throw new Error(`${name}: owned processes did not become ready: ${JSON.stringify(roles)}`)
  if (name === 'runner-exit') supervisorPid = roles.supervisor?.pid
  const owned = {
    driver: roles.driver.pid, twin: roles.twin.pid, server: roles.server.pid, shell: roles.shell.pid,
    sleep: roles.shell.sleep, stubborn: roles.stubborn.pid,
  }
  const ports = { twin: roles.twin.port, server: roles.server.port, stubborn: roles.stubborn.port }
  const before = {
    owned: Object.fromEntries(Object.entries(owned).map(([role, pid]) => [role, alive(pid)])),
    ports: { twin: await probePort(ports.twin), server: await probePort(ports.server), stubborn: await probePort(ports.stubborn) },
    sentinel: { pid: sentinel.pid, alive: alive(sentinel.pid), response: await probePort(sentinel.port) },
  }
  const supervisorPidOfRecord = supervisorPid
  const trigger = { sigint: 'kill(supervisor, SIGINT)', sigterm: 'kill(supervisor, SIGTERM)', 'lease-eof': 'close supervisor stdin (parent lease)', 'runner-exit': 'runner process exits (lease EOF)', deadline: 'run deadline 2s' }[name]
  if (name === 'sigint') process.kill(supervisorPidOfRecord, 'SIGINT')
  else if (name === 'sigterm') process.kill(supervisorPidOfRecord, 'SIGTERM')
  else if (name === 'lease-eof') runner.stdin.end()
  else if (name === 'runner-exit') runner.stdin.write('exit\n')
  const result = await Promise.race([exit, sleep(30_000).then(() => ({ timedOut: true }))])
  if (result.timedOut) throw new Error(`${name}: supervisor did not finish within 30s`)
  // The supervisor's own teardown is done when it exits; allow the OS to publish it.
  if (name === 'runner-exit') for (let i = 0; i < 100 && alive(supervisorPidOfRecord); i += 1) await sleep(100)
  const after = {
    owned: Object.fromEntries(Object.entries(owned).map(([role, pid]) => [role, alive(pid)])),
    group_exists: (() => { try { process.kill(-roles.group.pgid, 0); return true } catch (error) { return error.code === 'EPERM' } })(),
    ports: { twin: await probePort(ports.twin), server: await probePort(ports.server), stubborn: await probePort(ports.stubborn) },
    sentinel: { pid: sentinel.pid, alive: alive(sentinel.pid), response: await probePort(sentinel.port) },
    supervisor_alive: alive(supervisorPidOfRecord),
    root_removed: !(await fs.stat(root).then(() => true, () => false)),
  }
  const lines = stderrLines.filter((line) => /process group|escalation|stopping/.test(line))
  const escalated = lines.some((line) => line.includes('signal=SIGKILL'))
  const reaped = lines.some((line) => line.includes('fixture process group reaped'))
  const checks = {
    all_owned_started: Object.values(before.owned).every(Boolean),
    all_owned_gone: Object.values(after.owned).every((value) => !value),
    group_gone: !after.group_exists,
    owned_ports_before_open: Object.values(before.ports).every((value) => value.endsWith('-alive')),
    owned_ports_closed: Object.values(after.ports).every((value) => value === 'ECONNREFUSED'),
    stubborn_escalated_to_sigkill: escalated,
    supervisor_reaped_group: reaped,
    supervisor_exited: !after.supervisor_alive,
    root_removed: after.root_removed,
    sentinel_alive_and_responsive: after.sentinel.alive && after.sentinel.response === 'sentinel-alive',
    nonzero_exit_reports_interruption: name === 'runner-exit' || result.code !== 0,
  }
  const ok = Object.values(checks).every(Boolean)
  const record = { case: name, trigger, ok, supervisor_pid: supervisorPidOfRecord, exit: result, group: roles.group, owned, ports, before, after, supervisor_log: lines, checks }
  await fs.rm(root, { recursive: true, force: true })
  try { process.kill(sentinel.pid, 'SIGTERM') } catch {}
  return record
}

let failures = 0
for (const name of cases) {
  if (!ALL_CASES.includes(name)) throw new Error(`unknown probe case ${name}`)
  let record
  try { record = await runCase(name) } catch (error) { record = { case: name, ok: false, error: String(error.stack ?? error) } }
  if (!record.ok) failures += 1
  console.log(JSON.stringify(record))
}
console.log(JSON.stringify({ probe: 'summary', cases, failures }))
process.exit(failures === 0 ? 0 : 1)
