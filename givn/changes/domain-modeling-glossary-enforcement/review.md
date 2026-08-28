# Review: domain-modeling-glossary-enforcement

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
0 empty step bodies / 0 unimplemented!() found across 2 step definition files:
  tests/steps/fix_domain_modeling_skill_trigger_steps.rs — real assertions
  tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs — real assertions
```

The shared removed-scenario placeholder step has a real assertion body (reads
the skill template asset and asserts it names the skill) — never empty.

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| The skill trigger names conversation states (modified) | 0fc7edd | Yes (assets/skills/givn-domain-modeling/SKILL.md.tmpl + step) |
| The ubiquitous language is recorded in the project's arc42 glossary | a732d01 | Yes (SKILL.md.tmpl + shared step grep) |
| The skill requires the arc42 glossary for domain modeling | 3ff1afd | Yes (SKILL.md.tmpl + step) |
| The next-step guidance names the domain skill during later change phases | e551484 | Yes (src/cli/instructions.rs predicate + report_change + run_artifact + unit tests) |
| A project without the arc42 glossary is not pointed at the domain skill | c58781b | Yes (coupling fixture; disable-arc42 contract) |
| The design-review skill checks that the ubiquitous language is followed | eab4d44 | Yes (assets/skills/givn-design-review/SKILL.md.tmpl) |
| The review skill checks that the ubiquitous language is followed | 162d200 | Yes (assets/skills/givn-review/SKILL.md.tmpl) |
| An agent synchronizes skills and the domain skill is routed (modified e2e) | 173e27c | Yes (tests/steps/*_e2e_steps.rs) |

8 distinct hashes, one per scenario, plus the plan-artifact commit (861a4c1)
and the finalisation chore (de53aae: ticks + regenerated agent skills). The
archive gate's hash-presence and duplicate checks pass. No task was ticked
without a production-touching commit.

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Domain-modeling skill template retargeted | Yes | assets/skills/givn-domain-modeling/SKILL.md.tmpl |
| Design-review template ubiquitous-language check | Yes | assets/skills/givn-design-review/SKILL.md.tmpl |
| Review template ubiquitous-language check | Yes | assets/skills/givn-review/SKILL.md.tmpl |
| Router predicate + report_change + run_artifact | Yes | src/cli/instructions.rs |
| Regular step definitions | Yes | tests/steps/fix_domain_modeling_skill_trigger_steps.rs |
| E2E step definitions | Yes | tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs |

**Strict-mode proof** — confirmed present in tests/features_runner.rs:

```
fail_on_skipped() is configured in tests/features_runner.rs. RED was proven
real for every scenario: each targeted run failed with
"Step panicked. Captured output: not implemented: <...>" (scenarios 1-3, 6-8)
or "Step doesn't match any function" (scenarios 4, 5 — new fixture undefined)
until implemented.
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps checked
for a real-interface primary assertion:

```
The modified @e2e sync scenario drives the real compiled givn binary as a
subprocess (givn skills sync) and asserts on the generated
.opencode/skills/givn-domain-modeling/SKILL.md file content. No
repository-only assertion. CLI-only capability; the real interface is
subprocess invocation + the generated file.
```

**E2e scope check** — one `@e2e` scenario per distinct happy-path action, not
per variant:

```
@e2e actions (4, unchanged from the capability): sync skills; request
next-step guidance; request specs instructions; request design instructions.
The lifecycle-breadth routing (waiting at the review stage) and the coupling
guard are regular scenarios on the same commands — they are variants, not new
interfaces; design.md commit to this split (line 128-133).
```

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`. Tree search (incl. untracked) found no other/parallel
e2e implementation for this capability.

```
N/A — single implementation; the two step files are the only step definitions
for this capability.
```

**Local environment check** — the local run command starts the full stack
cleanly:

```
N/A — CLI-only tool with no external dependencies. No local environment needed.
```

**E2e isolation check** — `verify.command` (`./run-tests.sh`) differs from
`verify.e2e_command` (`./run-tests.sh --e2e`); isolation proven by counts:

```
change-scoped:  verify = 9 scenarios, 31 steps | e2e = 1 scenario, 6 steps
(1 < 9: verify.e2e_command reports strictly fewer scenarios)
```

**Design.md conformance check** — implementation's actual commands, file
layout, and framework choices diffed against design.md:

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Skill template | assets/skills/givn-domain-modeling/SKILL.md.tmpl | same | Yes |
| Design-review / review templates | mandatory ubiquitous-language check | same, per chapter | Yes |
| Router predicate | `(domain_modeling, arc42) -> Option<&'static str>` | same, any workflow step, both addons on | Yes |
| report_change | gains `arc42: bool`; call sites pass `manifest.addons.arc42` | same (2 call sites in run_generic + run_artifact) | Yes |
| Step files | one file per capability | fix_domain_modeling_skill_trigger_steps.rs + _e2e_steps.rs | Yes |
| Review-stage fixture | write specs/design/design-review(DESIGN-REVIEW: PASS)/tasks(full checkboxes), leave review.md absent | same | Yes |
| Coupling-guard fixture | `givn addons disable arc42` after init, domain_modeling on | same | Yes |
| Strict mode | .fail_on_skipped() | same in tests/features_runner.rs | Yes |
| No new structs/deps | none | none added | Yes |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions:

| Inventory entry | @e2e scenario exists? | Driving mechanism | Match? |
|---|---|---|---|
| run `givn skills sync` to regenerate skills | An agent synchronizes skills and the domain skill is routed (modified) | Real givn subprocess + generated SKILL.md file assertions | Yes |
| request next-step guidance via `givn instructions` | An agent requests next-step guidance and is routed to the domain skill (permanent) | Real givn subprocess, stdout | Yes |
| request the specs instructions via `givn instructions specs --change <id>` | An agent requests the specs guidance and is routed to the domain skill (permanent) | Real givn subprocess, stdout | Yes |
| request the design instructions via `givn instructions design --change <id>` | An agent requests the design guidance and is routed to the domain skill (permanent) | Real givn subprocess, stdout | Yes |

The three non-sync rows are covered by the capability's existing permanent
`@e2e` scenarios, run unchanged post-merge; the delta adds no new `@e2e`
scenario (the lifecycle-breadth routing is a regular-scenario variant of the
next-step action, per design.md).

**Result:** CLEAN

---

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The ubiquitous language is recorded in the project's arc42 glossary (added) | The skill documents a per-change ubiquitous-language table (removed) | Supersession: same topic, durable-artifact upgrade (change-scoped `domain.md` -> project arc42 glossary). Removed+added pair explained; merge removes by title. |
| The next-step guidance names the domain skill during later change phases (added) | An agent requests next-step guidance and is routed to the domain skill (permanent) | Variant: same action (generic next-step) at a different lifecycle stage (awaiting-review vs awaiting-specs). Keep both — different stage, same interface. |
| The design-review skill checks that the ubiquitous language is followed (added) | The review skill checks that the ubiquitous language is followed (added) | Shape match but distinct templates and distinct artifacts (design-review.md vs review.md / sign-off gates). Keep both. |
| The ubiquitous language is recorded in the project's arc42 glossary (added) | The skill requires the arc42 glossary for domain modeling (added) | Both load the skill body, but assert distinct behaviours (record-in-glossary+no-domain.md vs enable-arc42-first requirement). Keep both. |
| The trigger loads during every workflow step of a change (modified scenario 1 step) | The trigger loads before writing feature specs or the design (removed permanent step) | Direct modification: the old phase-scoped trigger step is replaced by the whole-lifecycle step in the same scenario. Not an overlap — a rewrite. |
| An agent requests next-step guidance and is routed to the domain skill | The next-step guidance names the domain skill during later change phases | Variant: same action (generic next-step) at a different lifecycle stage — awaiting-specs vs awaiting-review. Keep both; the stage is what changed. |
| An agent requests the specs guidance and is routed to the domain skill | The next-step guidance names the domain skill during later change phases | Distinct entry points: artifact-scoped `givn instructions specs --change <id>` vs the generic review-stage next-step. Keep both. |
| An agent requests the design guidance and is routed to the domain skill | The next-step guidance names the domain skill during later change phases | Distinct entry points: artifact-scoped `givn instructions design --change <id>` vs the generic review-stage next-step. Keep both. |
| A project without the addon is not routed to the domain skill | A project without the arc42 glossary is not pointed at the domain skill | Distinct guards: the addon-off contract (domain_modeling disabled) vs the coupling contract (domain_modeling on but arc42 off). Same fixture family, different invariant. Keep both. |

## Split-or-keep

No long scenarios in the delta (max 6 steps across 3 templates/scenarios); no
split-or-keep rows required.

## Arc42 implementation conformance

`addons.arc42` is enabled. `arc42.md` impact assessment: all 12 rows are
marked `Affected? = No` — the change edits skill templates and the CLI
instruction router, and it makes the domain-modeling skill *instruct future
glossary maintenance*; it does not itself edit any durable `docs/arc42/`
chapter. The glossary chapter remains the target of the skill for later work,
but this change records no architecture fact in it.

| arc42 claim | Design treatment | Implementation evidence | Match? |
|---|---|---|---|
| No chapter affected (all No) | No durable chapter edits | No file under docs/arc42 changed (git diff confirms) | Yes |

**ARC42 CONFORMANCE:** CLEAN

---

## Coverage

Coverage addon is enabled (`givn/config.yaml`). Measurement commands run
change-scoped (the delta suite; the archive-level full-project measurement
runs post-merge at archive):

```
GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs ./measure-coverage.sh
  -> coverage/non-e2e-cobertura.xml + coverage/e2e-cobertura.xml
./merge-coverages.sh -> coverage/cobertura-coverage.xml (fresh, 2026-08-28 21:32)
```

Validity (from merge run):
- Overall line-rate: 0.28 (delta-scoped measurement — only the 9 delta
  scenarios ran; full-project measurement at archive is higher).
- Unknown/zero: both report files produced and distinct (non-e2e vs e2e).
- Known new production path non-zero: `domain_skill_to_load` /
  `report_change` / `run_artifact` in src/cli/instructions.rs — 100 lines
  covered by the delta scenarios; src/skills/render.rs 27 lines (generated
  skill path via skills sync).
- Merged report freshly produced by the configured merge command; not stale.

**Coverage measured:** YES — every new router branch is exercised by the delta
scenarios (coupling guard runs the arc42-off path; review-stage scenario runs
the lifecycle-wide path); no unclassified gap for this change.

---

## Test suite result

Change-scoped:
```
GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs ./run-tests.sh
[Summary]
1 feature
9 scenarios (9 passed)
31 steps (31 passed)
```

Change-scoped e2e:
```
GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs ./run-tests.sh --e2e
[Summary]
1 feature
1 scenario (1 passed)
6 steps (6 passed)
```

Permanent (regression) — pre-merge, 2 expected failures; both are the exact
assertions the delta's `@givn.modified` scenarios replace at merge:
```
./run-tests.sh
[Summary]
397 scenarios (395 passed, 2 failed) / 2415 steps (2413 passed, 2 failed)
  ✘ "the trigger loads before writing feature specs or the design"
  ✘ "the generated skill loads before writing feature specs or the design"
```
Both failing steps reference the phase-scoped "before writing .feature specs
or design.md" phrase that scenario 1 and the sync e2e scenario of this change
delete from the template. That phrase no longer exists in the template; at
merge, the modified scenarios replace the permanent ones with the
whole-lifecycle steps, removing both references. The archive gate runs
post-merge `verify` and `verify-e2e` against the merged tree (archive.rs
210-216), so the merged permanent suite is re-validated at archive.

Additional: `cargo test --lib` -> 166 passed; `cargo clippy --workspace
--all-targets -- -D warnings` -> clean; `cargo fmt --all` -> clean;
`givn lint --change` -> clean. Live router (installed binary, temp project):
arc42 off -> no skill pointer; arc42 on -> skill named at specs and at the
review stage.

**Result:** PASS (change-scoped; permanent re-verified post-merge by the
archive gate)

---

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, placeholder step has a
      real assertion body, all commits verified, all promised components
      exist, no downgraded e2e scenario, no e2e scenario driven via
      HTTP/fetch() in place of the real CLI interface, no over-produced e2e
      scope).
- [x] All change-scoped scenarios pass (`./run-tests.sh` and
      `./run-tests.sh --e2e` exit 0 for the change).
- [x] Every coverage gap is classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and the
      tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (9/31 vs 1/6 change-scoped).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed, including both new fixtures exactly as specified.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section is present and accurate
      (`domain_skill_to_load` new signature, `report_change` arc42 param, no
      new structs).
- [x] arc42 conformance verified: no affected chapters, no durable chapter
      edits; recorded CLEAN.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
