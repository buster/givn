# Proposal: bubblewrap-sandbox

## Problem / Opportunity

The test harness runs AI agents inside a sandlock sandbox for filesystem and
process isolation. In practice, sandlock is unreliable for this workload:

- The agent process tree (ralph -> node -> ralph.js -> bun -> opencode ->
  subprocesses) produces deep fork chains with many shared-library loads.
- Sandlock intercepts every `openat` syscall via seccomp user-notify
  (SECCOMP_IOCTL_NOTIF_RECV/SEND), routing each through a user-space supervisor
  for COW filesystem virtualization. Under multi-process workloads this
  supervisor becomes a bottleneck.
- Observed failures: the sandlock C layer returns NULL during sandbox creation
  (`sandlock_create_for_run` / `sandlock_start` fails, producing exit code -1
  with no error detail), or the wait call (`sandlock_handle_wait_timeout`)
  blocks indefinitely.
- These failures are intermittent and unrecoverable, making automated test
  runs non-deterministic. A single CI run can pass, hang, or fail with the
  same inputs.

The opportunity: replace sandlock with a sandbox mechanism that uses kernel
namespaces and bind mounts instead of seccomp user-notify, eliminating the
per-syscall interposition bottleneck while preserving the same observable
isolation guarantees.

## Proposed Solution

Replace sandlock with **bubblewrap** (`bwrap`) for agent sandboxing.
Bubblewrap uses Linux user namespaces, mount namespaces, PID namespaces, and
seccomp BPF to isolate processes. It does not use seccomp user-notify, so the
kernel handles all filesystem access directly — no per-syscall user-space
round trip.

### Observable behaviour (unchanged)

- **Isolated workspace.** Each test run gets a fresh temporary workspace. The
  agent cannot access files outside the whitelist.
- **Network access.** The agent has default-allow outbound network access (API
  calls to LLM providers, package registries, etc. must work).
- **Filesystem confinement.** The agent can only read from system paths
  (`/usr`, `/bin`, `/lib`, `/lib64`, `/etc`, home directory) and can only
  write to the workspace, `/tmp`, and user cache directories.
- **Workspace persistence.** On agent success, all workspace changes are
  preserved for evaluation. On agent failure, the workspace is still preserved
  (the harness always needs the workspace for evaluation; "commit on success"
  becomes "copy from sandbox tmpfs to workspace on success").
- **Process limit.** The agent cannot fork more than 256 processes.
- **Memory limit.** The agent cannot allocate more than 16 GB of memory.
- **Disk limit.** The agent cannot write more than 20 GB of data.
- **No root required.** The harness runs unprivileged. No sudo, no SUID
  binaries, no elevated permissions.

### What changes, observably

- **Agent execution is reliable.** No intermittent sandbox creation failures
  or indefinite hangs. The agent starts and produces output every time.
- **Performance is consistent.** No per-syscall overhead from COW
  virtualization. Agent startup time is predictable.
- **No learn-then-confine profile mode.** The first-run profile learning phase
  is removed. Filesystem access rules are declared in the harness
  configuration, not learned at runtime.
- **No port remapping.** The `port_remap` feature (transparent TCP port
  virtualization) is removed. The harness does not use it for testing.

## Out of Scope

- **Firejail alternative.** Firejail is not being considered. It requires a
  SUID binary installation, which violates the unprivileged requirement.
- **Seccomp BPF customization beyond bubblewrap defaults.** The harness uses
  bubblewrap's built-in seccomp filter or no additional seccomp filter. Custom
  syscall blocking rules are deferred.
- **Network namespace isolation.** The agent retains full outbound network
  access. Denying specific destinations or protocols is deferred.
- **HTTP ACL / MITM proxy.** Bubblewrap does not provide these. They were not
  used by the harness with sandlock either.
- **Multi-architecture support (ARM, etc.).** Only x86-64 Linux is required.

## Open Questions

None. Bubblewrap 0.11.0 is already installed on the system, the kernel
(6.12) supports all required namespace features, and the replacement maps
directly to the existing sandbox configuration in `sandbox.py`.