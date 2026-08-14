# Design Review: prose-consolidation

## Scope

**PASS.** The proposal asks for one normative E2E/interaction policy and thin
generated entry points. The spec now covers canonical output, generated
guidance parity, project override preservation, flag-variant semantics, and
design/review delegation. The override scenario is deliberately regular: it
is a configuration variant of the same `givn instructions specs` action, not a
new inventory action.

## Technical Choices

**PASS.** The existing instruction cascade, embedded assets, renderer, and
real `givn` subprocess harness are reused. No include preprocessor, renderer
API, dependency, or runtime protocol is introduced. The `specs` instruction is
the canonical source because it is the policy's authoring entry point.

## Missing Scenarios

**PASS after hardening.** The feature now covers:

- canonical policy and its exact legacy wording removal;
- generated skill/command delegation and negative legacy-wording checks;
- project override replacement;
- output-affecting flag combinations classified as variants;
- design and review instructions delegating to the canonical source.

The embedded asset allowlist in design.md includes instructions, templates,
skills, and slash commands. Review performs the repository-wide parity scan in
addition to the executable scenarios.

## Testability and E2E Fidelity

**PASS after hardening.** The two normalized inventory entries have exactly one
`@e2e` scenario each. Both use the real `givn` CLI subprocess and assert the
CLI result before checking generated files. The project override and policy
variant scenarios are regular regression scenarios. The runner is strict via
`.fail_on_skipped()`; undefined steps fail non-zero before the capability step
file is implemented.

The generated-guidance scenario explicitly enables `opencode`, `.agents`, and
`.claude`, checks every configured skill target, checks the opencode command,
and rejects both concrete legacy phrasings. The design/review scenario reads
both instruction outputs through the CLI and checks canonical delegation.

## Risk

**PASS.** The dominant risk is leaving a duplicate wording in an omitted asset.
The allowlist names every policy-bearing embedded instruction, template, skill,
and command. The review scan rejects the old inventory-entry wording and
requires non-canonical sources to reference `givn instructions specs`.

## Architecture Documentation

**PASS.** Independent chapter assessment matches `arc42.md`:

| # | Assessment |
|---:|---|
| 1 | Yes: agent-authoring consistency is an explicit quality goal. |
| 2 | No: no new external constraint. |
| 3 | Yes: canonical guidance is a user/agent-facing CLI output. |
| 4 | No: the existing Rust/embedded-asset solution strategy is reused. |
| 5 | Yes: instruction, skill, command, and cascade responsibilities are clarified. |
| 6 | No: command flows are unchanged. |
| 7 | No: no deployment change. |
| 8 | Yes: guidance ownership and override resolution are cross-cutting. |
| 9 | Yes: ADR-038 records the single-source decision. |
| 10 | Yes: canonical-output, parity, and override quality scenarios are added. |
| 11 | Yes: R-17 records drift risk and parity mitigation. |
| 12 | Yes: `Canonical authoring policy` is added to the glossary. |

Chapters 01, 03, 05, 08, 09, 10, 11, and 12 contain matching decision-
specific updates. Chapter 09 ADR-038 uses the full MADR structure, including
drivers, options, consequences, and confirmation. Existing Mermaid/exception
policies and chapter integrity remain unchanged.

## Hardening Applied

- Replaced tautological negative assertions with the actual legacy wording.
- Added flag-variant semantics to the canonical specs instruction and design.
- Expanded the consolidation allowlist to templates, commands, skills, and
  artifact instructions.
- Added generated-target parity and legacy-wording checks.
- Added primary CLI output evidence to the skills-sync E2E scenario.
- Made the project override a regular configuration-variant scenario.
- Added canonical delegation checks for design and review instructions.
- Completed ADR-038 and reconciled R-17 plus Arc42 quality chapters.
- Rechecked the design after converting the Step Definitions section to the
  required Markdown table format for the integrity hook.

DESIGN-REVIEW: PASS
