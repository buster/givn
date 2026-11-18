# Instruction: ise

The `ise` addon applies [The Ise Principle](https://iseprinciple.org) to a
Givn project: a System whose `Specification` stays declared while the `Source
code` behind it is replaceable. The site is framework-agnostic and does not
depend on Givn. This instruction names the obligations every ise change
carries.

**Opt-in.** This instruction applies while `addons.ise` is enabled in
`givn/config.yaml`. `givn instructions ise` refuses by name while the addon is
off.

**Vocabulary.** This instruction uses `Component`, `Component Interface`,
contract, `Specification`, `Source code`, and `System` because `ise` requires
`architecture_principles`; that instruction defines them. It defines the
rebuild-specific terms it uses:

- **spec copy**: the plain directory outside the project that
  `givn ise rebuild-all` or `givn ise rebuild-component` writes from a selected
  revision.
- **spec-copy manifest**: the generated `ise-manifest/v1` file recording that
  copy's revision, inventories, counts, checksums, and location; it is
  ephemeral evidence, never project content.
- **checksum**: the SHA-256 of one path's identity (path, entry kind,
  executable marker, and content bytes) or of a sorted `path, checksum` rollup.
- **Rebuild impact record**: the trial review's table of files added, modified,
  deleted, moved, or renamed outside the declared Specification and the
  replaced unit, plus consolidations, each with its reason.
- **Replaced unit**: the Source code span one rebuild replaces.

## The Ise rebuild decision

An `ise`-enabled change records exactly one **Ise rebuild decision** as a
`Decides D<n>` entry in the design's `## Decisions` ledger. The decision is one
of:

| Decision | Meaning |
|---|---|
| `No rebuild` | The change keeps working with the existing Source code. No `Rebuilt code` line is recorded. |
| `Single-Component Rebuild` | One Component's Specification is updated as the active change's delta, its Source code and coupled unit tests are replaced, and its Component Interface keeps its consumers. |
| `Whole-system rebuild` | Every declared `source:` entry's Source code is replaced behind the System's external interfaces; persisted data and external contracts stay. |

A rebuilding change adds a plain **`Rebuilt code`** line with one value:
`discard` or `ship`. `ship` adopts the verified replacement into the change;
`discard` does not adopt it. A `No rebuild` change records no `Rebuilt code`
line.

The entry names the affected consumers, the persisted state the rebuild must
carry, and the rebuild boundary. It is the existing `Decides D<n>` entry; it
adds no mandatory design field.

Prose writes `Ise rebuild decision` in full. A bare `decision` names no domain
choice.

## Two rebuild practices

| Practice | Where the rebuild happens | What happens to the result | When to use it |
|---|---|---|---|
| Working-tree ship | The current project working tree | The replacement is committed as part of the change | `Single-Component Rebuild` with `Rebuilt code: ship` |
| Spec copy | A plain directory outside the project, written from a selected revision | The result stays outside the project; nothing is promoted | `Rebuilt code: discard`, or a copy-based Ise Test |

### Single-Component Rebuild with Rebuilt code: ship

The working-tree ship is the straightforward practice. It does not require a
spec copy, a candidate revision, an early archive merge, a generated spec-copy
manifest, a repository view that hides history, or a sandbox.

1. **Write the active Specification.** The updated use-case text and scenarios
   live in `givn/changes/<id>/specs/**`. Unchanged permanent rules stay in
   `givn/specs/**`; the active delta is what change-scoped verification runs.
2. **Keep the previous implementation in history.** Record the Specification
   delta while the previous Component's Source code is still available in
   revision history. A feature-only revision is not completed implementation
   evidence.
3. **Remove the old implementation in the working tree.** Delete that
   Component's Source code and coupled unit tests from the current project
   working tree. Keep its Component Interface, every other Component, and the
   acceptance suite. Do not remove unrelated Source code.
4. **Rebuild in place.** The coding agent rebuilds the Component in the working
   tree. It is instructed not to consult the previous implementation in
   accessible history.
5. **Verify, commit, archive.** Verify against the active delta and the
   applicable End-to-end (E2E) scenarios without changing the acceptance
   target. Commit the replacement as part of the change. `givn check tasks`
   verifies the change delta; `givn check review` checks the projected merge;
   archive merges the delta once, after the rebuilt code passes, and runs its
   normal merged-suite verification.

**Limitation.** A passing verification proves the replacement's tested
behaviour. It does not prove that the Specification alone was sufficient,
because the previous implementation stays accessible in history. Never report
the working-tree path as a clean-room result.

If the Specification or the E2E suite changes after the rebuild starts, record
a new frozen revision and rerun the relevant verification. Never move the
acceptance target to make the rebuilt code pass.

### Rebuilt code: discard

A discard rebuild does not adopt the rebuilt Source code into the project. Use
`givn ise rebuild-component` or `givn ise rebuild-all` to prepare a spec copy
from a selected revision, rebuild inside it, and record what the trial found.
The rebuilt work stays in the copy, and the trial review records that no
rebuilt file was promoted. The spec copy is never a branch or a worktree.

## Commands

The commands serve the spec-copy practice. Each one writes or reads the spec
copy; the rebuild happens in that copy, by the agent, and never in the command.
The spec-copy commands prepare an external copy; they never edit the project
working tree.

| Command | What it does |
|---|---|
| `givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Writes the spec copy in which every declared `source:` entry is removed. |
| `givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Writes the spec copy in which one Component's Source code is removed; neighbouring Components and their Component Interfaces stay. |
| `givn ise compare --manifest <file> --copy <dir>` | Reports every difference between the spec-copy manifest and the Specification files in the spec copy, the Specification files in the project working tree, and the End-to-end (E2E) tests. |

A refusal prints `ise rebuild-all:`, `ise rebuild-component:`, or
`ise compare:` to match the command that refused.

## The spec copy rule

The spec copy is written with its Specification files in the spec copy: the
matched files of the four content categories and the Specification Manifest. During a
rebuild the agent adds files, and those are the rebuilt Source code. The agent
may edit Source code kept because it is the System's Source code, but it does
not delete any file the copy was written with. The agent does not modify any
Specification file in the spec copy; adding a file under a declared manifest
entry is a Specification change, not the Source code, and is refused.

An `e2e` match is part of the Specification but never enters the spec copy.

The `Specification Manifest` is `givn/specification.yaml`, not the generated
spec-copy manifest. Its five categories are `behavior`, `structure`,
`contracts`, `limits`, and `e2e`. Each category holds manifest entries (a
literal path or glob); an entry matches files. An `e2e` entry's matched files
are read from the project working tree and never copied.

## The clean-room boundary

The builder reads no Source code outside the spec copy and the declared
inputs. Before a copy-based rebuild, scan the local package registry's source,
cache, and git stores for a published copy of the System, and quarantine every
hit outside the builder's reach; a store that does not exist has no hit. Record
the scan on its own `Clean-room scan` line in the trial review. A published
copy the builder can read is not a clean-room rebuild. A working-tree ship
makes no clean-room claim.

## Obligations

- **Specification completeness** — every behaviour, contract, limit,
  acceptance test, and file the rebuild needs is declared in
  `givn/specification.yaml` and reachable from the matched files. A file the
  rebuild needs that is not Source code belongs under the category that owns
  its content class: a build configuration file and a lockfile under
  `contracts`, a toolchain pin under `limits`, a structural template under
  `structure`, a schema or migration asset under `contracts`, and served
  instruction or skill text under `behavior`. The frozen End-to-end (E2E)
  tests fix the bytes they assert; a structural template or a schema asset
  fixes its content through the contract that names its shape. A file the
  rebuild needs that the Specification Manifest leaves undeclared is a
  `specification defect`. An asset the rebuild generates inside the replaced
  unit whose bytes no frozen test fixes is an `acceptable undocumented change`,
  not a rebuild impact row. The installed toolchain and dependency cache are
  supplied, never declared files: the lockfile declares the resolved
  dependencies, the toolchain pin declares the toolchain, the trial records how
  both were supplied, and a locked dependency the cache cannot resolve blocks
  the rebuild; the blocked rebuild is a setup failure, not a `Spec gap`, and it
  is not a `specification defect`. A contract the frozen tests rely on
  enumerates every value it fixes. A declared manifest entry must survive
  `givn archive`: a change's artifacts move from `givn/changes/<id>/` to
  `givn/archive/<id>/`, so a category names the archived path or another
  durable path, never an active change's artifact. `givn archive` refuses such
  an entry before moving the change. The spec copy always carries the
  Specification Manifest itself: `givn/specification.yaml` is an implicit
  Specification entry, not a sixth category, so a project need not list it and
  the agent reads the declared categories from the copy the selected revision
  records.
- **Two-layer test boundary** — the `@e2e` scenarios stay in the Specification
  under the `behavior` category with their exact titles, and run from the
  declared `e2e` package, which holds the runner and every `@e2e` step
  definition and survives a rebuild. Component scenarios stay in the
  Specification unchanged; only their step definitions and fakes are
  implementation and may be rewritten inside the Component boundary.
- **Persisted-state continuity** — a replacement preserves persisted state and
  public contracts; a required manual migration is a blocked rebuild.
- **Ise rebuild decision** — the change records one decision as a
  `Decides D<n>` entry, with the `Rebuilt code` line when it rebuilds.
- **Implementation freedom** — Source code layout may change while declared
  behaviour, Component Interfaces, persisted-state obligations, and technical
  limits stay satisfied. This is a rule, not another record; do not reproduce
  undocumented internal design merely because it existed previously.
- **Spec gap classification** — a corrective change records exactly one
  `Spec gap` outcome before any fix: `E2E test defect`, `implementation defect`,
  `specification defect`, or `acceptable undocumented change`.

## Stack mechanics

`givn instructions steps` maps the runner to its stack instruction and
carries the universal rules; `steps-rust` is one worked id. Request the stack
instruction `steps` names for the runner in hand.

An ise change carries three records:

| Record | Home | Content |
|---|---|---|
| `Ise rebuild decision` | the design's `## Decisions` ledger | one `Decides D<n>` entry naming `No rebuild`, `Single-Component Rebuild`, or `Whole-system rebuild`; a rebuilding entry adds the plain `Rebuilt code` line (`discard` or `ship`), the affected consumers, the persisted state, and the rebuild boundary |
| `Spec gap` | the corrective change's proposal | exactly one outcome and the artifact that is wrong, not who found it |
| `Rebuild impact record` | the trial review's evidence directory | every file added, modified, deleted, moved, or renamed outside the declared Specification and outside the replaced unit, and every consolidation (a merge, split, move, or rename of rebuilt paths), each with its reason; the trial review records it, and the clean-room scan is a separate line |

A rebuild with no outside effect records an empty table. The rebuild impact
record is one review question, never a gate, and it is not a behaviour diff.

Only `specification defect` is a gap in the Specification; an
`E2E test defect` is a gap in the test, an `implementation defect` is a gap in
the build, and an `acceptable undocumented change` is no gap at all.

The rules for writing each record are served by
`givn instructions design --change <id>` and
`givn instructions proposal --change <id>`; this instruction names the
obligation, not a second copy of the rules.
