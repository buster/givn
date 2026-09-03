---
status: accepted
date: 2026-09-02
decision-makers:
  - givn maintainers
consulted:
  - run-command proposal, executable specs, and design
  - kiekste-plan-v3 (kiekste Change 0 dependency and preview gates)
informed:
  - automation callers that drive givn (preview host, CI wrappers)
  - maintainers of consumer projects declaring their product start
---

# One tool declares and executes the product start command

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

### MADR checklist evidence

- [x] **Hard to reverse** - the ready line, exit-code table, port hand-off,
      and signal behaviour become public contracts that kiekste and caller
      scripts build on; unwinding execution later would strand those callers
      and re-split the contract.
- [x] **Shapes quality attributes** - one spec'd machine contract bounds how
      automation integrates: a single integration surface instead of
      per-caller process plumbing.
- [x] **Structural** - it adds a CLI surface, a process-supervision
      subsystem, and a declaration→execution data flow inside the
      single-binary boundary (ADR-004).
- [x] **Must be shared** - without this record an implementer would
      re-derive the executor boundary or move execution into a companion
      tool, splitting the contract; supporting only, never a gate.

- **alternatives** — three materially different options recorded below
  (declaration-only, execution only elsewhere via a companion tool,
  declare-and-execute in givn).
- **architectural_impact** — a new CLI surface plus process-supervision
  subsystem plus machine contract inside the existing single binary.
- **durable_consequence** — givn's public contract grows to include start
  execution; callers build on it, and the compatibility obligation is
  recorded in chapter 11 (R-45).
- **lower_level_artifact** — the declare-and-execute boundary is not
  expressible as a single scenario; the observable behaviour is owned by the
  `run-command` and `run-execution` capabilities, and no lower-level
  artifact owns the boundary itself.
- **existing_adr_check** — no existing ADR covers product execution
  (ADR-015 owns verification, ADR-028 owns gates); ADR-004 keeps its
  single-binary boundary and is honoured; ADR-028 is amended by this change.

## Context

After a green change, automation around givn must answer three questions about
the built product: what kind of product it is, which command starts it, and —
for a server — which port it listens on and how readiness is recognised. This
information exists nowhere today: the README documents it as free prose that no
gate holds honest, and an orchestrator would have to guess or interrogate the
agent. givn already owns the complementary declaration (the test runner in
`givn/commands.yaml`), so the start declaration belongs next to it —
author-owned, versioned, rendered into the README, and enforced at gates.

The open boundary was whether givn should also *execute* the declaration. The
original plan said no: the `run:` block would be declaration-only, and the
kiekste preview host would execute the start command itself. That position was
superseded during exploration of this change. A declaration nobody executes is
untested text: nothing proves the command starts the product, nothing pins the
port hand-off mechanics, and every caller re-implements readiness polling,
signal teardown, and exit-code interpretation — badly and differently.

## Decision drivers

- [x] **Trustworthy declaration** — a start declaration is only credible if the
      tool that reads it can also act on it; execution exercises the declared
      contract.
- [x] **One machine contract for automation** — callers (kiekste, CI wrappers)
      must build on one spec'd interface (ready line, exit codes, port
      hand-off), not on their own process plumbing.
- [x] **No guessing boundary preserved** — both declaring and executing stay
      strictly declaration-driven; givn derives nothing from repository
      content.
- [x] **Single-binary product boundary** — the executor must not reintroduce a
      daemon, a service, or a second binary (ADR-004).

## Considered options

1. **Declaration-only; an external orchestrator (kiekste) executes the start
   command.** — Rejected: the declaration stays unexercised, every caller
   re-implements readiness/teardown, and the declared port/ready fields are
   untested text until a caller happens to use them.
2. **Declaration in givn; execution by a separate companion tool.** — Rejected:
   a second artifact contradicts the single-binary boundary (ADR-004), splits
   the machine contract across two release cadences, and gives consumer
   projects one more install prerequisite for no architectural gain.
3. **Declaration and execution in givn (chosen).** — `givn run` executes the
   declared command as a foreground child with a spec'd contract: blocking
   readiness probe, greppable ready line, forwarded exit codes, forwarded
   signals on unix, `--port`/`--print` forms. Orchestration *around* it
   (previews, restarts, fleet management) remains the caller's job.

## Decision outcome

One givn change ships both the `run:` declaration capability and the `givn run`
executor. givn is the executor of first resort for the declared start command;
kiekste and other automation are callers of that machine contract, not owners
of execution. The contract's observable behaviour is owned by the Gherkin
specs (`run-command`, `run-execution` capabilities); this ADR records only the
boundary decision and its rationale. Residual risks (unix-only signal
forwarding, ephemeral-port race, declaration spoofing) are recorded in
chapter 11.

### Positive consequences

- The declaration is validated by execution: a wrong start command fails
  observably at `givn run`, not only in review prose.
- Automation needs exactly one integration surface instead of its own process
  management.
- The anti-gaming stance stays honest: givn checks only what it can prove
  deterministically, and now also *demonstrates* the declaration by running it.

### Negative consequences

- givn's public contract grows: ready line, exit codes, and signal behaviour
  become compatibility obligations for future releases.
- Process supervision inside a single-binary CLI adds a platform constraint
  (signal forwarding is unix-only) that the declaration-only plan did not have.

### Confirmation

The @e2e ready-line and exit-code scenarios of the `run-execution` capability
(ready line prefix, caller-provided port, exit codes 0/1/2/130/143, forwarded
child code) and the R1–R10 `check_run` validation suite of the
`run-command` capability are the fitness evidence: `./run-tests.sh` and
`./run-tests.sh --e2e` stayed green (548 and 106 scenarios, zero failures)
before this record left `proposed` — accepted 2026-09-03.

## Pros and Cons of the Options

### Declaration-only; execution stays with the orchestrator

- Good, because givn's public surface stays small and no
  process-supervision code enters the single binary.
- Bad, because the declaration is unexercised text: nothing proves the
  command starts the product or honours the declared port fields.
- Bad, because every caller re-implements readiness polling, signal
  teardown, and exit-code interpretation — badly and differently.

### Declaration in givn; execution only elsewhere (companion tool)

- Good, because the executor could evolve on its own release cadence.
- Bad, because it contradicts the single-binary boundary (ADR-004) and
  splits one machine contract across two artifacts and release trains.
- Bad, because consumer projects gain an install prerequisite for no
  architectural gain.

### One tool declares and executes (chosen)

- Good, because execution validates the declaration: a wrong start command
  fails observably at `givn run`, not only in review prose.
- Good, because automation integrates with exactly one spec'd surface
  (ready line, exit codes, port hand-off).
- Bad, because ready line, exit codes, and signal behaviour become durable
  compatibility obligations (chapter 11, R-45).

## Links

- ADR-004 (single static binary) — the no-daemon boundary this decision keeps.
- ADR-015 (declare the test runner explicitly) — the analogous declaration
  this decision extends to the start command.
- ADR-028 (hooks and mandatory checks) — amended by this change: the
  run-declaration gate is a new non-deactivatable mandatory check.
- Arc42 chapters 3 (context), 6.15 (`givn run` runtime), 8.16 (run-declaration
  concepts), 11 (risks R-41..R-45).
