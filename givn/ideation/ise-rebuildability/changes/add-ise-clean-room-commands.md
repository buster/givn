# Change seed: add-ise-clean-room-commands

Status: proposed by add-ise-clean-room-commands

Surface note (2026-11-14): the operator renamed the subcommand and its
replacement flag after the design review. The delivered surface is
`givn ise replace` with exactly one of `--full` and repeated `--path`; a
`--path` value is a tracked file or a directory that expands to every tracked
file beneath it. The accepted decisions below keep their substance; the names
and the directory rule are updated in place.

- Use-case IDs: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`)
- Topic: `ise-rebuildability`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: 3; depends on `add-ise-addon-and-specification` for the
  declaration and `enforce-ise-e2e-boundary` for the frozen `e2e` identity
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

A developer can produce a clean-room trial from a recorded revision, on Git
and Fossil, and a verifier can confirm its frozen inputs. The command form is
`givn ise replace [--at <revision>] (--full | --path <path>...) --into <dir>
[--receipt <path>] [--dry-run]`; `--into` is required and names a plain
directory outside the checkout. The trial removes exactly the tracked files
named with `--path` (or every tracked file under every `source:` root under
`--full`), records the retained and `e2e` identities, and re-checks them per
path on request. A refusal writes nothing.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `prepare-an-ise-test` | `clean-room-preparation` | `givn ise replace (--full / --path <path>...) --into <dir>` reads the selected revision, partitions the tracked paths, performs the nine refusals before any write, writes the trial directory, re-reads every file against its revision blob, records the identities, and prints the ten fixed report lines plus the receipt line when a receipt is written. |
| `prepare-an-ise-test` | `trial-input-verification` | `givn ise verify --trial <receipt> --at <dir>` re-checks the retained and `e2e` identities against the written trial and the current `e2e` package and reports drift per path. |

## Accepted decisions

### Q3: `givn ise replace` and `givn ise verify`

- Context: The clean room needed a command surface. The alternative was
  check-only enforcement, which leaves materialization to each project's
  script.
- Options:
  1. **Check-only enforcement**: In practice: `givn check tasks` refuses an
     in-process `@e2e` suite, and the harness or developer runs
     `git worktree add` or `fossil open` and deletes the paths. Gain: no new
     CLI and no materialization contract. Cost: every project writes its own
     clean-room script; Givn never verifies that history was withheld or that
     the deletion was exact.
  2. **`givn ise replace` and `givn ise verify`**: In practice: one protocol
     for a local developer, `givn-harness`, and a Givn self-rebuild. Gain: one
     tested clean-room protocol on Git and Fossil, with the refusals and the
     record owned by Givn. Cost: a new CLI surface and new materialization
     failure modes (gitlinks, symlinks, executable bits, ignored files), and
     both backends must agree.
- Chosen: **`givn ise replace` and `givn ise verify`**, bound to `addons.ise`.
  Rejected: check-only enforcement because the clean-room guarantee would
  rest on each project's script.

### Q4 and settled constraint 1: Deletion is set arithmetic on declared paths

- Context: Q4 fixed the declaration and the `source:` roots; the 2026-11-14
  settled constraint fixed that deletion is set arithmetic, never language
  inference. The options below are the alternatives that decision rules out.
  The trial fixes the paths the declaration's roots bound.
- Options:
  1. **Declared paths plus listed unit tests**: In practice:
     `--path crates/passuff-tui/src/keys/parser.rs --path
     crates/passuff-tui/tests/keys.rs`. Gain: no inference; `Not replaced`
     counts the files the developer did not name. Cost: the developer must
     name every path, and a missed unit test remains in the trial.
  2. **Infer unit tests from stack markers**: In practice: delete
     `#[cfg(test)]` modules, `test_*.py`, and Maven test roots automatically.
     Gain: less naming. Cost: language knowledge enters deletion, which the
     settled constraints forbid; it misses variants and can delete a file the
     developer wanted kept.
  3. **Delete the component's whole directory**: In practice:
     `--path crates/passuff-tui/src/keys/`. Gain: one path and no
     inference. Cost: it deletes files the developer did not name; no `Not
     replaced` count exists because nothing in the directory is left.
- Chosen: **Declared paths plus listed unit tests**. Rejected: stack-marker
  inference because deletion must not read language markers; whole-directory
  deletion because it removes unnamed files. The delivered surface adds one
  settled form: a named `--path` directory expands to every tracked file
  beneath it at the revision, so the developer can name a directory
  explicitly. That is not inference (the directory is an input), and the
  retained-role check runs over the expansion; `--full` names every tracked
  file under every `source:` root.

### Q5 and settled constraint 4: Only the E2E package is frozen

- Context: Q5 fixed the package boundary; the 2026-11-14 settled constraint
  fixed that only the End-to-end (E2E) test package is acceptance authority.
  The options below are the alternatives that decision rules out. The frozen
  set is the `@e2e` scenarios selected by tag plus every file of the package.
- Options:
  1. **Only the E2E package**: In practice: freeze the `@e2e` scenarios
     selected by tag plus every file of the `e2e` package. Gain: matches Ise
     Pattern 03; component step definitions stay implementation; a new
     feature's step bindings are expected. Cost: a test defect found mid-trial
     stops the trial.
  2. **Freeze the whole test target**: In practice: every step definition and
     harness file is frozen. Gain: one simpler digest. Cost: it freezes
     implementation; a new feature's step bindings are blocked; it contradicts
     Ise's discarded list.
  3. **Freeze nothing and rely on review**: In practice: the verifier reads
     the suite and judges it. Gain: no digest machinery. Cost: no tamper
     evidence; a weakened assertion passes.
- Chosen: **Only the E2E package**. Rejected: the whole target because it
  freezes implementation; review-only because it cannot prove the frozen
  inputs.

### Q13 and settled model: `Ise Test` in a plain directory

- Context: Q13 named the execution `Ise Test`; the settled model fixes a
  plain directory because the source's "throwaway branch" carries history and
  a Fossil checkout carries the repository database. The options below are
  the boundary alternatives.
- Options:
  1. **`Ise Test` in a plain directory**: In practice:
     `--into /tmp/ise-passuff`. Gain: no history and no Fossil repository
     database reach the builder; one noun covers both modes. Cost: the trial
     is not a version-control checkout, so normal history commands are absent
     by design.
  2. **`Ise Test` on a throwaway branch**: In practice: `git worktree add`
     or a Fossil branch. Gain: familiar version-control commands. Cost: the
     branch carries history, and the Fossil checkout carries the repository
     database, so the builder can read the old Source code.
  3. **No named execution**: In practice: the verifier runs the frozen suite
     directly. Gain: no container and no trial record. Cost: no frozen inputs,
     no record, and no way to distinguish a rebuild from an ordinary run.
- Chosen: **`Ise Test` in a plain directory**. Rejected: the throwaway branch
  because it leaks history; no named execution because it loses the frozen
  inputs.

### Q5 and Q13: Freeze after qualification

- Context: A new E2E scenario often needs correction before it is a valid
  acceptance test. The freeze must not preserve a false assumption, and it
  must not let the builder edit the test after seeing the result.
- Options:
  1. **Qualify, then freeze**: In practice: observe the new scenario failing
     against the pre-change implementation for the intended reason, then add it
     to the frozen set. Gain: the frozen test asserts the intended behavior.
     Cost: one qualification run before preparation.
  2. **Freeze first and correct during the trial**: In practice: edit the E2E
     test after a failure. Gain: no pre-trial step. Cost: the builder's write
     authority covers the test; a weakened assertion can pass; the freeze is
     not a freeze.
  3. **Never add a new E2E scenario in a rebuilding change**: In practice:
     only existing scenarios enter the trial. Gain: the simplest freeze. Cost:
     new behavior cannot be specified and the Specification cannot grow.
- Chosen: **Qualify, then freeze** (settled constraint 6, 2026-11-14; rests
  on Q5 and Q13). A mid-trial `E2E test defect` stops the trial, the test is
  corrected, and the trial restarts. Rejected: freezing first because it lets
  the builder edit acceptance; refusing new scenarios because it freezes the
  Specification.

### `S4`: `--into` is required and lies outside the checkout

- Context: `plan.md` `S4` settles the command surface (delivered as
  `givn ise replace [--at <revision>] (--full | --path <path>...) --into <dir>
  [--receipt <path>] [--dry-run]`; the earlier draft wrote the retired
  subcommand and flag names). The target must lie outside the checkout
  and must not already exist, so the parser needs a rule for an omitted flag.
  Earlier records wrote `--into` in brackets and left the omission open.
- Options:
  1. **Require `--into`**: In practice: omission refuses by name before any
     write. Gain: no accidental path and no cleanup surprise, and the
     outside-checkout rule has a named target to check. Cost: every
     invocation names the target.
  2. **Default to a fresh directory under the system temp**: In practice:
     `/tmp/givn-ise-<revision>`. Gain: a one-flag invocation. Cost: collision
     and cleanup rules, and a predictable path is visible to other local
     users.
  3. **Default to a sibling directory outside the checkout**: In practice:
     `../<project>-ise-<revision>`. Gain: the trial is visible next to the
     project. Cost: the sibling can sit inside another repository or inside a
     parent repository, so the outside-checkout and nested-repository refusals
     must handle it.
- Chosen: **Require `--into`** (`plan.md` `S4`, 2026-11-14). The target is a
  caller decision: no default can guarantee a directory outside the checkout,
  and the topic records no default. Rejected: the temp default because it adds
  collision and cleanup rules; the sibling default because it can sit inside
  another repository.

## Open questions

None. The `--into` question closed on 2026-11-14: `plan.md` `S4` makes
`--into` required and outside the checkout, and the decision above records it.
