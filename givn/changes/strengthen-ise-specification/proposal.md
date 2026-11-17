# Proposal: strengthen-ise-specification

## Release note

The clean-room commands become three named actions (`rebuild-all`, `rebuild-component`, and `compare`), and every one writes a manifest whose checksum rules a second implementation can reproduce byte for byte.

## Use-Case Context

- Use-case ID: `prepare-an-ise-test` (capabilities `clean-room-preparation`, `trial-input-verification`) and `declare-the-specification` (capability `specification-declaration`).
- Ideation topic: `ise-rebuildability` (`docs/ideation/ise-rebuildability.md`).
- Seed path: none. This change responds to the bounded self-rebuild's recorded `Spec gap` records and to the operator's settled command surface, not to an ideation change seed.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Review Context

- Review kind: independent
- Reason: the change pins the byte form of the spec copy's checksums, rewrites the command surface that produces them, and corrects three frozen scenarios. The author who writes the canonical statement and its golden fixture cannot independently prove that the statement matches the implementation bytes. The independent reviewer repeats the second-implementation check against the stated rule alone. This is the major-change reason: the manifest's compatibility contract, the command surface, and the frozen acceptance corpus change together.

## Problem / Opportunity

A fresh agent rebuilt the clean-room commands from a 90-file spec copy in 11 files and 2,772 lines, passed the slice's two `@e2e` scenarios twice, and reproduced 15 of 16 manifest fields. Its gap corpus records five defects in the Specification or its tests:

1. **`scenario-digest-canonical-form`** (`specification defect`, `givn/specs/prepare-an-ise-test/clean-room-preparation.feature`): production and the reconstruction produce different `end_to_end_checksum` and per-feature checksum values for identical inputs because the hashed byte sequence is unstated. The two `@e2e` scenarios assert only a 12-hex abbreviation, so the divergence is invisible to acceptance.
2. **`checkout-package-enumeration`** (`specification defect`, `givn/specs/prepare-an-ise-test/trial-input-verification.feature`): the comparison compares recorded End-to-end (E2E) identities against "the checkout" without saying whether that is the revision, the index, the working tree, or a walk, or whether ignored files count.
3. **`record-field-set-strictness-vs-pre-rule-record`** (`specification defect`, `docs/reference.md`): the reference says a missing field refuses, and also that a record written before `excluded_roles` existed is accepted. It does not say which fields are optional or how a strict reader treats a future field.
4. **`declaration-unreadable-file-api`** (`specification defect`, `givn/specs/declare-the-specification/specification-declaration.feature`): the declaration reader parses text. What a replacement does when the revision records a declaration it cannot read is unstated, as distinct from malformed or absent.
5. **`full-replacement-survives-in-three-scenarios`** (`E2E test defect`, `givn/specs/prepare-an-ise-test/clean-room-preparation.feature`): three scenarios assert that a file under a `source:` root survives a whole-source replacement without stating the Specification role that keeps it. The rule and the scenarios therefore read as a contradiction. The scenario text is wrong; the rule is correct.

The command surface carries a second problem the operator settled. `replace` and `verify` name mechanisms; `--full` and `--path` select the action through flags; the record speaks of `trial`, `receipt`, `retained`, `frozen package`, and `excluded_roles`. None of that is the vocabulary of the Ise pattern. A reader cannot tell from the command list that one command removes the whole Source code, one removes a named component, and one compares the spec copy with the project.

The cost is concrete: a rebuild can pass the frozen suite while producing a different manifest, and a second implementation cannot tell from the Specification whether its manifest is compatible.

## Proposed Solution

The Specification states the three commands, their report lines, the four rule sentences, and the manifest schema; the frozen corpus proves them.

### The command surface

```text
givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]

    Write the spec copy in which the whole product is rebuilt: every declared
    source root is removed from it.
    Ise Test: https://iseprinciple.org/

givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]

    Write the spec copy in which one component is rebuilt: only the named paths
    are removed, and neighbouring components and their contracts stay.
    Constant Ise: https://iseprinciple.org/#pattern-constant-ise

givn ise compare --manifest <file> --copy <dir>

    Report every difference between the manifest and the spec copy's Specification
    files, the checkout's Specification files, and the End-to-end (E2E) tests.
```

`--full` disappears. `rebuild-component` requires at least one path and refuses an empty list as a usage error. `--at` stays optional and defaults to `HEAD`. The earlier names `givn ise replace` and `givn ise verify` are retired; their report and refusal content survives under the new commands, and refusals print `ise rebuild-all:`, `ise rebuild-component:`, or `ise compare:` to match the command that refused. A path `rebuild-component` refuses because it lies outside every declared source root is refused with a sentence naming `rebuild-all`, the command that writes the copy removing every declared source root.

Vocabulary: **spec copy**, **manifest** (with a `checksums` section), **checksums**, **Specification files**, **Source code**, **End-to-end (E2E) tests**, **compare**. The words `trial`, `trial record`, `receipt`, `retained`, `kept source`, `replaced`, `outside the declaration`, `not replaced`, `frozen package`, `excluded_roles`, and `verify` are retired from the command surface and from the prose this change touches.

### Report lines

Both `rebuild-*` commands print eleven fixed lines, and a twelfth when a manifest path was given:

```text
Backend: <git | fossil>
Revision: <resolved revision>
Specification files: <n>
Source code kept: <n>
Source code removed: <n>
Outside the spec copy: <n>
Files beside removed paths: <n>
Specification checksum: <12 hex>
End-to-end checksum: <12 hex>
End-to-end tests: not copied (<n> files)
Copy directory: <dir>
Manifest: <file>
```

`--dry-run` prints `Copy directory (would write):` and, only when `--manifest` was given, `Manifest (not written):` instead of the last two lines, and writes nothing. `Manifest:` prints only when `--manifest` was given; the manifest is never printed to stdout. Each of the five counts has a one-sentence definition in the reference.

`compare` prints two subject lines:

```text
Specification files: match | <n> changed | <n> missing | <n> added
End-to-end tests: match | <n> changed | <n> missing | <n> added
```

A changed or missing path prints one `- <path>` line and an added path prints one `+ <path>` line, one line per path per run; a path that drifts in both the copy and the checkout prints one line, counted and owned by `End-to-end tests` when the checkout side owns the path and by `Specification files` otherwise. When one subject has more than one difference kind, its non-zero counts print in the fixed order `changed`, `missing`, `added`, comma-separated; a zero count is omitted, and `match` prints alone. For example, `Specification files: 1 changed, 1 added`. Exit 0 requires both lines to say `match`; any difference exits 1. A Specification file that changed in the checkout since the copy was written prints `Specification changed after the copy was written` and one `- <path>` line. A feature path that carries a selected `@e2e` scenario is owned by `End-to-end tests` on the checkout side, so its scenario change prints one `- <path>` line and no second sentence.

The checkout also contributes added paths. An added content path in the checkout's declared End-to-end (E2E) set, an added feature file carrying a selected `@e2e` scenario, and an added `e2e` root in the checkout's declaration are `End-to-end tests: <n> added` differences; an added content path matched by another declared Specification role is a `Specification files: <n> added` difference only when it lies outside the declared End-to-end (E2E) set and does not carry a selected `@e2e` scenario. Each prints one `+ <path>` line and exits 1. Membership in the checkout's declared End-to-end (E2E) set, or a selected `@e2e` scenario in a feature path, decides the subject for an added path, even when another Specification role also matches it.

### The rule the spec copy carries

Four sentences, in the served `ise` instruction and the reference:

> The spec copy is written with the project's Specification files. During a
> rebuild the agent adds files, and those are the rebuilt Source code. The agent
> may edit Source code kept because it is the product, but it does not delete
> any file the copy was written with. The agent does not modify any
> Specification file the copy was written with; adding a file under a declared
> Specification path is a Specification change, not the product, and is
> refused.

The manifest records the copy's Specification files and the paths it wrote as Source code kept. The comparison refuses a copy that lost a recorded kept path, naming it, and does not verify kept content, so a kept edit is allowed and unreported (R-162).

**Symlink containment.** A rebuild command refuses to write a symlink with an absolute target, even when that target resolves inside the spec copy, and a symlink whose relative target resolves outside the spec copy: an absolute target always refuses, and a relative target refuses when resolving it against the link's parent directory leaves the copy. The refusal names the link path and the target, exits 1, and writes nothing. A relative target that resolves inside the copy is written as recorded. R-150 covers the archive guard reading a declared Specification path that is a symlink into the change tree; this refusal covers a symlink entry written into the copy, so the two checks read different surfaces.

The `ise` instruction's existing obligations stay, restated in the new vocabulary.

### The five strengthen statements

1. **Canonical checksum bytes.** One path checksum is `sha256(path, NUL, kind, NUL, executable, NUL, content)`: `kind` is `f` (regular), `x` (executable regular), `l` (symlink), or `g` (gitlink); `executable` is `1` or `0`; `content` is the file bytes, the symlink target bytes, the recorded commit id for a gitlink, or the bytes the backend returns for the entry. The End-to-end (E2E) set may hold a gitlink; the copy set refuses one by name. A rollup is `sha256` over each `path, NUL, checksum-hex, LF` in ascending path order. `end_to_end_checksum` rolls up the union of `checksums.end_to_end` and `checksums.scenarios` keyed by path. A per-feature scenario checksum is `sha256(feature-path, NUL, (title, NUL, executable-content, NUL)...)` over the selected scenarios in file order. The executable content is the compact JSON array `[scenario keyword, background-or-null, sorted tags, steps, examples]`; tags exclude every `givn.` tag and `wip`. The report prints the first 12 lowercase hex characters; the manifest keeps all 64.
2. **End-to-end (E2E) tree enumeration.** The End-to-end (E2E) set is read from the selected revision's tracked paths, never the working tree. Each declared `e2e` entry matches tracked files and tracked directory prefixes. A matched file resolves through its parent directory. Each match takes as its root the nearest directory at or above it that holds a tracked `Cargo.toml`, or the matched path itself when no such directory exists. The set is every tracked file at or below a root, minus the declaration, ordered by path. An entry that matches the project root covers the whole tracked tree. Ignored and untracked files cannot enter. `compare` reads the checkout's content paths instead: tracked files plus untracked non-ignored files, sorted and deduplicated, with bytes read from disk.
3. **Manifest compatibility.** `ise-manifest/v1` names its scope and requires every field it lists. A field added later is optional with the older manifest's content as its default; a required field or a changed meaning opens a new scope, and a reader refuses a mismatched scope naming the manifest and the expected value. The older `ise-trial/v1` record shape is refused by name and is not migrated.
4. **Unreadable declaration.** Preparation reads the declaration recorded at the selected revision as a blob and never reads the working tree's declaration. Four cases are distinct, and all refuse with exit 1 before any write: the revision does not record the declaration (names the path, the revision, and `does not record`); the revision records it but the backend returns no blob (names the path, the revision, and `returned no blob`); the bytes are not UTF-8 (names the path and the decoding error); the bytes are not a valid declaration (names the path and the loader's reason). A present but unreadable working-tree declaration is the declaration reader's caller's case: `givn lint` refuses it by name and exits 1.
5. **Fixture wording.** The three scenarios state the tracked file and the Specification role that keeps it under `rebuild-all`: `src/kept.rs` is a Specification file through the `structure` role, and the executable script and symlink are Specification files through the `structure` role, each under the `source: src/**` root. Their asserted behaviour is unchanged.

The executable proof is two regular scenarios. The first writes the pinned canonical fixture, whose declaration, feature, documents, package manifest, and source bytes are stated in the owning use-case example, and asserts the full 64-hex `specification_checksum`, `end_to_end_checksum`, and per-path checksums. The second writes the pinned richer fixture, whose feature carries a background, a docstring with delimiter-adjacent line breaks, a table with Gherkin escapes, an outline, and Examples, and asserts its full 64-hex scenario checksum, `specification_checksum`, and `end_to_end_checksum`. A second implementation that follows the stated rule produces the same bytes; the design records the independent script that recomputed both fixtures from the rule alone.

### The manifest

The manifest is ephemeral evidence of one spec copy. It lives outside the copy and the checkout, is not archived or committed, and is required only by `compare`.

```yaml
scope: ise-manifest/v1
revision: <resolved revision>
removed_paths: [<path>...]
specification_files: [<path>...]
source_kept_files: [<path>...]
end_to_end_files: [<path>...]
counts:
  specification_files: <n>
  source_kept: <n>
  source_removed: <n>
  outside_spec_copy: <n>
  files_beside_removed_paths: <n>
checksums:
  specification: {<path>: <64 hex>}
  end_to_end: {<path>: <64 hex>}
  scenarios: {<feature path>: <64 hex>}
specification_checksum: <64 hex>
end_to_end_checksum: <64 hex>
copy_directory: <dir>
givn_version: <version>
```

The `specification_files`, `source_kept_files`, and `end_to_end_files` lists are the sorted path inventory; `end_to_end_files` is the sorted union of the keys of `checksums.end_to_end` and `checksums.scenarios`. `source_kept_files` records paths only, so `compare` refuses a deleted kept path and does not verify kept content. `checksums.end_to_end` holds one path checksum per `end_to_end_files` path, including a feature path that carries a selected `@e2e` scenario, and the rollup uses the scenario checksum when a path is in both `end_to_end` and `scenarios`.

## Capability Routing

`givn spec route` reported one recommendation: `EXTEND givn-workflow-enforcement` (use case `verify-change`, score 17.00). The match is template vocabulary (`givn check`, `route`), not this change's subject; every candidate it ranked belongs to change-authoring enforcement, while this change edits the clean-room commands' existing capabilities.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `clean-room-preparation` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND clean-room-preparation` | `rebuild-all` and `rebuild-component` are the two forms of the existing prepare action: one removes the whole declared Source code, one removes named component paths. That split was a flag choice before and is a command choice now; no new action or use case exists. |
| `trial-input-verification` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND trial-input-verification` | `compare` performs the existing re-check action under the settled name, with the same three subjects: the copy's Specification files, the checkout's Specification files, and the End-to-end (E2E) tests. No new action exists. |
| `specification-declaration` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND specification-declaration` | The unreadable working-tree declaration is an existing refusal of `givn lint`; the change pins its exit code and states the reader's file-access boundary. No new action exists. |
| `prose-consolidation` | not ranked | `EXTEND prose-consolidation` | The delta rewrites one permanent `prose-consolidation` scenario so the served `ise` guidance states the settled vocabulary; the permanent feature lives in `givn/specs/create-change/` and the delta sits in `specs/create-change/`. No new action exists. |
| `instruction-integrity` | not ranked | `EXTEND instruction-integrity` | The delta rewrites one permanent `instruction-integrity` scenario so the served `ise` instruction states the retained-path rule; the capability is owned by `givn/specs/fragments/` and the delta sits in `specs/fragments/`. No new action exists. |

## Out of Scope

- The promotion decision for `ise` (`S14`).
- The `ise` guidance note's proposed instruction sentences in `docs/ideation/ise-rebuildability/guidance-improvements.md`.
- What a trial contains: the declaration's roles, the placement rules, and the component boundary are unchanged.
- The 12 `acceptable undocumented change` records and the 2 `implementation defect` records from the bounded self-rebuild's gap corpus.
- Renaming the capability identifiers `clean-room-preparation` and `trial-input-verification`. The second contains a retired noun; it is an identifier the permanent corpus and the interaction inventory address, and renaming it would fork `givn/specs/prepare-an-ise-test/` without a behaviour change. Recorded as D2.
- A new `@e2e` action. `rebuild-all` and `rebuild-component` are variants of one prepare action, and `compare` is the re-check action under its settled name; the two `@e2e` scenarios are replaced, not added to.

## Open Questions

### D1 - Whether the manifest is printed when `--manifest` is omitted

- Question: Should a `rebuild-*` command print the manifest to stdout when `--manifest` is not given, as the retired `--receipt` flag did?
- Options:
  1. **Report only**; gain: stdout stays the report the report lines define, and `compare` cannot be fed by accident from a log; cost: a developer who wants a manifest must name a file; in practice: `givn ise rebuild-all --into /tmp/copy` prints the eleven fixed report lines and writes no manifest.
  2. **Print the manifest after the report**; gain: one command both writes the copy and shows its evidence; cost: the report is followed by YAML that a reader must strip, and a manifest becomes a log artifact rather than a file with a scope; in practice: every report consumer handles two output shapes.
- Disposition: answered. Source: the settled surface names `--manifest <file>` on both commands and `Manifest: <file>` as a report line; a printed manifest has no path to print.
- Decision: report only.

### D2 - Whether to rename the capability `trial-input-verification`

- Question: The retired noun `trial` survives in the capability identifier `trial-input-verification`. Rename it to a spec-copy name, or keep it?
- Options:
  1. **Keep the identifier**; gain: the permanent corpus, the interaction inventory, and the delta path keep their addresses, so the change stays a behaviour correction; cost: one identifier keeps a retired noun; in practice: `/skill:givn-*` guidance and `givn spec tree` still print `trial-input-verification`.
  2. **Rename it**; gain: no retired noun anywhere; cost: a permanent corpus move, a new feature path, an interaction-inventory rewrite, and an archive merge that creates one file and retires another in the same change; in practice: the change would need a corpus-rename step with its own review, and a second implementation would face two paths for one capability.
- Disposition: answered. Source: the change brief names the capability's action as `compare` and keeps the use case; identifiers are not prose.
- Decision: keep the identifier; the prose this change touches uses `compare`.

### D3 - Rule-nested `@e2e` scenarios in the per-feature checksum

- Question: Should a scenario nested under a Gherkin `Rule:` and tagged `@e2e` enter the per-feature scenario checksum? The current parser exposes no `Rule` scenarios, so the selection excludes them; whether the frozen acceptance runner executes one is unproven.
- Options:
  1. **State the exclusion** (current bytes); gain: no behaviour change, the manifest stays reproducible; cost: a `Rule:`-nested `@e2e` scenario would not be covered by the checksum while a runner might still execute it; in practice: a project adds `Rule: Billing` with an `@e2e` scenario and the checksum omits it.
  2. **Include `Rule:` scenarios**; gain: the checksum matches a runner that executes them; cost: a parser and selection behaviour change outside this change's scope; in practice: the checksum would grow an entry for every `Rule` scenario and every existing manifest would need re-derivation.
- Disposition: carried. Owner: the independent verifier. Deciding artifact: a follow-up change that aligns the runner and the checksum. This change states option 1's exclusion and records the residual risk in chapter 11.

### D4 - How the retired `ise-trial/v1` record shape is treated

- Question: Does `compare` read a manifest written before this change, or refuse it?
- Options:
  1. **Refuse by name**; gain: one schema with one strict reader, no second field set to maintain; cost: a spec copy prepared by an earlier version cannot be compared without re-preparing it; in practice: `givn ise compare` on an `ise-trial/v1` file prints `ise compare: the manifest scope is 'ise-trial/v1', expected 'ise-manifest/v1'` and exits 1.
  2. **Read both shapes**; gain: an in-flight rebuild survives the upgrade; cost: the reader carries two field sets, and `compare` must guess which counts and checksums the old names map to; in practice: every reader branch is a compatibility case with its own tests.
- Disposition: answered. Source: the change brief states the older record shape is not read by this command; the manifest is ephemeral, so re-preparing costs one command.
- Decision: refuse by name.
