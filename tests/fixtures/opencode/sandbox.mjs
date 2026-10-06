// Darwin Seatbelt profile shared by the runtime driver and the isolation probe.
// The profile denies all network access except loopback bind/inbound and
// outbound connections to the driver-owned loopback ports. A broader
// `(allow network* (local ip "localhost:*"))` rule is deliberately avoided: it
// also permits non-loopback outbound sockets.

export const SANDBOX_EXEC = '/usr/bin/sandbox-exec'

export function profileText(outboundPorts = []) {
  for (const port of outboundPorts) {
    if (!Number.isInteger(port) || port < 1 || port > 65535) throw new Error(`invalid owned port: ${port}`)
  }
  return [
    '(version 1)',
    '(allow default)',
    '(deny network*)',
    '(allow network-bind (local ip "localhost:*"))',
    '(allow network-inbound (local ip "localhost:*"))',
    ...outboundPorts.map((port) => `(allow network-outbound (remote ip "localhost:${port}"))`),
    '',
  ].join('\n')
}

export function requireSandbox() {
  if (process.platform !== 'darwin') {
    throw new Error(`network-isolated OpenCode fixtures are not implemented for ${process.platform}; refusing host-network fallback`)
  }
}
