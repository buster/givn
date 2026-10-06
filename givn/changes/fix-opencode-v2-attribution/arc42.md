# arc42 Documentation Update: fix-opencode-v2-attribution

## Scope and evidence boundary

Documentation for a planned change. Read the complete proposal, both fragment
delta features, technical design, durable support-fragment inventory, all
twelve current chapters, architecture index, and active/archived ADR register.
The selected approach preserves the supplied prototype except for default V1
`server()` delegation. The user's “No changes to the plugin logic itself.”
constraint remains binding, including its inaccurate fallback possibilities.

The compatibility contract covers V1 1.18.29+ within V1 and V2, using one
embedded canonical asset and the existing selected-target generation seam.
Real-loader compatibility, deterministic provider/catalog twins, OS network
isolation, and prototype characterization are planned, not implemented or run.
This artifact completes documentation only; it is not implementation,
design-review, coverage, terminal, or archive proof.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals / 10 quality-requirements | Yes | The selection row includes quality attributes: supported-generation compatibility and truthful best-effort interpretation are refined in chapter 10. Existing goals, stakeholders, and confirmed Personas in chapter 01 remain unchanged. | Refine compatibility/truthfulness quality acceptance in chapter 10; no chapter-01 goal or stakeholder edit. |
| 2 | 02 architecture-constraints | Yes | The V1-payload-only constraint does not describe the planned supported external compatibility window. | Replace TC-29 with planned recent-V1/V2 support, excluded older/future APIs, and unverified fallback labels. |
| 3 | 03 context-and-scope | Yes | The existing OpenCode interface must cover both generations without promising verified current-session facts. | Refine the interface row and distinguish planned compatibility/test-only twins from production External Systems. |
| 4 | 04 solution-strategy | Yes | Test-runtime provisioning and runner admission are explicitly constrained to the flake-owned development environment; setup uses permanent specs while delta scenarios remain @wip, with a final four-scope run after TDD completion. | Document the flake-only provisioning boundary, shell-only validator, and setup/final runner phases. |
| 5 | 05 building-block-view | Yes | The existing test acceptance blocks use a POSIX-shell validator and suite wrapper, with permanent-only setup and post-TDD all-root verification. | Describe the flake-exported runtime paths, shell validator, final four-scope wrapper, and supervisor as test-only blocks. |
| 6 | 06 runtime-view | Yes | V2 request observation and shell preparation differ from the existing V1 flow. | Preserve V1 hooks; add V2 sequence, session/last/default precedence, late session-ID injection, unavailable handling, and regeneration/precondition boundaries. |
| 7 | 07 deployment-view | Yes | Runtime provisioning is now owned solely by the development flake; scripts do not bootstrap tools at test time. | Retain one selected-target file; correct Claude footprint to settings.local.json; distinguish the active development-shell runtime closure from deployed services. |
| 8 | 08 crosscutting-concepts | Yes | Canonical test commands validate paths exported by the active shell without invoking Nix or Fish. | Document shell-only runtime validation, fail-closed prerequisites, and retained attribution fallbacks. |
| 9 | 09 architecture-decisions | No | The local reversible adapter approach fails ADR qualification; existing generated-agent ownership and upgrade-safety boundaries remain unchanged. | No ADR, chapter-09, or ADR-index edits. |
| 10 | 10 quality-requirements | Yes | Setup proves permanent runner connectivity and provider readiness; real V2 attribution evidence follows the adapter TDD scenario. | Refine Q-78; add planned Q-93–Q-95 and distinguish setup smoke from real-loader scenario evidence. |
| 11 | 11 risks-and-technical-debt | Yes | Cross-session/late-ID, default-model, retained-variant, and active-shell admission/cleanup limitations must not be presented as corrected. | Refine R-50 and R-73–R-76; require shell-only canonical-gate validation and signal-aware owned-group teardown probes. |
| 12 | 12 glossary | Yes | Existing attribution terms incorrectly equate every supplied label with a current-session fact. | Apply the design's exact Attribution context, Model identity, and Reasoning mode definitions with a planned-V2/no-fabrication scope note; add no Aggregate or Domain Event. |

## Index and user-facing changes

Hardening on 2026-10-04 corrected the original row-1 mismatch: the independent
grilling assessment was **Yes**, while this marker was **No**. Row 1 now says
**Yes** and names both mapped chapters; the actual quality refinement remains
in chapter 10. All twelve final classifications agree. This before/after record
does not erase the original finding or create new chapter-01 goals.

- `docs/arc42/README.md`: link the compatibility design and affected chapter
  scope; explicitly separate documentation completion from execution proof.
- `README.md`: add the planned support window, six regeneration actions,
  retained label/variant caveat, overwrite boundary, and design link. Preserve
  unrelated release, coverage, and installation content.
- The reusable support fragment (`givn/specs/fragments/fragment.md`) already
  contains all 38 normalized interactions. It was read, not changed; no
  duplicated inventory or invented `usecase.md` is added.

## ADR classification

ADR action: CANONICAL_ARTIFACT

ADR target: `givn/changes/fix-opencode-v2-attribution/design.md`

The Boolean gate is false: real alternatives and external-protocol impact
pass, but durable consequence and lower-level artifact classification fail.
The design remains the one canonical home for the local plumbing rationale;
feature files own observable behavior and chapters own the affected facts.

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/fix-opencode-v2-attribution/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "The proposal records the user choosing recent V1 plus V2 over including older V1.",
      "The user explicitly disallowed attribution-logic changes; rejected alternatives remain in the proposal."
    ],
    "architectural_impact": [
      "One existing attribution adapter supports both external plugin protocols without moving ownership or dependencies."
    ],
    "durable_consequence": [
      "The embedded adapter entrypoint update is reversible within one change, without data migration or a new cross-component invariant."
    ],
    "lower_level_artifact": [
      "The two existing capability features own compatibility behavior; design.md completely owns the local realization and preservation rationale."
    ],
    "existing_adr_check": [
      "Read docs/arc42/09-architecture-decisions.md and docs/arc42/adr/README.md, including its archived-register section; there is no separate archive/README.md.",
      "Searched docs/arc42/adr recursively for OpenCode, opencode, plugin, attribution, and agent integration; no independent attribution ADR was found.",
      "Read generated-agent ownership (ADR-013) and preservation-oriented upgrade (ADR-023); neither boundary changes."
    ]
  }
}
```

### Falsification and supporting checklist

- **Single-change reversibility applies:** one adapter's local entrypoint and
  test evidence can be reversed in one change. Multiple regeneration callers
  do not become separate changed production components.
- **No multi-change constraint applies:** no new shared authority, dependency,
  data, or deployment boundary constrains two future components or changes.
- **Product-behavior exclusion applies:** supported attribution after each
  maintenance action belongs to the existing fragment's Gherkin contract; no
  component, authority, or deployment boundary is moved.
- [ ] **Hard to reverse:** local asset/test change; no migration required.
- [ ] **Shapes lasting quality attributes:** compatibility evidence is local,
  not a new system-wide quality floor.
- [x] **Structural:** compatibility with two existing external plugin protocols
  affects the adapter interface, not its ownership boundary.
- [x] **Must be shared:** maintainer support context only; supporting evidence
  cannot override failed mandatory dimensions or falsification.

No parallel ADR, amendment, supersession, archive copy, or register entry is
created. The ADR procedure remains owned by the Arc42 instruction.

### Retained best-effort behavior candidate

The separate choice to retain supplied model/reasoning fallbacks is also
`NOT_QUALIFIED`: alternatives `PASS`; architectural impact `FAIL`; durable
consequence `FAIL`; lower-level artifact `FAIL`; existing-ADR check `PASS`;
must-be-shared `SUPPORTING`. Route: `CANONICAL_ARTIFACT`, exactly one destination
`givn/changes/fix-opencode-v2-attribution/specs/fragments/attribute-commits.feature`.
Current-session-only unavailable values were rejected by the user's explicit
preservation constraint. No component, authority, dependency, deployment, or
persisted-data boundary moves. Observable fallback behavior is fully owned by
that feature; glossary and chapter-11 risks clarify it without duplicating an
ADR. All three falsification tests apply; no MADR action is needed.

## Documentation validation and execution limitations

- Read `README.md` and `AGENTS.md`; loaded `givn-domain-modeling` and
  documentation/OpenCode skills. Read the full embedded Arc42 and dialog
  instructions, plus the project impact-assessment template. No project
  Arc42 instruction override or dialog override was found at the inspected
  cascade paths.
- This subagent has no shell execution tool. `givn instructions`,
  `givn instructions arc42-docs --change fix-opencode-v2-attribution`, and
  `givn instructions dialog` could not be executed here. Reading their source
  instructions is not represented as resolved CLI output. The parent supplied
  the discovered artifact order: this documentation step precedes design-review.
- Used `patch` only for documentation edits; all twelve assessment rows have
  explicit Yes/No decisions and reasons. New diagram content is Mermaid with
  an adjacent textual description. Retained risks and fallback examples are
  checked against the complete supplied design/specs, not invented fixes.
- Read back the edited sections and marker. Grep confirmed exactly twelve
  assessment rows and the final Status marker, with no conflict markers in
  the assessment. Trailing-whitespace searches returned no matches in
  `docs/arc42/**/*.md`, root `README.md`, or this assessment. These are text
  checks, not a substitute for `git diff --check`.
- `git diff --check` could not be executed in the original author session
  without a shell tool; that original session did not claim a pass. No runtime, Cucumber, Nix,
  strictness, coverage, or visual-evidence tests were executed here.
- Proposal/specs/design, user plugins, harness plugin, production code, tests,
  configuration, tasks, and review artifacts were not edited. No regeneration,
  commit, status command, subagent, or next-stage action was performed.

### Parent validation after hardening — 2026-10-04

The original author-session limitations above remain historical facts. The
parent supplied actual command evidence from
`/Users/daniel.kressner/dev/PRIVATE/givn`: its first action `givn instructions`
exited 0; resolved design-review JSON and dialog commands exited 0 with full
output read. After hardening, resolved arc42-docs, design, specs and review
instruction commands all exited 0 with full output read. Fresh change lint
exited 2 (`2 file(s) checked, 18 finding(s)`, WIP/advisory findings, no parse
failure); `git diff --check` exited 0 with no output. These are parent results,
not commands executed by either shell-less documentation session. Detailed
provenance and the raw lint-output path are in `design-review.md`.

A separate fresh quality reviewer, session `ses_ef86b9d55ffeSsDeiR8VlMmRHk`,
approved the hardening with no blockers, majors or minors and confirmed
instruction consistency, all-12 assessment alignment and both prototype hashes.
The historical instruction/whitespace validation limitation is resolved; no
runtime, strictness, isolation, cleanup, coverage or terminal proof is implied.
Those implementation/setup checks remain mandatory and unexecuted.

## Status
STATUS: DONE
