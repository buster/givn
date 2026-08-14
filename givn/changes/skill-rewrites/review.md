# Review: skill-rewrites

## Fabrication Audit

- Step files scanned: `tests/steps/skill_rewrites_steps.rs` and reused
  `tests/steps/prose_consolidation_steps.rs`; no empty/no-op bodies exist.
- Strict mode proof is present in tasks.md and the undefined-step probe exits
  non-zero through `.fail_on_skipped()`.
- All four scenarios have commits:
  - `433cdb8` review disposition choreography and step registration;
  - `20423bd` Black-Box-First design/step guidance;
  - `77534f5` deterministic pre-retrieval spec guidance;
  - `7e3f470` synchronized generated gate guidance.
- The commits touch embedded implementation assets and executable test glue;
  no new runtime component was promised.
- The delta contains one `@e2e` scenario for its one normalized inventory
  action. No E2E tag was removed.
- `verify.command` is `./run-tests.sh`; `verify.e2e_command` is
  `./run-tests.sh --e2e`. Active counts are 3 versus 1; permanent counts are
  166 versus 24. The E2E subset is strictly smaller.
- The real interface is the `givn` CLI subprocess. The E2E Then assertions
  include the sync command output before generated-file checks.
- No browser, HTTP, database, queue, external service, or digital twin is
  involved. No parallel E2E implementation exists.

### Interaction Coverage

| Inventory entry | Matrix row | @e2e scenario | Driver | Result |
|---|---|---|---|---|
| Run `givn skills sync` to receive workflow guidance for deterministic gates | Yes | Synchronized guidance explains how to resolve the deterministic gates | Real `givn skills sync` subprocess | Clean |

The three instruction scenarios are regular variants/outputs of the same
guidance workflow and are not extra inventory actions.

## Verification

### Active Change

```text
givn lint --change skill-rewrites
givn lint: 1 file(s) checked — clean

GIVN_FEATURES=givn/changes/skill-rewrites/specs ./run-tests.sh
1 feature
3 scenarios (3 passed)
13 steps (13 passed)

GIVN_FEATURES=givn/changes/skill-rewrites/specs ./run-tests.sh --e2e
1 feature
1 scenario (1 passed)
9 steps (9 passed)
```

### Permanent Suite

```text
./run-tests.sh
21 features
166 scenarios (166 passed)
939 steps (939 passed)

./run-tests.sh --e2e
16 features
24 scenarios (24 passed)
144 steps (144 passed)
```

### Retrieval Boundary

The positive-command scan for `givn spec index` and `givn spec search` is clean.
The only remaining mentions are negative guardrails stating that retrieval is
not available until the later retrieval change.

### Coverage

```text
./measure-coverage.sh
./merge-coverages.sh
coverage/cobertura-coverage.xml:
lines-covered=5159 lines-valid=8154 line-rate=0.6326956095168016
branches-covered=0 branches-valid=0 branch-rate=0.0
```

Coverage is fresh and includes the Gherkin runner plus the real CLI subprocess.
The exercised asset-resolution and skills-sync paths have non-zero production
coverage. No new production logic or uncovered production boundary was added.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. This change updates only asset prose on top of the
canonical architecture documented by Change 3; all twelve rows in arc42.md
correctly state no architectural impact. Existing durable chapters remain
substantive and unchanged by this choreography-only change.

| Chapter/fact | Marker | Design/tasks | Implementation evidence | Result |
|---|---|---|---|---|
| All 12 chapters | No affected chapters | Asset-only rewrite, existing cascade | Embedded skills/commands only | Clean |

ARC42 CONFORMANCE: CLEAN

## Coverage Classification

No new production runtime lines were introduced. All new guidance paths are
covered by the four feature scenarios. There is no dead production code,
missing production test, or legitimately hard-to-test production boundary.

## Sign-Off

- [x] Fabrication audit clean.
- [x] Strict mode proven.
- [x] All tasks have commits and evidence.
- [x] Active suites pass.
- [x] Permanent suites pass.
- [x] Retrieval boundary scan clean.
- [x] Coverage measured and merged.
- [x] Arc42 conformance clean.

## Overlap dispositions

The deterministic gate reports shared assertion shapes between the new
pre-retrieval guidance scenario and existing prose-consolidation scenarios.
They remain separate because they verify different policy phases and outcomes.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Canonical E2E policy is emitted by specs instructions` | `Spec instructions use deterministic gates before retrieval exists` | boundary: canonical policy versus pre-retrieval gate boundary |
| `Project override replaces the canonical specs instruction` | `Spec instructions use deterministic gates before retrieval exists` | boundary: project override versus retrieval-phase policy |
| `Canonical policy classifies output-affecting flags as variants` | `Spec instructions use deterministic gates before retrieval exists` | boundary: action normalization versus retrieval-phase policy |

REVIEW: PASS
