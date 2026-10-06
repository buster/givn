#!/usr/bin/env node

import { spawn } from 'node:child_process'
import fs from 'node:fs/promises'
import path from 'node:path'
import crypto from 'node:crypto'
import net from 'node:net'
import { fileURLToPath } from 'node:url'

const runtimes = {
  v1_floor: process.env.GIVN_OPENCODE_V1_FLOOR,
  v1_latest: process.env.GIVN_OPENCODE_V1_LATEST,
  v2_latest: process.env.GIVN_OPENCODE_V2_LATEST,
}

function required(value, name) {
  if (!value) throw new Error(`required runtime setting ${name} is absent`)
  return value
}

function run(executable, args, options = {}) {
  return new Promise((resolve, reject) => {
  const child = spawn(executable, args, { stdio: ['ignore', 'pipe', 'pipe'], ...options })
    const stdout = []
    const stderr = []
    child.stdout.on('data', (part) => stdout.push(part))
    child.stderr.on('data', (part) => stderr.push(part))
    child.on('error', reject)
    child.on('close', (code, signal) => resolve({ code, signal, stdout: Buffer.concat(stdout).toString(), stderr: Buffer.concat(stderr).toString() }))
    if (options.timeout) setTimeout(() => child.kill('SIGTERM'), options.timeout).unref()
  })
}

const root = process.cwd()
const project = path.join(root, 'project')
const generatedPlugin = path.join(project, '.opencode', 'plugins', 'givn-attribution.js')
required(process.env.TMPDIR, 'TMPDIR')
const node = required(process.env.GIVN_OPENCODE_NODE, 'GIVN_OPENCODE_NODE')
const modules = required(process.env.GIVN_OPENCODE_MODULES, 'GIVN_OPENCODE_MODULES')
const fish = required(process.env.GIVN_OPENCODE_FISH, 'GIVN_OPENCODE_FISH')
const git = required(process.env.GIVN_OPENCODE_GIT, 'GIVN_OPENCODE_GIT')
const givn = process.env.GIVN_COMPILED_GIVN
const mode = process.argv[2] ?? 'setup-smoke'
const release = process.argv[3] ?? 'v2_latest'
const resultPath = path.join(root, 'runtime-result.json')
const executable = required(runtimes[release], release)
const markerPath = path.join(root, 'provider-ready.json')
const providerLog = path.join(root, 'provider.log')
const attributionOutput = path.join(root, 'attribution-result.json')
const nonce = crypto.randomUUID()
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
  GIVN_OPENCODE_NODE: node,
  GIVN_OPENCODE_FISH: fish,
  GIVN_OPENCODE_GIT: git,
  GIVN_OPENCODE_MODULES: modules,
  GIVN_COMPILED_GIVN: givn,
  GIVN_OPENCODE_PROJECT: project,
  GIVN_FIXTURE_RESULT_PATH: resultPath,
  GIVN_FIXTURE_COMMAND: `${node} attribution-fixture.mjs`,
  PATH: `${path.dirname(node)}:${path.dirname(git)}:/usr/bin:/bin:/usr/sbin:/sbin`,
  LANG: 'C.UTF-8',
  LC_ALL: 'C.UTF-8',
  TERM: 'dumb',
  NO_PROXY: 'localhost,127.0.0.1,::1',
}
let provider
let server
let exitCode = 0

async function stop(child) {
  if (!child || child.exitCode !== null) return
  child.kill('SIGTERM')
  await Promise.race([new Promise((resolve) => child.once('exit', resolve)), new Promise((resolve) => setTimeout(resolve, 2500))])
  if (child.exitCode === null) child.kill('SIGKILL')
}

function sandboxCommand(executable, args, outboundPorts = []) {
  if (process.platform !== 'darwin') {
    throw new Error(`network-isolated OpenCode fixtures are not implemented for ${process.platform}; refusing host-network fallback`)
  }
  const profile = path.join(root, `network-${crypto.randomUUID()}.sb`)
  const outbound = outboundPorts.map((port) => `(allow network-outbound (remote ip "localhost:${port}"))`).join('\n')
  return fs.writeFile(profile, [
    '(version 1)',
    '(allow default)',
    '(deny network*)',
    '(allow network-bind (local ip "localhost:*"))',
    '(allow network-inbound (local ip "localhost:*"))',
    outbound,
    '',
  ].join('\n')).then(() => ({
    command: '/usr/bin/sandbox-exec',
    args: ['-f', profile, executable, ...args],
  }))
}

async function spawnSandboxed(executable, args, options = {}, outboundPorts = []) {
  const wrapped = await sandboxCommand(executable, args, outboundPorts)
  return spawn(wrapped.command, wrapped.args, options)
}

async function assertNoAncestorOpenCodeDiscovery() {
  let directory = path.dirname(project)
  while (true) {
    for (const candidate of [
      path.join(directory, '.opencode'),
      path.join(directory, 'opencode.json'),
      path.join(directory, 'opencode.jsonc'),
    ]) {
      try {
        await fs.access(candidate)
        throw new Error(`OpenCode ancestor discovery would escape the fixture project: ${candidate}`)
      } catch (error) {
        if (error.code !== 'ENOENT') throw error
      }
    }
    const parent = path.dirname(directory)
    if (parent === directory) return
    directory = parent
  }
}

async function validateGeneratedPlugin() {
  required(givn, 'GIVN_COMPILED_GIVN')
  await fs.mkdir(project, { recursive: true })
  await assertNoAncestorOpenCodeDiscovery()
  const gitInit = await run(git, ['init', '--quiet'], { cwd: project, env: childEnvironment })
  if (gitInit.code !== 0) throw new Error(`fixture git init failed: ${gitInit.stderr}`)
  const initialized = await run(givn, ['init', '--agents', 'opencode', '--no-addons'], { cwd: project, env: childEnvironment })
  if (initialized.code !== 0) throw new Error(`compiled givn init failed: stdout=${initialized.stdout} stderr=${initialized.stderr}`)
  if (initialized.stdout.includes('warning: skill generation failed')) throw new Error(`compiled givn init reported integration generation failure: ${initialized.stdout}`)
  const pluginDirectory = path.join(project, '.opencode', 'plugins')
  const entries = await fs.readdir(pluginDirectory)
  if (entries.length !== 1 || entries[0] !== 'givn-attribution.js') {
    throw new Error(`compiled givn init must generate exactly one canonical OpenCode plugin; found ${JSON.stringify(entries)}`)
  }
  const plugin = path.join(pluginDirectory, entries[0])
  if (!(await fs.stat(plugin)).isFile()) throw new Error(`generated canonical plugin is not a file: ${plugin}`)
  const source = await fs.readFile(plugin, 'utf8')
  const expectedAsset = await fs.readFile(path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../assets/agent-integrations/opencode/givn-attribution.js'), 'utf8')
  if (source !== expectedAsset) throw new Error('compiled Givn generated plugin differs from the canonical embedded asset')

  if (project !== path.join(root, 'project')) throw new Error(`OpenCode loader project does not match generated plugin project: ${project}`)
  return plugin
}

function completedToolResult(stdout) {
  const records = stdout.split(/\r?\n/).filter(Boolean).map((line) => {
    try { return JSON.parse(line) } catch { return null }
  }).filter(Boolean)
  const pending = [...records]
  while (pending.length) {
    const value = pending.pop()
    if (!value || typeof value !== 'object') continue
    if (value.type === 'tool_use' && value.part?.type === 'tool' && value.part.state?.status === 'completed') {
      return value.part.state.output
    }
    pending.push(...Object.values(value))
  }
  throw new Error('OpenCode stdout has no actual completed shell tool result event')
}

try {
  for (const directory of ['home', 'config/opencode', 'data/opencode', 'cache', 'state', 'tmp', ...(mode === 'setup-smoke' ? ['project'] : [])]) {
    await fs.mkdir(path.join(root, directory), { recursive: true })
  }
  if (mode !== 'setup-smoke') await validateGeneratedPlugin()
  const globalConfig = path.join(root, 'config/opencode/opencode.json')
  await fs.writeFile(globalConfig, JSON.stringify(release === 'v2_latest' ? { update: 'disable' } : { autoupdate: false }))
  provider = await spawnSandboxed(node, [path.join(path.dirname(fileURLToPath(import.meta.url)), 'provider_stub.mjs'), markerPath], {
    cwd: root, env: childEnvironment, stdio: ['ignore', 'inherit', 'inherit'],
  })
  const readyUntil = Date.now() + 15_000
  let readiness
  while (Date.now() < readyUntil) {
    try { readiness = JSON.parse(await fs.readFile(markerPath, 'utf8')); break } catch {}
    if (provider.exitCode !== null) throw new Error(`provider twin exited early: ${provider.exitCode}`)
    await new Promise((resolve) => setTimeout(resolve, 50))
  }
  if (!readiness || readiness.nonce !== nonce || typeof readiness.providerNonce !== 'string') throw new Error('provider twin readiness nonce is malformed')
  const providerUrl = `http://127.0.0.1:${readiness.port}/v1`
  const projectConfig = release === 'v2_latest'
    ? { '$schema': 'https://opencode.ai/config.json', model: 'givn-fixture/model-a', shell: fish, snapshots: false, providers: { 'givn-fixture': { name: 'Attribution fixture', env: ['GIVN_FIXTURE_API_KEY'], package: '@opencode/ai/providers/openai-compatible', settings: { baseURL: providerUrl }, models: { 'model-a': { capabilities: { tools: true, input: ['text'], output: ['text'] }, limit: { context: 32768, output: 4096, variants: [{ id: 'high', settings: {} }] }, variants: [{ id: 'high', settings: {} }] }, 'model-b': { variants: [{ id: 'low', settings: {} }] } } } }, plugins: ['-opencode.provider.ollama', '-opencode.provider.lmstudio', '-opencode.provider.vllm'], permissions: [{ action: 'shell', resource: `${node} attribution-fixture.mjs`, effect: 'allow' }], agent: { build: { tools: { '*': false, shell: true } } } }
    : { '$schema': 'https://opencode.ai/config.json', provider: { 'givn-fixture': { npm: '@ai-sdk/openai-compatible', name: 'Attribution fixture', options: { baseURL: providerUrl, apiKey: 'fixture-not-a-secret' }, models: { 'model-a': { tool_call: true, variants: { high: {} } } } } }, model: 'givn-fixture/model-a', snapshot: false, autoupdate: false, permission: { bash: { '*': 'deny', [`${node} attribution-fixture.mjs`]: 'allow' } } }
  await fs.writeFile(path.join(project, 'opencode.json'), JSON.stringify(projectConfig, null, 2))
  await fs.writeFile(path.join(project, 'package.json'), JSON.stringify({ type: 'module' }))
  await fs.symlink(modules, path.join(project, 'node_modules'))
  await fs.writeFile(path.join(project, 'attribution-fixture.mjs'), `import { writeFileSync } from 'node:fs'; const names=['GIVN_ATTRIBUTION_ACTIVE','GIVN_ATTRIBUTION_LLM','GIVN_ATTRIBUTION_REASONING','GIVN_ATTRIBUTION_HARNESS','GIT_COMMITTER_NAME']; const result=Object.fromEntries(names.map((name)=>[name,process.env[name]??null])); console.log('GIVN_ATTRIBUTION_FIXTURE_RESULT '+JSON.stringify(result)); writeFileSync(${JSON.stringify(attributionOutput)},JSON.stringify(result));`)
  const version = await run(executable, ['--version'], { cwd: project, env: childEnvironment })
  if (version.code !== 0) throw new Error(`OpenCode version probe failed: ${version.stderr}`)
  if (mode === 'smoke' || mode === 'setup-smoke') {
  const isV2 = release === 'v2_latest'
    let apiPort
    let activePluginList
    if (isV2) {
      apiPort = await new Promise((resolve, reject) => {
        const listener = net.createServer()
        listener.once('error', reject)
        listener.listen(0, '127.0.0.1', () => {
          const address = listener.address()
          listener.close(() => resolve(address.port))
        })
      })
      const serverArgs = ['serve', '--hostname', '127.0.0.1', '--port', String(apiPort)]
      server = await spawnSandboxed(executable, serverArgs, { cwd: project, env: { ...childEnvironment, OPENCODE_SERVER_PASSWORD: 'fixture-server-password' }, stdio: ['ignore', 'pipe', 'pipe'] }, [readiness.port, apiPort])
      const serverOutput = []
      server.stdout.on('data', (part) => serverOutput.push(part))
      server.stderr.on('data', (part) => serverOutput.push(part))
      const until = Date.now() + 15_000
      let info
      while (Date.now() < until) {
        if (server.exitCode !== null) throw new Error(`OpenCode server exited before readiness: ${server.exitCode}`)
        try {
          const response = await fetch(`http://127.0.0.1:${apiPort}/api/info`, { headers: { Authorization: `Basic ${Buffer.from(`opencode:fixture-server-password`).toString('base64')}` } })
          if (response.ok) info = await response.json()
          if (info) break
        } catch {}
        await new Promise((resolve) => setTimeout(resolve, 50))
      }
      const reportedTmp = info?.paths?.tmp
      const fixtureRoot = path.resolve(root)
      const normalizedTmp = reportedTmp ? path.resolve(reportedTmp) : ''
      const serverTmpRoot = path.resolve(fixtureRoot, 'tmp', 'opencode')
      const expectedVersion = version.stdout.trim().replace(/^opencode v?/, '')
      if (!info || info.pid !== server.pid || info.version !== expectedVersion || (normalizedTmp !== serverTmpRoot && !normalizedTmp.startsWith(`${serverTmpRoot}${path.sep}`))) throw new Error(`OpenCode server ownership check failed: spawned_pid=${server.pid}, expected_version=${expectedVersion}, fixture_tmp_root=${serverTmpRoot}, info=${JSON.stringify(info)}`)
      const pluginResponse = await fetch(`http://127.0.0.1:${apiPort}/api/plugin?${new URLSearchParams({ 'location[directory]': project })}`, { headers: { Authorization: `Basic ${Buffer.from('opencode:fixture-server-password').toString('base64')}` } })
      if (!pluginResponse.ok) throw new Error(`V2 plugin discovery diagnostic failed: HTTP ${pluginResponse.status}`)
      const discoveredPlugins = await pluginResponse.json()
      const entry = discoveredPlugins.data?.find((plugin) => path.resolve(plugin.source?.path ?? '') === path.resolve(generatedPlugin))
      console.log(JSON.stringify({ kind: 'v2-plugin-discovery-before-run', generatedPlugin, active: entry ?? null, all: discoveredPlugins }))
      server.logs = serverOutput
    }
    if (mode === 'setup-smoke') {
      console.log(JSON.stringify({ kind: 'runtime-smoke', release, version: version.stdout.trim(), providerPort: readiness.port, providerNonce: readiness.providerNonce, serverPid: server?.pid ?? null }))
      exitCode = 0
    } else {
    const args = isV2
      ? ['run', '--server', `http://127.0.0.1:${apiPort}`, '--format', 'json', '--auto', '--model', 'givn-fixture/model-a#high', 'Run the attribution fixture command.']
      : ['run', '--format', 'json', '--auto', '--model', 'givn-fixture/model-a', '--variant', 'high', 'Run the attribution fixture command.']
  const cli = await sandboxCommand(executable, args, isV2 ? [readiness.port, apiPort] : [readiness.port])
  const result = await run(cli.command, cli.args, { cwd: project, env: { ...childEnvironment, ...(isV2 ? { OPENCODE_PASSWORD: 'fixture-server-password' } : {}) }, timeout: 15_000 })
  process.stdout.write(result.stdout)
  process.stderr.write(result.stderr)
  if (result.code !== 0) {
    await fs.writeFile(path.join(root, 'driver-diagnostics.log'), Buffer.concat(server?.logs ?? []).toString())
    await fs.writeFile(path.join(root, 'provider-diagnostics.log'), await fs.readFile(providerLog).catch(() => ''))
    if (result.code === 128) throw new Error(`OpenCode run exceeded the 15 second provider deadline; provider log: ${path.join(root, 'provider-diagnostics.log')}`)
  }
    if (result.code !== 0) throw new Error(`OpenCode run exited ${result.code} (${result.signal ?? 'no signal'})`)
    const toolOutput = completedToolResult(result.stdout)
    const resultLine = toolOutput.split(/\r?\n/).find((line) => line.startsWith('GIVN_ATTRIBUTION_FIXTURE_RESULT '))
    if (!resultLine) throw new Error(`completed shell tool result omitted attribution fixture sentinel: ${toolOutput}`)
    const actualFromTool = JSON.parse(resultLine.slice('GIVN_ATTRIBUTION_FIXTURE_RESULT '.length))
    const actual = JSON.parse(await fs.readFile(attributionOutput, 'utf8'))
    if (JSON.stringify(actualFromTool) !== JSON.stringify(actual)) throw new Error(`completed tool result environment differs from spawned shell evidence: tool=${JSON.stringify(actualFromTool)}, file=${JSON.stringify(actual)}`)
    if (isV2) {
      const pluginUrl = new URL(`/api/plugin?${new URLSearchParams({ 'location[directory]': project })}`, `http://127.0.0.1:${apiPort}`)
      const pluginResponse = await fetch(pluginUrl, { headers: { Authorization: `Basic ${Buffer.from('opencode:fixture-server-password').toString('base64')}` } })
      if (!pluginResponse.ok) throw new Error(`V2 active-plugin diagnostic failed: HTTP ${pluginResponse.status}`)
      activePluginList = await pluginResponse.json()
      const generated = activePluginList.data?.find((entry) => entry.source?.path === generatedPlugin)
      if (!generated || generated.id !== 'givn-attribution' || generated.state?.status !== 'active') {
        throw new Error(`V2 did not activate exactly one generated canonical plugin ${generatedPlugin}: ${JSON.stringify(generated)}`)
      }
    }
    if (actual.GIVN_ATTRIBUTION_ACTIVE !== '1' || actual.GIVN_ATTRIBUTION_LLM !== 'givn-fixture/model-a' || actual.GIVN_ATTRIBUTION_REASONING !== 'high' || actual.GIVN_ATTRIBUTION_HARNESS !== 'opencode' || actual.GIT_COMMITTER_NAME !== 'LLM: givn-fixture/model-a via opencode') throw new Error(`actual shell result did not contain expected attribution: ${JSON.stringify(actual)}; V2 active plugins=${JSON.stringify(activePluginList ?? null)}; generated plugin=${generatedPlugin}`)
    if (isV2 && actual.GIVN_ATTRIBUTION_ACTIVE !== '1') throw new Error('V2 shell result is missing plugin attribution')
    if (isV2) console.log(JSON.stringify({ kind: 'v2-plugin-discovery', project, generatedPlugin, plugins: activePluginList }))
    const evidence = { kind: 'attribution-result', release, version: version.stdout.trim(), generatedPlugin, result: actual }
    console.log(JSON.stringify(evidence))
    if (resultPath) await fs.writeFile(resultPath, JSON.stringify(evidence))
    }
  }
} catch (error) {
  exitCode = 1
  console.error(`OpenCode runtime driver failure: ${error.stack ?? error}`)
} finally {
  await stop(server)
  await stop(provider)
}

process.exitCode = exitCode
