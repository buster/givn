# Ise addon vocabulary review

Status: read-only audit at revision `a7e1dfe6a0574ce8e7ec6821b48cef2bed26b498` on `ise/implementation`, 2026-11-16.

Scope: the served `ise` instruction, the `givn ise replace` and `givn ise verify` commands, their report output, the `ise-trial/v1` record, the `givn addons enable ise` gate, and the glossary rows the addon owns or consumes. `givn/ideation/ise-rebuildability/guidance-improvements.md` is a proposed instruction text, not a current surface, so its terms are not counted here.

Method: read `src/main.rs` (the clap definitions), `src/cli/ise.rs` (the report), `src/ise/mod.rs`, `src/ise/partition.rs`, `src/ise/record.rs`, `src/ise/verify.rs` (the rules and refusals), `src/scm.rs` (the selector refusals), `src/cli/addons.rs` and `src/cli/instructions.rs` (the add-on gates), `assets/instructions/ise.md`, `docs/reference.md`, and `docs/glossary.md`. Counts are case-insensitive whole-phrase matches, with `_` treated as a word separator, so `retained_files` counts as `retained`.

## Inventory

### Commands and flags

| Command | Flags | Behaviour |
| --- | --- | --- |
| `givn ise replace` | `--at <revision>` (default `HEAD`), `--full`, `--path <path>` (repeatable), `--into <dir>` (required), `--receipt <path>`, `--dry-run`, `-h`/`--help` | Writes one recorded revision into a new plain directory outside the checkout. `--full` and `--path` form a required, mutually exclusive group. `--full` selects every tracked file under the declared `source:` roots; `--path` selects a tracked file or every tracked file under a directory. `--dry-run` performs every read and refusal and writes nothing. `--receipt` writes the YAML record; without it the record is printed to stdout. |
| `givn ise verify` | `--trial <path>` (required), `--at <dir>` (required), `-h`/`--help` | Reads the YAML record named by `--trial` and re-checks the trial directory named by `--at`. `--trial` is the record; `--at` is the directory. |
| `givn addons enable ise` | `<name>` (positional), `--operator`, `--reason <text>`, `--change <id>`, `-h`/`--help` | Writes `addons.ise: true`, scaffolds `givn/specification.yaml` when absent, and resyncs generated guidance. `--operator` requires an interactive terminal and a non-empty `--reason`; `--change` selects the active change whose lock the override updates. |
| `givn addons disable ise` | `<name>` (positional), `--operator`, `--reason <text>`, `--change <id>`, `-h`/`--help` | Writes `addons.ise: false` and removes the workflow effect. Existing project files are not deleted. The same operator rules apply. |
| `givn instructions ise` | `<id>` (positional), `--change <id>`, `--json`, `-h`/`--help` | Serves the `ise` instruction only while `addons.ise` is effective. |

### Report lines

`givn ise replace` prints these lines in this order:

| # | Exact label | Conditional |
| --- | --- | --- |
| 1 | `Backend: <name>` | No |
| 2 | `Revision: <revision>` | No |
| 3 | `Retained: <n>` | No |
| 4 | `Kept source: <n>` | No |
| 5 | `Replaced: <n>` | No |
| 6 | `Outside the declaration: <n>` | No |
| 7 | `Not replaced: <n>` | No |
| 8 | `Retained digest: <12 hex>…` | No |
| 9 | `E2E digest: <12 hex>…` | No |
| 10 | `Frozen package: <roles joined by ", ">, not copied (<n> file or files)` | Only when the frozen set is non-empty. `file` when `n` is 1, `files` otherwise. |
| 11 | `Written path: <path>` | No |
| 12 | `Receipt path: <path>` | Only when `--receipt` is given. |

After those lines, `--dry-run` adds:

```text
Would write:
  <path>
```

The paths are sorted and indented by two spaces. Without `--receipt`, the YAML record follows the report on stdout. `Written path` and `Receipt path` are also printed in a dry run, where neither the directory nor the record is created; see F4 and F5.

`givn ise verify` prints one of three shapes:

| Outcome | Stream | Lines |
| --- | --- | --- |
| Clean | stdout | `Trial: <dir>`, `Retained: clean`, `E2E: clean` |
| Drift | stderr | `Trial: <dir>`, one `Retained drift: <path> (<reason>)` or `E2E drift: <path> (<reason>)` per path, then `Verification failed: <n> drifting path(s)` |
| Refusal | stderr | `ise verify: <reason>` |

The six drift reasons are `the path is missing from the trial`, `the trial differs from the record`, `the path is missing from the checkout's e2e package`, `the checkout's e2e package gained this path`, `the checkout differs from the record`, and `the selected @e2e scenarios differ in the checkout`.

### Refusals

Named rule refusals exit `1` and print `<command>: <reason>`; clap usage errors (a missing `--into`, both `--full` and `--path`, an omitted `--trial` or `--at`) exit `2`. The backend selector's three messages are shared by both commands.

`givn ise replace`:

| # | Verbatim reason | Rule |
| --- | --- | --- |
| 1 | "read the effective manifest at '<root>'" | The effective manifest must load. |
| 2 | "ise is not enabled; run `givn addons enable ise` first" | `addons.ise` must be effective. |
| 3 | "architecture_principles is not enabled; run `givn addons enable architecture_principles` first" | The prerequisite add-on must be effective. |
| 4 | "the checkout is dirty at <paths>; commit or restore these paths first" | No working-tree change may exist before a trial is written. |
| 5 | "revision '<value>' does not exist in this repository" | The selected revision must exist. |
| 6 | "the revision '<revision>' does not record 'givn/specification.yaml'; a clean-room trial always carries the declaration" | The selected revision must record the declaration. |
| 7 | "givn/specification.yaml: <loader reason>" | The recorded declaration must parse under the strict loader. |
| 8 | "the declared <role> entry '<pattern>' matches nothing at revision '<revision>'" | Every declared retained entry must match a tracked path or a directory. |
| 9 | "'<path>' is outside the project; a replacement path is a project-relative tracked file or directory" | A `--path` value must normalize inside the project. |
| 10 | "'<path>' is retained by the Specification declaration at revision '<revision>'; a retained path is never replaced" | The declaration is never a replacement candidate. |
| 11 | "'<path>' is retained by the <role> role at revision '<revision>'; a retained path is never replaced" | A retained role match is never replaced. |
| 12 | "'<path>' is outside every declared source root at revision '<revision>'" | A replacement candidate must lie under a `source:` root. |
| 13 | "the target '<into>' is inside the checkout '<checkout>'" | The trial directory must lie outside the checkout. |
| 14 | "the target '<into>' already exists; a trial is written only into a new path" | The trial directory must not exist. |
| 15 | "the receipt '<receipt>' is inside the checkout '<checkout>'" | The record must lie outside the checkout. |
| 16 | "the receipt '<receipt>' already exists; a record is never overwritten" | The record is create-new. |
| 17 | "the receipt '<receipt>' lies inside the trial '<trial>'" | The record must not be inside the trial. |
| 18 | "the receipt '<receipt>' is an ancestor of the trial '<trial>'" | The record must not contain the trial. |
| 19 | "'<path>' is a version-control marker; a clean-room trial copies no repository metadata" | No `.git`, `.fslckout`, or `_FOSSIL_` component in the copy set. |
| 20 | "'<path>' is a repository link; a clean-room trial copies no repository link" | No gitlink or submodule in the copy set. |
| 21 | "the written trial differs from revision '<revision>' at <paths>" | The re-read must match every written byte and mode. The trial is removed. |
| 22 | "the revision does not list '<path>', so the trial cannot write it" | Every copy path must exist at the revision. |
| 23 | "the revision returned no content for '<path>'" | Every copy path must have a blob. |
| 24 | "'<path>' is a symlink; this platform cannot write one" | Non-Unix platforms refuse a symlink entry. |
| 25 | "the path '<path>' has no existing ancestor: <error>" | A write target must resolve through an existing ancestor. |
| 26 | "write the trial record '<path>'" | The record write can fail after the trial is written; the trial is removed, then the error is returned. |

The backend selector adds:

| Verbatim reason | Rule |
| --- | --- |
| "the project root '<root>' must be the worktree top level; it is nested inside '<top>'" | The project root must be the Git worktree top level. |
| "ambiguous version control at '<root>': both a Git worktree marker and a Fossil checkout marker (<marker>) are present" | One backend only. |
| "no Git worktree or Fossil checkout found at '<root>'; a version-controlled project is required" | A backend must be present. |

`givn ise verify`:

| # | Verbatim reason | Rule |
| --- | --- | --- |
| 1 | "read the effective manifest at '<root>'" | The effective manifest must load. |
| 2 | "ise is not enabled; run `givn addons enable ise` first" | `addons.ise` must be effective. |
| 3 | "architecture_principles is not enabled; run `givn addons enable architecture_principles` first" | The prerequisite add-on must be effective. |
| 4 | "read the trial record '<path>'" | The record file must be readable and parse as YAML. |
| 5 | "the record '<path>' has the schema <n>; a trial record is schema 1" | `schema` must be `1`. |
| 6 | "the record '<path>' has the content scope '<scope>'; expected 'ise-trial/v1'" | `content_scope` must be `ise-trial/v1`. |
| 7 | "the record '<path>' has no 64-hex replaced_digest; the record is no trial evidence" | `replaced_digest` must be 64 hex characters. |
| 8 | "the trial directory '<dir>' is missing" | `--at` must exist. |
| 9 | "givn/specification.yaml is missing; the clean-room trial requires a declared Specification" | The checkout must carry a declaration. |
| 10 | "the record '<path>' names the written path '<written>' but the trial is '<trial>'" | `written_path` must resolve to `--at`. |
| 11 | "the record '<path>' records the backend '<recorded>' but the selected backend is '<selected>'" | `backend` must equal the selected backend. |
| 12 | "the record '<path>' lies inside the trial '<trial>'; the trial's builder must not own the record" | The record must lie outside the trial. |
| 13 | "the trial '<dir>' holds <n> path(s) the frozen e2e package covers; the builder must not receive the acceptance suite: <paths>" | A record whose `excluded_roles` names `e2e` must not be accompanied by a trial holding a frozen path. It names at most 20 paths and appends ` (and <n> more)`. |

`givn addons enable ise` and `givn addons disable ise`:

| Command | Verbatim reason | Rule |
| --- | --- | --- |
| `enable ise` | "addon 'ise' requires 'architecture_principles'; enable it first" | The prerequisite add-on must be on. |
| `disable architecture_principles` | "addon 'architecture_principles' is required by 'ise'; disable 'ise' first" | A dependent add-on must be off. |
| either, locked change | "add-on configuration is operator-owned and is locked by active change(s): <ids>. `givn addons` cannot change `<addon>` during a change. Install the missing dependency in an isolated environment or report an environment failure; do not disable its gate. An operator can override on an interactive terminal with `--operator --reason "<reason>"`." | Add-on configuration is operator-owned while a change lock exists. |
| either, no terminal | "the add-on operator override requires an interactive terminal; no configuration was changed" | The override needs a TTY. |
| either, empty reason | "the add-on operator override requires a non-empty --reason; no configuration was changed" | The override needs a reason. |
| either, `--change` with no lock | "change '<id>' has no add-on lock to override" | The named change must have a lock. |
| either, several locks and no `--change` | "several locked changes are active; select one with --change <id>: <ids>" | One lock must be selected. |

`givn instructions ise`:

| Verbatim reason | Rule |
| --- | --- |
| "instruction 'ise' requires the 'ise' addon; enable it with `givn addons enable ise`" | The instruction is add-on bound. |

I/O context wrappers that only repeat a path are omitted from these tables (`read the project tree at '<root>'`, `read the trial directory '<dir>'`, `digest the selected @e2e scenarios in '<path>'`). Whitespace in the locked-change message is normalized to single spaces; its words are exact.

### `ise-trial/v1` record

The record is strict: `deny_unknown_fields`, all fields required except `excluded_roles`, which defaults to an empty list. Field order is serialization order.

| Field | Type | Value and meaning |
| --- | --- | --- |
| `schema` | integer | `1`. |
| `content_scope` | string | `ise-trial/v1`. |
| `backend` | string | `git` or `fossil`. |
| `revision` | string | The resolved revision the trial was read from. |
| `named_paths` | list of strings | Normalized, sorted, deduplicated `--path` values. Empty under `--full`. |
| `replaced_paths` | list of strings | The sorted tracked paths removed from the trial. |
| `counts.retained` | integer | Number of retained paths. Equals the `retained_files` entry count. |
| `counts.kept_source` | integer | Number of non-retained, non-replaced tracked paths under a `source:` root. |
| `counts.replaced` | integer | Number of replaced paths. Equals the `replaced_paths` length. |
| `counts.outside_declaration` | integer | Number of non-retained, non-replaced tracked paths outside every `source:` root. |
| `counts.not_replaced` | integer | Direct tracked files in the replaced files' parent directories that were not replaced. |
| `excluded_roles` | list of strings | `[e2e]` when the frozen set is non-empty, otherwise empty. An absent field means an older record whose frozen files are retained entries. |
| `retained_digest` | string | Rollup over `retained_files`. |
| `e2e_digest` | string | Rollup over `e2e_files` and `e2e_scenarios`. |
| `replaced_digest` | string | Rollup over the replaced paths as the trial was written. `verify` checks only that it is 64 hex characters. |
| `retained_files` | map from path to digest | One identity per retained path. Always includes `givn/specification.yaml`. Never includes a frozen path. |
| `e2e_files` | map from path to digest | One identity per file of the resolved `e2e` package. Includes `givn/specification.yaml` when the `e2e` role resolves over it. |
| `e2e_scenarios` | map from feature path to digest | One digest per `behavior` feature file that carries selected `@e2e` scenarios. |
| `written_path` | string | The resolved absolute trial directory. |
| `givn_version` | string | The running binary's package version. |

### Glossary rows

`docs/glossary.md` is the declared vocabulary authority. This table splits the rows the add-on introduced from the rows it consumes.

| Term | Line | Introduced by this add-on? | Definition and anti-terms, shortened |
| --- | --- | --- | --- |
| `Change strategy` | 41 | Yes | The one `Decides D<n>` entry naming incremental modification, Constant Ise, or whole-product rebuild, with the Ise Test mode when it rebuilds. Anti-term: Rebuild scope. |
| `Constant Ise` | 42 | Yes | Ise Pattern 01: delete one component's implementation and white-box tests inside its boundary, then rebuild from the updated Specification. Not Full rebuild; not incremental modification. |
| `Ise Test` | 45 | Yes | One clean-room rebuild execution that freezes the End-to-end (E2E) tests, hands the Specification to the agent, builds the replacement, and records whether the spec passed. Anti-terms: Reconstruction trial, Ise Test result. |
| `End-to-end (E2E) test` | 72 | Yes, the row was added with the add-on; the concept predates the row | A black-box acceptance test through real external interfaces that survives a rebuild unchanged. Not Component test. Anti-terms: Black-box verifier; not Verification scope. |
| `E2E package` | 73 | No, added with ADR-079 | The package holding the E2E runner and every `@e2e` step definition, linking no production source. Anti-terms: test harness, Black-box verifier. |
| `E2E identity` | 74 | Yes, with the ise change-receipt work | The `e2e` package digests, runner-command digest, Givn version, and E2E scope outcome bound by an `ise`-effective change receipt. Anti-term: E2E digest. |
| `Trial record` | 75 | Yes | The YAML record `givn ise replace` writes or prints for one Ise Test, under `content_scope: ise-trial/v1`. Anti-terms: Ise Test result, receipt. |
| `Trial E2E digest` | 76 | Yes | The identity of the frozen E2E inputs: selected `@e2e` scenarios per `behavior` feature plus every file of the resolved `e2e` package. Printed as `E2E digest`. Anti-terms: frozen digest, e2e fingerprint. |
| `Trial directory` | 77 | Yes | The plain directory outside the checkout holding the non-frozen retained roles, the declaration, and the kept source with replacement paths removed. Not trial. Anti-terms: clean room, trial folder. |
| `Frozen package` | 78 | Yes | Every tracked file of the resolved `e2e` packages at the selected revision, except `givn/specification.yaml`. Recorded as `e2e_files` and the Trial E2E digest. Not End-to-end (E2E) test. Anti-terms: frozen digest, excluded files. |
| `Stack instruction` | 95 | No | A served `<id>-<stack>` instruction carrying one runner's mechanics. Not Stack recipe. |
| `Source code` | 147 | Yes | The replaceable application code and internal structure behind the public interfaces. Not Specification. Anti-term: Production source. |
| `Spec gap` | 148 | Yes | The classification record of one observed difference between the frozen E2E result and the declared Specification, holding exactly one outcome. Not Disposition. Anti-terms: Mismatch triage, Coverage gap. |
| `Specification` | 149 | Yes | The declared durable authoring input a rebuild reads: behavior, structure, contracts, limits, and black-box E2E tests. Not Source code. Anti-terms: Replacement package, Permanent specification corpus, Product specification. |
| `Specification declaration` | 150 | Yes | The tracked `givn/specification.yaml` naming five retained roles and the replaceable source roots. Not Run declaration; not Source root. Anti-terms: specification manifest, specification index. |
| `Retained path` | 151 | Yes | A path matched by one of the five retained roles, or `givn/specification.yaml` itself. Not Source root. |
| `Content path` | 152 | Yes | A path the selected backend reports: tracked plus untracked, non-ignored. Read by the archive guard and `givn ise verify`. Not working-tree path. |
| `Opt-in addon` | 205 | Yes | An addon whose declared default is `false`, requiring explicit enablement. Not Addon default. Anti-terms: Default-off addon, Experimental addon, opt-in default. |
| `SCM backend` | 218 | No | The concrete version-control system: Git or Fossil. Anti-terms: provider, driver. |
| `Revision` | 219 | No | One immutable recorded state: a Git commit or a Fossil check-in. |
| `Checkout` | 220 | No | The working directory as the active backend sees it. |
| `Clean state` | 221 | No | The backend reports no changes outside the resolved evidence paths. |

`Component`, `Port`, and `contract` are not in `docs/glossary.md`. The served `ise` instruction says `architecture_principles` defines them; `assets/instructions/architecture-principles.md` defines lowercase `contract`, `component`, `port`, and `adapter` in its terminology paragraph and its "Components, ports, and adapters" section. The `ise` instruction capitalizes `Component` and `Port`, but the source instruction does not.

### Term sources

| Source | Terms |
| --- | --- |
| Ise Principle | `Ise Test`, `Constant Ise`, `Dry-Run Ise`, `Full replacement`, `clean room` (the guarantee sense) |
| Givn-wide terms the add-on depends on, whose rows came from ise-topic changes | `End-to-end (E2E) test` (the concept predates the add-on), `E2E package` (added with ADR-079), `Stack instruction` (added by the stack-instruction split) |
| New to this add-on | `Change strategy`, `Spec gap`, `Specification`, `Specification declaration`, `Source code`, `Retained path`, `Content path`, `Trial record`, `Trial E2E digest`, `Trial directory`, `Frozen package`, `Opt-in addon`, `E2E identity` |
| Defined by `architecture-principles`, absent from the glossary | `component`, `port`, `contract`, `adapter` |

### Term counts

Counting rule: case-insensitive whole phrase, `_` as a separator. The served instruction is `givn instructions ise` output without the appended `Skill to load now` line; its text matches `assets/instructions/ise.md`. "Command table" is `docs/reference.md` lines 33 and 34. "Clean-room section" is `docs/reference.md` lines 365 to 546. "Addons section" is lines 101 to 176 and is context, not one of the two command sections.

| Term | Served `ise` instruction | Command table | Clean-room section | Addons section |
| --- | ---: | ---: | ---: | ---: |
| trial | 1 | 3 | 33 | 0 |
| declaration | 2 | 0 | 18 | 5 |
| retained | 3 | 1 | 23 | 0 |
| replaced | 0 | 0 | 9 | 0 |
| kept source | 0 | 0 | 1 | 0 |
| outside the declaration | 0 | 0 | 1 | 0 |
| frozen package | 0 | 0 | 3 | 0 |
| excluded | 0 | 0 | 6 | 0 |
| scope | 0 | 0 | 2 | 3 |
| mode | 2 | 0 | 0 | 1 |

Related forms, not in the requested list, show the split more sharply: the instruction uses `rebuild` 6 times, `replaceable` once, and `replacement` once. The clean-room section uses `record` 23 times, `receipt` 7 times, `written` 6 times, `removed` once, and `trial record` 0 times.

### `Spec gap`

There is no machine schema for `Spec gap`. It is prose in a corrective change's proposal. The served instructions name three parts:

| Part | Value |
| --- | --- |
| Home | The corrective change's proposal. |
| Outcome | Exactly one of the four values below, recorded before any fix. |
| Wrong artifact | The artifact that is wrong, not the person who found it. |

The four outcome values:

| Value | What is wrong | Route |
| --- | --- | --- |
| `E2E test defect` | The E2E test. | Correct the E2E test; the Ise Test stops and restarts. A corrective change edits the E2E suite. |
| `implementation defect` | The build. | Return to the agent inside the Ise Test. No corpus or suite edit. |
| `specification defect` | The Specification. | Open a corrective change that adds the missing rule, then rebuild. |
| `acceptable undocumented change` | Nothing that matters. | Record the difference in the corpus. |

Only `specification defect` is a gap in the Specification. `acceptable undocumented change` is no gap at all.

### First-time noun load

Counting rule: distinct noun phrases in the command help, the replace report, and the record, after collapsing case and plurals. Flag values and verbs are excluded. Composed phrases such as `E2E file` are not counted twice.

| Surface | Nouns | New |
| --- | --- | ---: |
| Command help | clean-room, Ise Test, trial, revision, tracked file, source root, path, directory, checkout, trial record, receipt, dry run | 12 |
| Report | backend, retained, kept source, replaced, outside the declaration, not replaced, digest, E2E, frozen package, written path | 10 |
| Record | schema, content scope, named path, count, excluded role, identity, rollup, scenario, version, Specification, role | 11 |
| Total | 33 distinct nouns | 33 |

Twelve of the 33 are placement or evidence labels a user must map before reading the output: `Retained`, `Kept source`, `Replaced`, `Outside the declaration`, `Not replaced`, `Frozen package`, `Retained digest`, `E2E digest`, `Backend`, `Written path`, `Receipt path`, and `excluded_roles`. The command help defines none of the five placement counts, `Written path`, `Receipt path`, or `Frozen package`.

### Surface consistency

| Concept | Command help | `docs/reference.md` | Served `ise` instruction | Report and record |
| --- | --- | --- | --- | --- |
| The materialized directory | `trial directory`, `trial` | `trial` (33), `trial directory` (1), `clean-room trial` | `trial` (1) | `Written path`, `Trial:`, `written_path` |
| The YAML record | `trial record`, `receipt` | `record` (23), `receipt` (7), `trial record` (0) | none | `Receipt path`, `content_scope` |
| The declaration | none | `declaration` (18), `givn/specification.yaml` | `declaration` (2) | `Outside the declaration` |
| The excluded e2e set | `frozen retained and e2e inputs` | `frozen package` (3), `frozen set` (2), `e2e package` (3), `excluded role` (2) | `frozen End-to-end (E2E) tests` | `Frozen package`, `excluded_roles`, `e2e_files` |
| The replacement set | `Replace every tracked file` | `Replaced`, `replaced_paths`, `removed` (1) | `replaceable`, `rebuild` (6) | `Replaced`, `replaced_paths` |
| The five placement counts | none | labels in the sample only | none | `Retained`, `Kept source`, `Replaced`, `Outside the declaration`, `Not replaced` |
| The trial outcome | none | `verification`, `drift` | `Ise Test`, `Spec gap` | `Verification failed`, `Retained: clean`, `E2E: clean` |
| The Ise Test mode | `--dry-run`, `--full` | `dry run` | `Ise Test mode` (2) | no field |

The eight concepts appear under more than 40 distinct label strings across the four surfaces. F1 to F16 state which of these labels are safe and which are not.

## Findings

Ordered by expected user confusion, highest first.

### F1. `--trial` names the record and `--at` names the trial directory

**Surfaces:** `givn ise verify --help` ("`--trial <TRIAL>  Trial record to read`" and "`--at <AT>  Trial directory the record must belong to`"), the command table (`--trial <record> --at <dir>`), the clean-room section ("confirms the trial directory exists"), and the error "the trial directory '<dir>' is missing".

**Why it costs the user:** the flag whose name says trial takes the YAML record, and the flag whose name says nothing takes the trial. A user who follows the flag names passes the directory to `--trial` and the record to `--at`, then receives two refusals whose nouns do not match the flags. `--at` is also the revision flag on `replace`, so the same flag changes meaning between two adjacent commands.

**Replacement:** `givn ise verify --record <path> --trial <dir>`. If the breaking change is refused, change the metavariables to `--trial <record>` and `--at <trial-directory>` and repeat the mapping in the command table, the help, and the refusal for a missing directory. Do not keep `--at` for two different types across `replace` and `verify`.

### F2. `replace`, `Replaced`, and `--full` promise product mutation; the command writes a trial copy

**Surfaces:** the command name, `givn ise replace --help` ("Replace every tracked file", "Replace a tracked file"), the report line `Replaced: <n>`, the record field `replaced_paths`, and the reference ("the sorted tracked set actually removed").

**Why it costs the user:** `replace --full` removes nothing from the checkout. It writes a new directory outside the checkout and omits the selected paths from that copy. A user who expects a destructive command either avoids it or runs it expecting the product to change. The report's `Replaced: 2` lands immediately after the write and reinforces the wrong reading. The plan and glossary use `removed` for the same set, so the code's own documents split between `replace` and `remove`.

**Replacement:** help first line: "Prepare a trial copy from one recorded revision; the checkout is never modified." Report: `Removed from trial: <n>`. Record field `removed_paths` in a v2 schema. If the command name stays, every surface must say "prepare" in the first sentence and `replace` only in the flags and the glossary's `replaceable` sense.

### F3. `verify` promises product verification; it checks input drift

**Surfaces:** the command name, `givn ise verify --help` ("Re-check a prepared trial's frozen retained and e2e inputs"), the clean-room section ("verification never writes to the trial"), the clean line `Retained: clean` / `E2E: clean`, and the drift line `Verification failed`.

**Why it costs the user:** the command runs no test, builds no product, and records no pass or fail. It re-reads the trial's retained bytes and the checkout's `e2e` package and selected scenarios. A reader who sees `Verification failed` can conclude the rebuild failed, when only an input changed. The Ise Test's own outcome is not represented.

**Replacement:** help: "Re-check a trial's retained bytes and frozen e2e inputs against its record." Clean lines: `Retained bytes: clean`, `Frozen e2e inputs: clean`. Drift line: `Input check failed: <n> drifting path(s)`. If the command name changes, `givn ise check-trial` names the mechanism without claiming the rebuild verdict.

### F4. One record wears four names, and the glossary rejects one of them

**Surfaces:** help (`trial record`, `--receipt`), report (`Receipt path`), reference (`record` 23 times, `receipt` 7 times, `trial record` 0 times), record (`content_scope: ise-trial/v1`), and glossary (`Trial record`; anti-term `receipt` because it collides with the change verification receipt).

**Why it costs the user:** the glossary says `receipt` is ambiguous and the command uses it as a flag and a report line. The canonical term `Trial record` never appears in the clean-room section that documents the artifact. A user searching for `trial record` in the reference finds nothing; a user searching for `receipt` finds the flag but the glossary tells them not to use the word. The file's own `content_scope` introduces a third noun, `ise-trial/v1`. In a dry run with `--receipt`, the report also prints `Receipt path:` although no file is written, and the `Would write` block lists only trial files. The label promises a file the run does not create.

**Replacement:** keep the flag `--receipt` only if the CLI is frozen, but print `Record path:` and document `Trial record` as the artifact. Rename the flag to `--record` in the same release as the other breaking changes. Suppress `Receipt path` in a dry run or print `Record path (not written):`. Never use `receipt` in prose for this artifact. The `content_scope` value is a format tag, not a noun the user should learn; see F12.

### F5. One directory wears five names and the glossary bans the shortest one

**Surfaces:** help (`Trial directory`, `trial`), report (`Written path`, `Trial:`), reference (`trial` 33 times, `trial directory` once), glossary (`Trial directory`; "not trial"; anti-term `clean room`), and the instruction ("The trial always carries the declaration").

**Why it costs the user:** `Written path` is a directory, not a file path. `Trial:` in the verify clean block names the directory, while `Trial` in `ise-trial/v1` and the instruction sometimes names the execution. The glossary explicitly says "Trial directory, not trial", then the command, the report, and the served instruction use `trial` for the directory. The user must infer which sense is active from the command. In a dry run, `Written path` names a directory that does not exist, and the `Would write` block lists files that were not written.

**Replacement:** report `Trial directory: <path>`; verify clean block `Trial directory: <path>`; record field `trial_directory` in v2. In prose, use `Trial directory` for the materialized copy and `Ise Test` for the execution. Print `Trial directory (would write):` in a dry run or suppress the line until a write happens. Keep `clean room` for the guarantee only, as the glossary says.

### F6. One exclusion wears nine names

**Surfaces:** report (`Frozen package: e2e, not copied`), record (`excluded_roles`, `e2e_files`), reference (`frozen package` 3, `frozen set` 2, `e2e package` 3, `excluded role` 2, `excluded_roles` 4), verify help (`frozen retained and e2e inputs`), verify refusal (`the frozen e2e package covers`, `the acceptance suite`), and glossary (`Frozen package`; anti-terms `excluded files`, `frozen digest`).

**Why it costs the user:** `excluded_roles` is a role name list, `e2e_files` is a path map that includes the declaration, `Frozen package` counts only the frozen paths, and the refusal calls the same set `the acceptance suite`. A user comparing `Frozen package: e2e, not copied (12 files)` with `e2e_files` (13 entries when the declaration is included) has no label that explains the difference. `Frozen package` is a package, but the number counts paths. `frozen retained` in the help is a third sense: the retained bytes are not the frozen package.

**Replacement:** report `Frozen e2e files: <n> not copied`; record `frozen_roles: [e2e]` and `frozen_paths: <n>` in v2, keeping `e2e_files` as the identity map. Help: "Re-check a trial's retained bytes and frozen e2e files against its record." Use `Frozen package` only when naming the glossary term, never as a count label.

### F7. The five placement labels are printed but not defined, and two are defined differently in the plan

**Surfaces:** report and record counts (`Retained`, `Kept source`, `Replaced`, `Outside the declaration`, `Not replaced`), the reference (the sample block, one sentence for `Retained`, and the record table's `replaced_paths` row; no definition for `Kept source`, `Outside the declaration`, or `Not replaced`), the plan (lines 411 to 430), and the use case (`Not replaced` as direct siblings).

**Why it costs the user:** the report's central output is five counts. The reference that documents the command prints them, defines `Retained` only as a count that excludes frozen paths, and defines `Replaced` only indirectly through `replaced_paths`. `Not replaced` is a sibling count, not the complement of `Replaced`; `Outside the declaration` is computed against `source:` roots in code but described against declared roles in the plan. A user cannot reconcile the counts from the labels, and a reviewer cannot tell which definition is the contract.

**Replacement:** add one definition per count to the command help or the reference's `**Placement.**` paragraph. Rename `Not replaced` to `Sibling files kept` (or change the count to all non-replaced tracked files, which changes behaviour). Rename `Outside the declaration` to `Outside every source root`, matching the code. Keep `Kept source` only if it is defined as "tracked files under a source root that are neither retained nor removed".

### F8. `Spec gap` is a container that includes "no gap", and one value differs from the container by one word

**Surfaces:** served `ise` instruction ("exactly one `Spec gap` outcome"), proposal and review guidance, glossary (`Spec gap`), use case, and the four values.

**Why it costs the user:** `Spec gap: acceptable undocumented change` reads as a contradiction. `Spec gap` and `specification defect` differ by one word, so a reader can mistake the container for the value. The record is prose, not a schema, so nothing checks the pair. The glossary says `acceptable undocumented change` is "no gap at all", which the container name denies.

**Replacement:** always print `Spec gap outcome: <value>` in guidance and records, never `Spec gap: <value>`. Keep the container name and add one sentence to the glossary: "`Spec gap` names the record, not a claim that every outcome is a gap." A rename to `Rebuild difference` would remove the contradiction but reopens Q14 and the permanent corpus.

### F9. `Retained` has three meanings and `Kept source` is a second retention word

**Surfaces:** replace report (`Retained: <n>`), verify clean block (`Retained: clean`), record (`retained_files`, `retained_digest`), glossary (`Retained path`), and the report's `Kept source`.

**Why it costs the user:** `Retained: 20` is a count, `Retained: clean` is a verdict, and `retained_files` is a map. `Kept source` is also "kept", but it is a different bucket: source-root files that are neither retained nor removed. The user must learn that `Retained` never covers `Kept source` and that the verify verdict covers the declaration plus the retained set.

**Replacement:** replace report `Retained paths: <n>`; verify `Retained paths: clean`; glossary `Retained path` stays. Rename `Kept source` to `Source kept` or define it beside `Retained` in the help. Keep the record field names for v1, but document the three senses in one table.

### F10. `E2E digest` and `Trial E2E digest` disagree across two glossary rows

**Surfaces:** report (`E2E digest`), record (`e2e_digest`), glossary `Trial E2E digest` ("printed as `E2E digest`"), and glossary `E2E identity` ("the trial's printed line is the Trial E2E digest").

**Why it costs the user:** the two glossary rows contradict each other and one contradicts the code. A user who reads `E2E identity` expects the printed label `Trial E2E digest`; the report prints `E2E digest`. The shorter label is also the anti-term for the change receipt's identity, so the user cannot tell whether the number belongs to the trial or to the change receipt.

**Replacement:** print `Trial E2E digest:` and keep `e2e_digest` as the field. Amend the `Trial E2E digest` row to say "printed as `Trial E2E digest`" and keep the anti-term list on `E2E identity`. This is a report-only change, no schema change.

### F11. `replaced_digest` is recorded but never re-checked

**Surfaces:** record field, reference (`replaced_digest` fingerprints the replaced paths "as the trial was written, not the product"; verify checks only 64 hex characters), and the report, which omits it.

**Why it costs the user:** a digest in a record named "evidence" implies that verification recomputes it. `verify` never reads a replaced path, so the value is a fingerprint of content that no longer exists in the trial. A user cannot use it to prove the replacement set, and a corrupt-but-64-hex value passes.

**Replacement:** one of three: document "recorded, not re-checked" in the reference field row; remove the field in v2; or recompute it from the recorded revision and compare. The first is cheapest and removes the false promise.

### F12. Record jargon has no command counterpart: `content_scope`, `named_paths`, and `mode`

**Surfaces:** record (`content_scope`, `named_paths`), reference ("content scope", "`--path` input"), served instruction (`Ise Test mode`), and the flags (`--dry-run`, `--full`).

**Why it costs the user:** `content_scope` is a format tag but `scope` elsewhere means `Verification scope`. `named_paths` is the `--path` input; the flag has no `named` and the field's `named` has no definition. The instruction tells the agent to record an `Ise Test` mode, while the command offers `--dry-run` and `--full`; no surface maps `Dry-Run Ise` to `--dry-run` or `Full replacement` to the default write. The user must learn a design-record noun and a CLI flag that never meet.

**Replacement:** record `record_kind: ise-trial/v1` (or keep `content_scope` and document it as a format tag, never a verification scope); rename `named_paths` to `requested_paths` in v2; add one line to the `ise` instruction: "`Dry-Run Ise` is `--dry-run`; `Full replacement` is a write without it." Keep `mode` in the design record and the flags in the command, but name the mapping once.

### F13. Add-on gate refusals use four templates for one condition

**Surfaces:** `givn ise replace` and `verify` ("ise is not enabled; run ..."), `givn instructions ise` ("instruction 'ise' requires the 'ise' addon; enable it with ..."), `givn addons enable ise` ("addon 'ise' requires 'architecture_principles'; enable it first"), and `givn addons disable architecture_principles` ("addon 'architecture_principles' is required by 'ise'; disable 'ise' first").

**Why it costs the user:** the same prerequisite produces four sentences: two name the `ise` gate, one names the `architecture_principles` prerequisite, and one names the reverse dependency. The instruction refusal is tautological because the instruction id and the add-on id are both `ise`: "instruction 'ise' requires the 'ise' addon". A user cannot grep one string to find the gate, and a test that pins one string does not pin the others.

**Replacement:** two templates only. Off add-on: "the '<addon>' addon is off; run `givn addons enable <addon>` first." Missing prerequisite: "the '<addon>' addon requires '<prerequisite>'; run `givn addons enable <prerequisite>` first." Use the same two in the commands, the instruction, and the add-on enable path.

### F14. `Component`, `Port`, and `contract` are cited but absent from the glossary

**Surfaces:** served `ise` instruction's Vocabulary paragraph, `assets/instructions/architecture-principles.md`, and `docs/glossary.md` (no rows).

**Why it costs the user:** the glossary says it is the sole vocabulary authority, and the `ise` instruction points at `architecture_principles` for `Component` and `Port`. That instruction defines lowercase `component`, `port`, `contract`, and `adapter`. The capitalized `Component` and `Port` in `ise` have no glossary row, and `Port` appears once in the vocabulary sentence and never in an obligation. A reader cannot resolve the capitalized terms in the authority the project names.

**Replacement:** either add `Component`, `Port`, and `contract` rows to the glossary with the architecture-principles definitions, or lowercase them in the `ise` instruction and add one sentence: "`component`, `port`, and `contract` are defined by `givn instructions architecture-principles`." Remove `Port` from the `ise` vocabulary sentence if no obligation uses it.

### F15. `Specification declaration` and `declaration` are the same file, and `declared` covers both the file and its entries

**Surfaces:** glossary (`Specification declaration`), reference (`Declaration.`, `declaration` 18 times, `Specification declaration` 0 times), report (`Outside the declaration`), refusal 10 (`the Specification declaration`), and the instruction (`the declaration itself`, `a declared retained path`).

**Why it costs the user:** `declaration` can mean the file, one role entry in the file, or the act of declaring. `Outside the declaration` sounds like "outside the file", which is impossible. The canonical term appears only in the glossary and one refusal; the command's own documentation uses the short form.

**Replacement:** define first use as "the Specification declaration (`givn/specification.yaml`)", then use `declaration` for the file and `declared role` or `declared path` for an entry. Rename the report line `Outside every source root`; that removes the need for `Outside the declaration` entirely. Do not capitalize `declaration` as a term.

### F16. Minor surface drift: "ten fixed lines", `path(s)`, and `Backend` versus `SCM backend`

**Surfaces:** plan ("ten fixed report lines"), reference ("The command prints these lines" and the conditional `Frozen package`), verify (`path(s)`), report (`Backend: git`), and glossary (`SCM backend`).

**Why it costs the user:** `Frozen package` is conditional and sits between two fixed lines, so the ten-line list is not fixed in order or count. `path(s)` is a machine-plural that the `Frozen package` line avoids. `Backend` is an abbreviation of the glossary's `SCM backend`. These are small, but they are the first lines a user reads.

**Replacement:** say "nine fixed lines, then the conditional `Frozen package`, then `Written path`". Pluralize the drift count as `1 drifting path` / `2 drifting paths`. Use `SCM backend: git` or keep `Backend` and add it to the glossary as a short form.

## Decisions needed

### D1. Breaking CLI renames

The three high-cost fixes are `--trial`/`--at` on `verify`, `--receipt` to `--record`, and `replace` to `prepare`. Alternatives:

| Choice | Cost |
| --- | --- |
| Rename all three before the `ise/implementation` merge | The merge is the last cheap moment; the plan, transcripts, harness bundles, and Passuff commands that use the old names must be updated. |
| Rename `--trial`/`--at` and `--receipt`, keep `replace` | Removes the wrong-argument failure and the glossary's banned noun; leaves the destructive-sounding command name and needs a help sentence on every surface. |
| Keep all names, change only docs and report labels | No command churn; the `--trial`/`--at` inversion remains a live usability defect. |

### D2. Record schema v2 or v1 aliases

Renaming `replaced_paths`, `written_path`, `excluded_roles`, and `content_scope` changes `ise-trial/v1`. Alternatives:

| Choice | Cost |
| --- | --- |
| Keep v1 and document aliases | No record churn; the field names keep the misleading nouns. |
| Cut `ise-trial/v2` before merge | Clean names; every archived transcript and harness fixture that reads v1 must be updated or accepted as historical, and older binaries refuse the v2 record by unknown field. |
| Keep v1 fields, add v2 fields beside them | Two names in one record; the strict loader makes the dual shape permanent. |

### D3. Placement semantics

`Not replaced` is a direct-sibling count. `Outside the declaration` is computed against `source:` roots but described against declared roles. Alternatives:

| Choice | Cost |
| --- | --- |
| Keep the counts, rename the labels | No behaviour change; the report and record still need a definition table. |
| Change `Not replaced` to all non-replaced tracked files | Breaks the `Not replaced: 2` scenario and changes the `--full` counts wherever a retained or frozen path shares a replaced directory. |
| Change `Outside the declaration` to match the plan's role test | Breaks the kept-source split; changes the report and the record. |

### D4. `Spec gap` container

The container includes `acceptable undocumented change`, which is no gap. Alternatives:

| Choice | Cost |
| --- | --- |
| Keep `Spec gap`, always print `Spec gap outcome:` | No corpus churn; the container name still contradicts one value in prose. |
| Rename to `Rebuild difference` | Removes the contradiction; reopens Q14 and touches the glossary, instruction, proposal, review, use case, and permanent feature. |
| Keep `Spec gap` only for the three gap values, add a separate `no gap` record | Two records for one observation; contradicts "exactly one outcome". |

### D5. `E2E digest` label

The report prints `E2E digest`; the glossary's `Trial E2E digest` row says that is the printed form, and the `E2E identity` row says the printed form is `Trial E2E digest`. Alternatives:

| Choice | Cost |
| --- | --- |
| Print `Trial E2E digest` | One-line report change; no schema change; makes the anti-term true. |
| Amend `E2E identity` to say `E2E digest` | No code change; keeps the shorter label and accepts the collision with the change receipt's identity. |
| Keep both and define the context | Two labels for one number; the user still has to disambiguate. |

### D6. `replaced_digest`

The field is recorded but not re-checked. Alternatives:

| Choice | Cost |
| --- | --- |
| Document "recorded, not re-checked" | No code change; the false promise remains in the field name. |
| Remove it in v2 | Removes a digest from the evidence record; older records keep it. |
| Recompute it from the recorded revision in `verify` | Restores the promise; `verify` then needs the replaced blobs and the recorded revision, which it currently does not read. |

### D7. Glossary authority for `Component`, `Port`, and `contract`

The `ise` instruction capitalizes terms the glossary does not define. Alternatives:

| Choice | Cost |
| --- | --- |
| Add three glossary rows | One authority; the glossary grows and duplicates the architecture-principles definitions. |
| Lowercase the terms and point to `architecture-principles` | No new rows; the glossary is no longer the sole definition site for terms `ise` uses. |
| Remove `Port` from the vocabulary sentence | Smallest change; leaves `Component` and `contract` unresolved by the same test. |

### D8. Add-on refusal standardization

Four templates cover the `ise` add-on gate and its prerequisite, and tests may pin the exact strings. Alternatives:

| Choice | Cost |
| --- | --- |
| Standardize all four before merge | One grepable message; update every test that asserts a refusal string and the reference's refusal table. |
| Standardize only the `ise` command pair | Small blast radius; leaves `instructions` and `addons` on different templates. |
| Keep the strings and document the mapping | No test churn; the tautological instruction refusal remains. |

## Verification

`givn lint` at the audited revision exits `0` with 64 files checked and writes nothing. This document is the only file added by the audit.

*[Back to the topic document](topic-document.md)*
