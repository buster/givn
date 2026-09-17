# Design: term-introduction-discipline

## Technical Approach

Guidance-only change: three assets and one project document. No runtime code,
dependency, manifest entry, or gate is added. The existing asset cascade and
skill renderer are reused; the deterministic term checker stays out of scope.

## Term Record Schema

`assets/skills/givn-ideation/SKILL.md.tmpl` gains the `domain-terms.md` entry
schema next to the topic-state list, following the `questions.md` and use-case
schema precedent in the same file:

```markdown
| Term | Status | Definition | Contrast | First use | Collision check |
|---|---|---|---|---|---|
| <Term> | candidate | <one sentence> | <X, not Y> | <file:line> | <commands run, result> |
```

Status is `candidate`, `confirmed`, or `rejected`:

- `candidate` — the definition and first use are sufficient.
- `confirmed` — requires a genus-and-differentia definition, a contrast pair,
  and one concrete domain sentence (the full term introduction test lives in
  `givn-domain-modeling`) plus a recorded collision check. The one-line
  criteria keep `confirmed` reachable in a project without the domain-modeling
  skill; the full test is not duplicated here.
- `rejected` — keeps the definition and the rejection reason; never deleted.

Escape `|` as `\|` inside the Collision check cell and prefer hit-count
commands without pipelines, so the recorded command stays inside its column.

## Search-Before-Coin Procedure

The Event Storming collision bullet becomes a three-step procedure that must
be executed and recorded before a term reaches `confirmed`:

1. Grep the durable glossary (`docs/arc42/12-glossary.md` when the arc42 addon
   is enabled) for the term and its synonyms.
2. Grep every sibling topic's `domain-terms.md` and the repository for the
   term.
3. Record the commands run and the hit counts in the `Collision check` column.
   "No collision" without recorded commands is not a record.

## Term Introduction Test

`assets/skills/givn-domain-modeling/SKILL.md.tmpl` gains a section that gates
`confirmed`:

1. **Genus + differentia** — "a `<Term>` is a `<kind>` that `<distinguishes
   it>`". Example: "a Port is the interface contract at which one
   implementation can be replaced".
2. **Contrast pair** — "`Term`, not `<near neighbour>`" with the distinction.
   Example: "Port, not Boundary — a boundary is a limit that must not be
   crossed".
3. **One concrete domain sentence** using the term.

The failure mode is demonstrated: "a Substrate is a kind of ___" has no
referent, so the underlying concepts must be named separately. The default
anti-term list is folded into this section as the worked generic-word set
rather than a separate rule set.

## Ideation Flow Integration

| Surface | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` — topic state | `domain-terms.md` gains the entry schema. |
| `assets/skills/givn-ideation/SKILL.md.tmpl` — Event Storming | The collision bullet points at the three-step procedure. |
| `assets/skills/givn-ideation/SKILL.md.tmpl` — Quality reviews | The Event Storming branch verifies a recorded collision check for every `confirmed` term entry. |
| `assets/skills/givn-ideation/SKILL.md.tmpl` — Handoff | Only `confirmed` terms are proposed for the durable term list; `rejected` terms are reported with their reason. |
| `assets/skills/givn-domain-modeling/SKILL.md.tmpl` | New term introduction test section; references the ideation schema as the record's home. |
| `docs/workflow.md` §3 | Short paragraph naming the term record and pointing at the skill; the schema is not duplicated. |

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| The ideation guidance carries the term schema and the collision procedure | "domain-terms.md", "`candidate`, `confirmed`, or `rejected`", "Contrast", "First use", "definition and first use", "recorded collision check", "only `confirmed` terms", "never deleted", "givn-domain-modeling", "Collision check", "Grep the durable glossary", "without recorded commands is not a record" |

Bare status words are deliberately not asserted: "confirmed" and "rejected"
already occur in the current template as unrelated substrings, so the scenario
asserts the status list and the schema-distinctive phrases instead.

Per-step phrase mapping for the domain-modeling scenario:

| Step | Asserted phrases |
|---|---|
| Then the body requires a genus and differentia definition | "Genus + differentia" and "is a `<kind>` that" |
| Then the body requires a contrast pair | "Contrast pair" and "not `<near neighbour>`" |
| Then the body requires one concrete domain sentence | "concrete domain sentence" |
| Then the body demonstrates a failure when the genus slot cannot be filled | "Substrate" and "has no referent" |

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| Generated ideation guidance is synchronized from canonical assets | `givn/specs/author-change/usecase.md` | Term schema, procedure, review branch, and handoff rule live in the ideation skill template | The ideation guidance carries the term schema and the collision procedure |
| Domain modeling maintains the arc42 glossary | `givn/specs/fragments/domain-skill-trigger.feature` | The introduction test lives in the domain-modeling skill template | The domain-modeling guidance carries the term introduction test |
| Ideation parks unknowns and reviews at phase boundaries | `givn/specs/fragments/ideation-skill.feature` | The quality-review branch verifies recorded collision checks | Existing ideation guidance scenarios remain green |

Persona impact: `agent--technical-role` consumes the generated ideation and
domain-modeling guidance; `givn-maintainer--owner` owns the rule corpus and
the term-record convention. No Persona is invented or promoted; Gherkin actors
stay the Developer and the Authoring agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| domain-skill-trigger | `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` | Direct skill-template read; adds one When and four Then steps |

The new steps read `assets/skills/givn-domain-modeling/SKILL.md.tmpl`
directly, matching the existing steps in that file; no CLI call is needed
because the permanent scenarios already prove the generated path.

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/term-introduction-discipline/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI for the generated-guidance scenario; direct asset
  read for the domain-modeling template scenario.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| ideation-skill: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing) | CLI | Real `givn skills sync` subprocess |
| domain-skill-trigger: synchronize skills and route the domain skill (fragment; no inventory) | An agent synchronizes skills and the domain skill is routed (existing) | CLI | Real `givn skills sync` subprocess |

No new `@e2e` scenario is added: the actions (synchronize skills, route the
domain skill) are already proven end-to-end, and the new scenarios are
guidance-content variants. No browser UI exists.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | the skill renderer path that emits the ideation and domain-modeling guidance |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

Guidance content changes only: the ideation skill template, the
domain-modeling skill template, and one project workflow document. The
renderer, cascade, generated targets, and gates are untouched.

## Risks

- Pure authoring guidance: a lazy agent can write "no collision" without
  grepping. The pasted-command requirement is the mitigation, consistent with
  the repository's existing paste-the-command culture.
- Requiring the full test only for `confirmed` keeps early ideation cheap;
  `candidate` entries stay one line.
