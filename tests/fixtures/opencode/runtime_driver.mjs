#!/usr/bin/env node
// Real-runtime orchestration for the OpenCode attribution scenarios.
//
// usage: runtime_driver.mjs <setup-smoke|run> <v1_floor|v1_latest|v2_latest> [main|subagent]
//
// The driver does not emulate the plugin loader. It starts the loopback
// provider twin and (for V2) an owned foreground server, runs the actual
// OpenCode CLI against an already prepared project containing the generated
// canonical plugin, and normalizes the completed shell-tool result.

import { spawn } from 'node:child_process'
import fs from 'node:fs/promises'
import path from 'node:path'
import crypto from 'node:crypto'
import net from 'node:net'
import { fileURLToPath } from 'node:url'
import { SANDBOX_EXEC, profileText, requireSandbox } from './sandbox.mjs'

const here = path.dirname(fileURLToPath(import.meta.url))
const runtimes = {
  v1_floor: process.env.GIVN_OPENCODE_V1_FLOOR,
  v1_latest: process.env.GIVN_OPENCODE_V1_LATEST,
  v2_latest: process.env.GIVN_OPENCODE_V2_LATEST,
}
const ATTRIBUTION_NAMES = ['GIVN_ATTRIBUTION_ACTIVE', 'GIVN_ATTRIBUTION_LLM', 'GIVN_ATTRIBUTION_REASONING', 'GIVN_ATTRIBUTION_HARNESS', 'GIT_COMMITTER_NAME']
const SENTINEL = 'GIVN_ATTRIBUTION_FIXTURE_RESULT '
const MAIN_PROMPT = 'Run the attribution fixture command.'
const SUBAGENT_PROMPT = 'Ask the attribution subagent to run the attribution fixture command.'
const SERVER_PASSWORD = 'fixture-server-password'
const CLI_TIMEOUT_MS = 30_000
// Destinations the pinned runtimes try to reach on startup. The refusing proxy
// answers 403 and Seatbelt blocks everything else; any other attempt fails the run.
const KNOWN_REFUSED_DESTINATIONS = new Set(['models.opencode.ai:443', 'registry.npmjs.org:443'])

function required(value, name) {
  if (!value) throw new Error(`required runtime setting ${name} is absent`)
  return value
}

function run(executable, args, options = {}) {
  return new Promise((resolve, reject) => {
    const { timeout, ...spawnOptions } = options
    const child = spawn(executable, args, { stdio: ['ignore', 'pipe', 'pipe'], ...spawnOptions })
    const stdout = []
    const stderr = []
    let timedOut = false
    child.stdout.on('data', (part) => stdout.push(part))
    child.stderr.on('data', (part) => stderr.push(part))
    child.on('error', reject)
    child.on('close', (code, signal) => {
      resolve({ code, signal, timedOut, stdout: Buffer.concat(stdout).toString(), stderr: Buffer.concat(stderr).toString() })
    })
    if (timeout) {
      setTimeout(() => {
        timedOut = true
        child.kill('SIGTERM')
      }, timeout).unref()
    }
  })
}

const root = process.cwd()
const mode = process.argv[2] ?? 'setup-smoke'
const release = process.argv[3] ?? 'v2_latest'
const scenario = process.argv[4] ?? 'main'
const project = process.env.GIVN_OPENCODE_PROJECT ?? path.join(root, 'project')
const generatedPlugin = path.join(project, '.opencode', 'plugins', 'givn-attribution.js')
const node = required(process.env.GIVN_OPENCODE_NODE, 'GIVN_OPENCODE_NODE')
const modules = required(process.env.GIVN_OPENCODE_MODULES, 'GIVN_OPENCODE_MODULES')
const fish = required(process.env.GIVN_OPENCODE_FISH, 'GIVN_OPENCODE_FISH')
const git = required(process.env.GIVN_OPENCODE_GIT, 'GIVN_OPENCODE_GIT')
required(process.env.TMPDIR, 'TMPDIR')
const executable = required(runtimes[release], release)
const isV2 = release === 'v2_latest'
// A substitute plugin is a negative control only; an exported variable alone must never pass the suite.
const controlPlugin = Boolean(process.env.GIVN_FIXTURE_CONTROL_PLUGIN)
if (controlPlugin && process.env.GIVN_OPENCODE_NEGATIVE_CONTROL !== '1') {
  throw new Error('GIVN_FIXTURE_CONTROL_PLUGIN is only valid as a negative control: also set GIVN_OPENCODE_NEGATIVE_CONTROL=1')
}
const resultPath = process.env.GIVN_FIXTURE_RESULT_PATH
const markerPath = path.join(root, 'provider-ready.json')
const providerLog = path.join(root, 'provider.log')
const attributionOutput = path.join(root, 'attribution-result.json')
// Contract probe (test-only): an extra recorder plugin beside the canonical one.
const probeLog = process.env.GIVN_FIXTURE_CONTRACT_PROBE ? path.join(root, 'contract-probe.jsonl') : null
const PROBE_PLUGIN = 'givn-contract-probe.js'
const nonce = crypto.randomUUID()
// The V2 shell is zsh (its login PATH reset cannot resolve a bare 'node'), so the
// fixed command uses the pinned absolute Node path.
const fixtureCommand = `${node} attribution-fixture.mjs`
const childEnvironment = {
  HOME: path.join(root, 'home'),
  TMPDIR: path.join(root, 'tmp'),
  XDG_CONFIG_HOME: path.join(root, 'config'),
  XDG_DATA_HOME: path.join(root, 'data'),
  XDG_CACHE_HOME: path.join(root, 'cache'),
  XDG_STATE_HOME: path.join(root, 'state'),
  GIT_CONFIG_GLOBAL: path.join(root, 'gitconfig'),
  GIT_CONFIG_SYSTEM: '/dev/null',
  GIT_CONFIG_NOSYSTEM: '1',
  GIVN_PROVIDER_LOG: providerLog,
  GIVN_DRIVER_NONCE: nonce,
  GIVN_FIXTURE_API_KEY: 'fixture-not-a-secret',
  GIVN_FIXTURE_COMMAND: fixtureCommand,
  ...(probeLog ? { GIVN_CONTRACT_PROBE_LOG: probeLog } : {}),
  PATH: `${path.dirname(node)}:${path.dirname(git)}:/usr/bin:/bin:/usr/sbin:/sbin`,
  LANG: 'C.UTF-8',
  LC_ALL: 'C.UTF-8',
  TERM: 'dumb',
  NO_PROXY: 'localhost,127.0.0.1,::1',
}
let provider
let server
const serverLog = []
let exitCode = 0

async function stop(child) {
  if (!child || child.exitCode !== null) return
  child.kill('SIGTERM')
  await Promise.race([new Promise((resolve) => child.once('exit', resolve)), new Promise((resolve) => setTimeout(resolve, 2500))])
  if (child.exitCode === null) child.kill('SIGKILL')
}

async function sandboxed(program, args, outboundPorts = []) {
  requireSandbox()
  const profile = path.join(root, `network-${crypto.randomUUID()}.sb`)
  await fs.writeFile(profile, profileText(outboundPorts))
  return { command: SANDBOX_EXEC, args: ['-f', profile, program, ...args] }
}

async function spawnSandboxed(program, args, options, outboundPorts = []) {
  const wrapped = await sandboxed(program, args, outboundPorts)
  return spawn(wrapped.command, wrapped.args, options)
}

async function assertNoAncestorOpenCodeDiscovery() {
  let directory = path.dirname(project)
  while (true) {
    for (const candidate of [path.join(directory, '.opencode'), path.join(directory, 'opencode.json'), path.join(directory, 'opencode.jsonc')]) {
      let exists = true
      try { await fs.access(candidate) } catch (error) {
        if (error.code !== 'ENOENT') throw error
        exists = false
      }
      if (exists) throw new Error(`OpenCode ancestor discovery would escape the fixture project: ${candidate}`)
    }
    const parent = path.dirname(directory)
    if (parent === directory) return
    directory = parent
  }
}

async function validateGeneratedPlugin() {
  await assertNoAncestorOpenCodeDiscovery()
  const entries = await fs.readdir(path.dirname(generatedPlugin)).catch(() => [])
  const expectedEntries = probeLog ? ['givn-attribution.js', PROBE_PLUGIN] : ['givn-attribution.js']
  if (JSON.stringify([...entries].sort()) !== JSON.stringify(expectedEntries)) {
    throw new Error(`the project must contain exactly ${JSON.stringify(expectedEntries)}; found ${JSON.stringify(entries)}`)
  }
  const source = await fs.readFile(generatedPlugin, 'utf8')
  const expected = await fs.readFile(path.resolve(here, '../../../assets/agent-integrations/opencode/givn-attribution.js'), 'utf8')
  if (source !== expected && !controlPlugin) throw new Error('the generated canonical plugin differs from the embedded asset')
}

function ndjson(stdout) {
  return stdout.split(/\r?\n/).filter(Boolean).flatMap((line) => {
    try { return [JSON.parse(line)] } catch { return [] }
  })
}

// Every completed tool part found anywhere in a parsed JSON value.
function completedTools(value, found = []) {
  if (Array.isArray(value)) value.forEach((item) => completedTools(item, found))
  else if (value && typeof value === 'object') {
    if (value.type === 'tool' && value.state?.status === 'completed') found.push(value)
    Object.values(value).forEach((item) => completedTools(item, found))
  }
  return found
}

// The run stream names the tool `tool` and carries `state.output`; the V2 session
// message API names it `name` and carries `state.content[].text`.
function toolName(part) {
  return part.tool ?? part.name
}

function toolOutput(part) {
  if (typeof part.state.output === 'string') return part.state.output
  return (part.state.content ?? []).map((entry) => entry?.text ?? '').join('')
}

function shellResult(toolParts) {
  const parts = toolParts.filter((part) => ['shell', 'bash'].includes(toolName(part)))
  if (parts.length !== 1) throw new Error(`expected exactly one completed shell tool result, found ${parts.length}`)
  const output = toolOutput(parts[0])
  const line = output.split(/\r?\n/).find((entry) => entry.startsWith(SENTINEL))
  if (!line) throw new Error(`completed shell tool result omitted the attribution sentinel: ${output}`)
  return { output, record: JSON.parse(line.slice(SENTINEL.length)) }
}

async function startProvider() {
  provider = await spawnSandboxed(node, [path.join(here, 'provider_stub.mjs'), markerPath], { cwd: root, env: childEnvironment, stdio: ['ignore', 'inherit', 'inherit'] })
  const readyUntil = Date.now() + 15_000
  let readiness
  while (Date.now() < readyUntil) {
    try { readiness = JSON.parse(await fs.readFile(markerPath, 'utf8')); break } catch {}
    if (provider.exitCode !== null) throw new Error(`provider twin exited early: ${provider.exitCode}`)
    await new Promise((resolve) => setTimeout(resolve, 50))
  }
  if (!readiness || readiness.nonce !== nonce || typeof readiness.providerNonce !== 'string') throw new Error('provider twin readiness nonce is malformed')
  return readiness
}

function proxyEnvironment(readiness) {
  const url = `http://127.0.0.1:${readiness.proxyPort}`
  return { HTTP_PROXY: url, HTTPS_PROXY: url, http_proxy: url, https_proxy: url }
}

// The subagent's configured model comes from the scenario (default: the fixture's model-b at variant low).
const subagentModel = process.env.GIVN_FIXTURE_SUBAGENT_MODEL ?? 'givn-fixture/model-b#low'

function projectConfig(providerUrl) {
  if (isV2) {
    return {
      '$schema': 'https://opencode.ai/config.json',
      model: 'givn-fixture/model-a',
      shell: fish,
      snapshots: false,
      providers: {
        'givn-fixture': {
          name: 'Attribution fixture',
          env: ['GIVN_FIXTURE_API_KEY'],
          package: '@opencode/ai/providers/openai-compatible',
          settings: { baseURL: providerUrl },
          models: {
            'model-a': { capabilities: { tools: true, input: ['text'], output: ['text'] }, limit: { context: 32768, output: 4096 }, variants: [{ id: 'high', settings: {} }] },
            'model-b': { capabilities: { tools: true, input: ['text'], output: ['text'] }, variants: [{ id: 'low', settings: {} }] },
            'title-model': {},
          },
        },
      },
      agents: {
        title: { model: 'givn-fixture/title-model' },
        'attribution-subagent': { mode: 'subagent', description: 'Runs the attribution fixture command.', model: subagentModel },
      },
      plugins: ['-opencode.provider.ollama', '-opencode.provider.lmstudio', '-opencode.provider.vllm'],
      permissions: [
        { action: 'shell', resource: fixtureCommand, effect: 'allow' },
        { action: 'subagent', resource: 'attribution-subagent', effect: 'allow' },
      ],
    }
  }
  return {
    '$schema': 'https://opencode.ai/config.json',
    provider: { 'givn-fixture': { npm: '@ai-sdk/openai-compatible', name: 'Attribution fixture', options: { baseURL: providerUrl, apiKey: 'fixture-not-a-secret' }, models: { 'model-a': { tool_call: true, variants: { high: {} } } } } },
    model: 'givn-fixture/model-a',
    snapshot: false,
    autoupdate: false,
    permission: { bash: { '*': 'deny', [fixtureCommand]: 'allow' } },
  }
}

async function writeFixtureProject(providerUrl) {
  await fs.writeFile(path.join(project, 'opencode.json'), JSON.stringify(projectConfig(providerUrl), null, 2))
  await fs.writeFile(path.join(project, 'package.json'), JSON.stringify({ type: 'module' }))
  await fs.symlink(modules, path.join(project, 'node_modules'))
  const script = [
    "import { writeFileSync } from 'node:fs'",
    `const names = ${JSON.stringify([...ATTRIBUTION_NAMES, 'OPENCODE_SESSION_ID'])}`,
    'const result = Object.fromEntries(names.map((name) => [name, process.env[name] ?? null]))',
    `console.log(${JSON.stringify(SENTINEL)} + JSON.stringify(result))`,
    `writeFileSync(${JSON.stringify(attributionOutput)}, JSON.stringify(result))`,
    '',
  ].join('\n')
  await fs.writeFile(path.join(project, 'attribution-fixture.mjs'), script)
}

async function freePort() {
  return new Promise((resolve, reject) => {
    const listener = net.createServer()
    listener.once('error', reject)
    listener.listen(0, '127.0.0.1', () => {
      const { port } = listener.address()
      listener.close(() => resolve(port))
    })
  })
}

function authHeaders() {
  return { Authorization: `Basic ${Buffer.from(`opencode:${SERVER_PASSWORD}`).toString('base64')}` }
}

async function startServer(readiness, version) {
  const serverOutput = serverLog
  for (let attempt = 1; attempt <= 3; attempt += 1) {
    const apiPort = await freePort()
    server = await spawnSandboxed(executable, ['serve', '--hostname', '127.0.0.1', '--port', String(apiPort)], { cwd: project, env: { ...childEnvironment, ...proxyEnvironment(readiness), OPENCODE_SERVER_PASSWORD: SERVER_PASSWORD }, stdio: ['ignore', 'pipe', 'pipe'] }, [readiness.port, readiness.proxyPort, apiPort])
    server.stdout.on('data', (part) => serverOutput.push(part))
    server.stderr.on('data', (part) => serverOutput.push(part))
    const until = Date.now() + 15_000
    let info
    while (Date.now() < until && !info) {
      if (server.exitCode !== null) break
      try {
        const response = await fetch(`http://127.0.0.1:${apiPort}/api/info`, { headers: authHeaders() })
        if (response.ok) info = await response.json()
      } catch {}
      if (!info) await new Promise((resolve) => setTimeout(resolve, 50))
    }
    if (server.exitCode !== null && attempt < 3) continue // bind collision: retry on a new port
    const tmpRoot = path.resolve(root, 'tmp', 'opencode')
    const reportedTmp = info?.paths?.tmp ? path.resolve(info.paths.tmp) : ''
    const expectedVersion = version.replace(/^opencode v?/, '')
    if (!info || info.pid !== server.pid || info.version !== expectedVersion || (reportedTmp !== tmpRoot && !reportedTmp.startsWith(`${tmpRoot}${path.sep}`))) {
      throw new Error(`OpenCode server ownership check failed: spawned_pid=${server.pid}, expected_version=${expectedVersion}, fixture_tmp_root=${tmpRoot}, info=${JSON.stringify(info)}, output=${Buffer.concat(serverOutput).toString()}`)
    }
    return { apiPort, info, output: serverOutput }
  }
  throw new Error('OpenCode server bind retries exhausted')
}

async function api(apiPort, pathname, query = {}) {
  const url = new URL(pathname, `http://127.0.0.1:${apiPort}`)
  for (const [key, value] of Object.entries(query)) url.searchParams.set(key, value)
  const response = await fetch(url, { headers: authHeaders() })
  if (!response.ok) throw new Error(`GET ${url.pathname} failed: HTTP ${response.status}`)
  return response.json()
}

async function activePlugin(apiPort) {
  const listing = await api(apiPort, '/api/plugin', { 'location[directory]': project })
  const entry = listing.data?.find((plugin) => plugin.source?.path && path.resolve(plugin.source.path) === path.resolve(generatedPlugin))
  const local = (listing.data ?? []).filter((plugin) => plugin.source?.type !== 'builtin')
  return { entry, local }
}

async function main() {
  for (const directory of ['home', 'config/opencode', 'data/opencode', 'cache', 'state', 'tmp']) {
    await fs.mkdir(path.join(root, directory), { recursive: true })
  }
  if (mode === 'run') await validateGeneratedPlugin()
  else await fs.mkdir(project, { recursive: true })
  await fs.writeFile(path.join(root, 'config/opencode/opencode.json'), JSON.stringify(isV2 ? { update: 'disable' } : { autoupdate: false }))
  const readiness = await startProvider()
  const providerUrl = `http://127.0.0.1:${readiness.port}/v1`
  await writeFixtureProject(providerUrl)
  const version = (await run(executable, ['--version'], { cwd: project, env: childEnvironment }))
  if (version.code !== 0) throw new Error(`OpenCode version probe failed: ${version.stderr}`)
  const versionText = version.stdout.trim()

  let serverInfo = null
  let apiPort
  if (isV2) {
    const started = await startServer(readiness, versionText)
    apiPort = started.apiPort
    serverInfo = started.info
    const { entry } = await activePlugin(apiPort)
    console.log(JSON.stringify({ kind: 'v2-plugin-discovery-before-run', generatedPlugin, active: entry ?? null }))
  }
  if (mode === 'setup-smoke') {
    console.log(JSON.stringify({ kind: 'runtime-smoke', release, version: versionText, providerPort: readiness.port, providerNonce: readiness.providerNonce, proxyPort: readiness.proxyPort, serverPid: server?.pid ?? null }))
    return
  }

  if (scenario === 'subagent' && !isV2) throw new Error('the subagent scenario is V2 only')
  const prompt = scenario === 'subagent' ? SUBAGENT_PROMPT : MAIN_PROMPT
  const debug = process.env.GIVN_DRIVER_DEBUG ? ['--print-logs', '--log-level', 'DEBUG'] : []
  const args = isV2
    ? ['run', '--server', `http://127.0.0.1:${apiPort}`, '--format', 'json', '--auto', '--model', 'givn-fixture/model-a#high', prompt]
    : ['run', ...debug, '--format', 'json', '--auto', '--model', 'givn-fixture/model-a', '--variant', 'high', prompt]
  const ports = isV2 ? [readiness.port, readiness.proxyPort, apiPort] : [readiness.port, readiness.proxyPort]
  const cli = await sandboxed(executable, args, ports)
  const env = { ...childEnvironment, ...proxyEnvironment(readiness), ...(isV2 ? { OPENCODE_PASSWORD: SERVER_PASSWORD } : {}) }
  const result = await run(cli.command, cli.args, { cwd: project, env, timeout: CLI_TIMEOUT_MS })
  process.stdout.write(result.stdout)
  process.stderr.write(result.stderr)
  const providerRequests = (await fs.readFile(providerLog, 'utf8').catch(() => '')).split('\n').filter(Boolean).map((line) => JSON.parse(line))
  const diagnose = () => `provider log: ${JSON.stringify(providerRequests)}; server log: ${Buffer.concat(serverLog).toString()}`
  if (result.timedOut) throw new Error(`OpenCode run exceeded the ${CLI_TIMEOUT_MS / 1000} second deadline; ${diagnose()}`)
  if (result.code !== 0) throw new Error(`OpenCode run exited ${result.code} (${result.signal ?? 'no signal'}); ${diagnose()}`)
  const refused = providerRequests.filter((entry) => entry.proxy).map((entry) => entry.target)
  const unknown = refused.filter((target) => !KNOWN_REFUSED_DESTINATIONS.has(target))
  if (unknown.length) throw new Error(`unexpected egress attempt through the refusing proxy: ${JSON.stringify(unknown)}`)
  const events = ndjson(result.stdout)
  const parentTools = completedTools(events)
  const parentSessionID = events.find((event) => event.sessionID)?.sessionID ?? null

  let recorded
  let child = null
  if (scenario === 'subagent') {
    const subagent = parentTools.filter((part) => toolName(part) === 'subagent')
    if (subagent.length !== 1) throw new Error(`expected exactly one completed subagent tool result in the parent stream, found ${subagent.length}; ${diagnose()}`)
    const output = toolOutput(subagent[0])
    const childSessionID = subagent[0].state.metadata?.sessionID ?? output.match(/sessionID="([^"]+)"/)?.[1]
    if (!childSessionID) throw new Error(`subagent tool result does not name the child session: ${output}`)
    if (childSessionID === parentSessionID) throw new Error('subagent child session equals the parent session')
    const messages = await api(apiPort, `/api/session/${encodeURIComponent(childSessionID)}/message`)
    try { recorded = shellResult(completedTools(messages)) } catch (error) { throw new Error(`${error.message}; child messages: ${JSON.stringify(messages).slice(0, 3000)}`) }
    if (recorded.record.OPENCODE_SESSION_ID !== childSessionID) throw new Error(`the subagent shell result belongs to session ${recorded.record.OPENCODE_SESSION_ID}, expected child ${childSessionID}`)
    child = { sessionID: childSessionID, parentSessionID, parentToolResult: output }
  } else {
    recorded = shellResult(parentTools)
    if (isV2 && recorded.record.OPENCODE_SESSION_ID !== parentSessionID) throw new Error(`shell result session ${recorded.record.OPENCODE_SESSION_ID} differs from run session ${parentSessionID}`)
  }
  const spawned = JSON.parse(await fs.readFile(attributionOutput, 'utf8'))
  if (JSON.stringify(spawned) !== JSON.stringify(recorded.record)) throw new Error(`completed tool result environment differs from spawned shell evidence: tool=${JSON.stringify(recorded.record)}, file=${JSON.stringify(spawned)}`)

  let plugins = null
  if (isV2) {
    const { entry, local } = await activePlugin(apiPort)
    // The control plugin only exists to show the primary assertion rejects it; its failed activation is recorded, not thrown.
    if (!controlPlugin && (!entry || entry.id !== 'givn-attribution' || entry.state?.status !== 'active' || local.length !== (probeLog ? 2 : 1))) {
      throw new Error(`V2 did not activate exactly one generated canonical plugin ${generatedPlugin}: entry=${JSON.stringify(entry)} local=${JSON.stringify(local)}`)
    }
    plugins = { entry, local }
  }
  const evidence = {
    kind: scenario === 'subagent' ? 'subagent-attribution-result' : 'attribution-result',
    release,
    version: versionText,
    scenario,
    generatedPlugin,
    argv: [executable, ...args],
    cli: { exitCode: result.code, signal: result.signal, stdout: result.stdout, stderr: result.stderr },
    toolOutput: recorded.output,
    result: Object.fromEntries(ATTRIBUTION_NAMES.map((name) => [name, spawned[name]])),
    sessionID: spawned.OPENCODE_SESSION_ID,
    child,
    server: serverInfo,
    plugins,
    contractProbe: probeLog ? (await fs.readFile(probeLog, 'utf8').catch(() => '')).split('\n').filter(Boolean).map((line) => JSON.parse(line)) : null,
    providerRequests: providerRequests.filter((entry) => !entry.proxy),
    refusedEgress: refused,
  }
  console.log(JSON.stringify({ kind: evidence.kind, release, version: versionText, result: evidence.result }))
  if (resultPath) await fs.writeFile(resultPath, JSON.stringify(evidence))
}

process.on('SIGTERM', () => { void Promise.all([stop(server), stop(provider)]).finally(() => process.exit(143)) })
process.on('SIGINT', () => { void Promise.all([stop(server), stop(provider)]).finally(() => process.exit(130)) })

try {
  await main()
} catch (error) {
  exitCode = 1
  console.error(`OpenCode runtime driver failure: ${error.stack ?? error}`)
} finally {
  await stop(server)
  await stop(provider)
}
process.exitCode = exitCode
