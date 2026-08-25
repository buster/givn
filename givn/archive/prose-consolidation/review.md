# Review: prose-consolidation

## Fabrication Audit

### Step and Task Integrity

- Step file scanned: `tests/steps/prose_consolidation_steps.rs`.
- It contains real subprocess setup and concrete assertions; no empty bodies,
  `pass`, bare `return`, or placeholder implementation remains.
- The file is registered in `tests/steps/mod.rs` and matches design.md.
- Strict-mode proof is recorded in tasks.md and exits non-zero on an undefined
  step through `.fail_on_skipped()`.
- Scenario commits:
  - `7187b03` override subprocess assertions and step registration;
  - `5e38ef1` canonical flag-variant scenario;
  - `80b6575` design/review delegation scenario;
  - `cf63571` canonical specs policy and first E2E scenario;
  - `ad5a962` generated guidance, templates, commands, skills, Arc42 updates,
    and second E2E scenario;
  - `809f1fc` final regular-scenario lifecycle tag cleanup.
- The asset commits intentionally modify embedded production inputs; the new
  step file provides the executable black-box glue. No component named by
  design.md is missing.

### E2E Integrity and Interaction Coverage

- The delta has exactly two `@e2e` scenarios, one per normalized inventory
  action. The override is a regular configuration variant of the first action.
- No `@e2e` tag was removed.
- `verify.command` is `./run-tests.sh`; `verify.e2e_command` is
  `./run-tests.sh --e2e`. They are distinct and the permanent baseline counts
  are 163 non-E2E versus 22 E2E scenarios.
- The E2E command invokes `tests/features_runner.rs` through `run-tests.sh`.
  No parallel prose-consolidation E2E implementation exists.
- The real interface is the CLI subprocess. The two E2E scenarios assert CLI
  stdout first; generated files are the secondary result of `skills sync`.
- All three configured skill targets (`opencode`, `agents`, `claude`) are
  checked; the opencode command target is checked separately.

| Inventory entry | Matrix row | Scenario | Driver | Result |
|---|---|---|---|---|
| Run `givn instructions specs --change prose-consolidation` to read the canonical authoring policy | Yes | Canonical E2E policy is emitted by specs instructions | Real `givn instructions` subprocess | Clean |
| Run `givn skills sync` to receive generated guidance that follows the canonical policy | Yes | Generated guidance points to the canonical E2E policy | Real `givn skills sync` subprocess, then emitted target files | Clean |

The regular override scenario is explicitly not an inventory entry because it
is a configuration variant of the first action.

### Asset Parity Audit

The complete allowlist from design.md was scanned:

- `assets/instructions/{specs,design,review,tasks}.md`;
- `assets/skills/{givn-spec,givn-design,givn-review,givn-steps,givn-tasks,givn-implement,givn-design-review}/SKILL.md.tmpl`;
- `assets/commands/{givn-spec,givn-design,givn-review,givn-tasks,givn-steps,givn-implement,givn-design-review,givn-propose}.md`;
- `assets/templates/{spec.feature,design.md}`.

The case-insensitive legacy wording scan returned clean. Every non-canonical
asset delegates to `givn instructions specs`; generated skills were verified
across all three configured targets and the opencode command was verified.

## Verification Evidence

### Active Change

```text
givn lint --change prose-consolidation
givn lint: 1 file(s) checked — clean

GIVN_FEATURES=givn/changes/prose-consolidation/specs ./run-tests.sh
[Summary]
1 feature
3 scenarios (3 passed)
13 steps (13 passed)

GIVN_FEATURES=givn/changes/prose-consolidation/specs ./run-tests.sh --e2e
[Summary]
1 feature
2 scenarios (2 passed)
17 steps (17 passed)
```

All five scenarios were also run individually with the anchored Cucumber name
filter. Every targeted run passed in both verification passes.

### Permanent Baseline

```text
./run-tests.sh
20 features
163 scenarios (163 passed)
926 steps (926 passed)

./run-tests.sh --e2e
15 features
22 scenarios (22 passed)
127 steps (127 passed)
```

Archive reruns both commands after the two E2E scenarios merge; the expected
post-archive counts are 163 non-E2E and 24 E2E scenarios.

### Coverage

```text
./measure-coverage.sh
./merge-coverages.sh

coverage/cobertura-coverage.xml:
lines-covered=5156 lines-valid=8154 line-rate=0.6323276919303409
branches-covered=0 branches-valid=0 branch-rate=0.0
```

The merged report is fresh and includes the Gherkin runner plus the real givn
subprocess. Existing production paths in `src/config/cascade.rs`,
`src/skills/render.rs`, and `src/cli/instructions.rs` are exercised with
non-zero line coverage. No production logic lines were added; there are no new
uncovered production gaps to classify. Test glue and Markdown assets do not
constitute production coverage gaps.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. The implementation matches the change-level marker
and durable chapters:

| Chapter/fact | Durable source | Marker | Design/tasks | Implementation evidence | Result |
|---|---|---|---|---|---|
| Agent-authoring consistency goal | ch.01 FR-22 and secondary goals | Yes | Canonical policy and parity scenarios | Canonical output and parity scan | Clean |
| Guidance CLI surface | ch.03 interface table | Yes | `givn instructions specs` | Scenario 1 real subprocess | Clean |
| Asset/cascade responsibilities | ch.05 module table | Yes | Allowlist and existing cascade | Assets and `skills sync` | Clean |
| Normative policy cross-cutting rule | ch.08 §8.3a | Yes | Single source/delegation | All generated guidance checks | Clean |
| ADR-038 | ch.09 ADR-038 | Yes | Canonical source decision | Asset implementation and scenarios | Clean |
| Quality scenarios Q-27/Q-28 | ch.10 | Yes | Canonical, parity, override scenarios | Five feature scenarios | Clean |
| Drift risk R-17 | ch.11 | Yes | Full allowlist scan and target checks | Review audit | Clean |
| Canonical authoring policy term | ch.12 | Yes | Canonical terminology | Design/proposal use | Clean |

ARC42 CONFORMANCE: CLEAN

## Coverage Classification

No new executable production lines are uncovered. The only new implementation
surface is embedded guidance plus test glue, and all new observable paths are
covered by the five Gherkin scenarios. No dead production code or legitimately
hard-to-test production boundary was introduced.

## Sign-Off

- [x] Fabrication audit clean.
- [x] Strict mode proven.
- [x] All tasks have implementation/evidence commits.
- [x] Active non-E2E and E2E suites pass.
- [x] Permanent baseline suites pass.
- [x] Coverage measured and merged.
- [x] Asset parity scan clean.
- [x] Arc42 conformance clean.

## Overlap dispositions

These shape matches are retained because each scenario checks a distinct
configuration or authoring outcome; no scenario is merged or removed.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Canonical E2E policy is emitted by specs instructions` | `Project override replaces the canonical specs instruction` | boundary: default resolution versus explicit project configuration |
| `Canonical E2E policy is emitted by specs instructions` | `Canonical policy classifies output-affecting flags as variants` | boundary: canonical policy output versus flag-variant rule |
| `Project override replaces the canonical specs instruction` | `Canonical policy classifies output-affecting flags as variants` | boundary: configuration override versus policy classification |

## Post-Archive Verification

- [x] Archive completed and merged the five scenarios into
      `givn/specs/prose-consolidation/prose-consolidation.feature`.
- [x] Permanent target contains no `@wip` tags.
- [x] Post-merge full suite: 166 scenarios and 939 steps passed.
- [x] Post-merge E2E suite: 24 scenarios and 144 steps passed.
- [x] Coverage merge published `5159/8154` lines and `0/0` branches.

REVIEW: PASS
