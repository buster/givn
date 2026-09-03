# Proposal: run-command

## Problem / Opportunity

After a green change, the natural next question is: how does this product get
started? Today givn records how a project is *verified* (the test runner lives
in the project's own command file), but not how the built product is *started*.
Every consumer has to guess or ask:

1. Is the product a server, a command-line tool, or a library?
2. Which command starts it?
3. If it is a server: which port does it listen on, and how does a caller
   recognise that it is ready?

Automation built on givn — for example the kiekste preview host — must not
guess and must not interrogate the agent. Human developers opening the README
get no trustworthy "how to start" section either; whatever is written there is
free prose that no gate holds honest.

This information belongs in the same project-owned, versioned command file
that already records how the project is verified: it is project-specific, it is
reviewed in diffs, and it is part of what makes a change done.

Additionally, a declaration nobody executes is untested text. The declaration
is only trustworthy if the tool that reads it can also act on it.

## Proposed Solution

**Declaring.** Every givn project declares how its built product is started in
`givn/commands.yaml`, next to the verify commands. The declaration names the
product type (server, command-line tool, or library), the exact start command,
and — for a server — exactly one way the port is chosen (a fixed port, or a
named environment variable through which the caller hands the port over), the
path that answers once the server is ready to serve, and how long a caller
should wait for readiness.

**Scaffolding and backfill.** A freshly initialised project carries the
declaration scaffold with an obvious "not configured yet" placeholder command.
Existing projects receive the same scaffold through the managed upgrade path:
`givn upgrade` appends it, commits it together with the other managed files,
and tells the user it must be filled in before archiving. Running the upgrade
again changes nothing.

**Documenting.** The README gains a managed run block under Quick start,
rendered from the declaration: the start command for a tool, start command plus
port and readiness information for a server, and a one-line "nothing to start"
statement for a library. While the declaration is unconfigured, the block shows
an open placeholder that the existing README check reports. A damaged managed
block (exactly one marker present) is reported like any other damaged block.

**Validating.** The declaration is checked by deterministic rules — no
heuristics, no inferring the product type from source code or manifests. An
unconfigured or inconsistent declaration is reported with named findings by
`givn check review`, and `givn archive` blocks while findings remain: a change
is not done while the project does not document how it starts. A library is
allowed to keep the placeholder command; a declared library is complete as it
stands.

**Starting.** givn itself can start the product: `givn run` executes the
declared command. For a server, givn waits until the declared readiness path
answers, prints one greppable ready line naming the local address, and lets the
started product's output flow through. A caller may pass a port explicitly
(handed to the product through the declared environment variable); otherwise a
free port is allocated when the declaration says the port comes through the
environment, or the fixed declared port is used. `givn run --print` emits the
start plan as a single shell-evaluable line without starting anything.
Interrupting `givn run` (SIGINT/SIGTERM) stops the started product; givn waits
for it and reports itself via a distinguishable exit code. Exit codes separate
the outcomes: nothing to start (library), declaration problems before start,
readiness timeout, and the started product's own exit code, which is forwarded
unchanged.

**Nothing is guessed.** givn never derives the product type or the start
command from the repository's source code, build manifests, container files, or
framework conventions. The declaration is the single source of truth, and it is
the project author's responsibility — visible in review, enforced at the gate.

## Out of Scope

- givn never guesses the product type or start command: no derivation from
  source code, Cargo/package manifests, Dockerfiles, or framework conventions.
- No health monitoring beyond the declared readiness probe while
  `givn run` is running: no liveness polling after readiness, no uptime
  tracking, no restart on failure.
- No background or daemon mode for `givn run`: the command runs in the
  foreground until the product exits or the caller interrupts it.
- No migration bundle for the new scaffold — existing projects get it through
  the ordinary managed upgrade, because the new block is additive and nothing
  existing breaks.
- No new lifecycle artifact: the start declaration is configuration, not an
  artifact in the change graph.
- No hosting, remote coordination, or preview orchestration — starting is local
  and one-shot; orchestration around it remains the caller's job (kiekste).
- No readiness probe semantics beyond "answers with success": content
  inspection, authentication, or TLS are the product's own concerns.

## Open Questions

None. The previously open boundary — declaration only versus declaration and
execution — was resolved in exploration: this change ships both, with the
executor's machine contract (ready line, exit codes, port hand-off, signal
behaviour) specified as observable behaviour in the specs.
