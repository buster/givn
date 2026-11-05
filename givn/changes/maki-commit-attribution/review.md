Review kind: self-review
Author: maki agent (maki-commit-attribution author)
Reviewer context: maki agent (maki-commit-attribution author)

# Review: maki-commit-attribution

## Scope

The delta adds the generated Maki commit-attribution integration and its
permission grant: four prepare/partial scenarios, five initialization and
manifest scenarios, and four end-to-end scenarios. The review covers the delta
features, the generated Maki asset and manifest, the generator and merge path,
the installation/upgrade reports, and the documentation updates.

## Fabrication audit

**Step definition scan.** The changed step files contain no empty bodies,
`unimplemented!()`, or `todo!()`:

```text
$ grep -rnE '^\s*\{\s*\}\s*$|unimplemented!|todo!' \
    tests/steps/llm_attribution_e2e_steps.rs \
    tests/steps/development_timeline_support.rs \
    tests/steps/initialize_a_project_steps.rs \
    tests/steps/llm_attribution_steps.rs \
    tests/steps/llm_attribution_installation_e2e_steps.rs \
    tests/steps/agent_activity_reporting_steps.rs
no empty bodies or stubs in the changed step files
```

**Commit cross-check.** Every checked scenario records a batch revision:

| Scenario | Commit | Evidence in the revision |
|---|---|---|
| Maki prepares attribution for a revision command | `a022d6bf` | Maki driver, prepared-command steps, module asset (asset shipped in `b44ef4ee`) |
| Maki leaves a command that creates no revision unchanged | `a022d6bf` | driver and non-revision step |
| Maki marks a partial attribution context as unavailable | `a022d6bf` | driver and partial-context step |
| Maki marks a missing reasoning mode as unavailable | `a022d6bf` | driver and reasoning-only step |
| The generated Maki permission manifest names the session read | `b44ef4ee` | manifest asset and manifest assertions |
| Initializing a Maki target installs the attribution integration | `b44ef4ee` | `MAKI_INTEGRATIONS`, fences, module asset |
| Synchronizing an existing Maki manifest adds the reporter and attribution grants | `b44ef4ee` | grant merge, legacy fixture |
| Synchronizing an unmodified generated Maki manifest grants attribution | `b44ef4ee` | prior-manifest recognition and replacement |
| Synchronizing an edited Maki manifest preserves its attribution revocation and names the needed grant | `b44ef4ee` | warning path and edited fixture |
| Consumer initializes a project and receives shared attribution plumbing | `e5d7601b` | shared integration list and Maki target |
| Consumer sees the commit attribution installation report | `e5d7601b` | installation report path list |
| Consumer upgrades an existing project with attribution plumbing missing | `e5d7601b` | upgrade fixture and managed-revision list |
| Maki attributes a commit using its current model | `e5d7601b` | real-Maki e2e driver and Git assertions |

**Promised components.**

| Component from design.md | Exists | Path |
|---|---|---|
| Maki attribution module | yes | `assets/agent-integrations/maki/givn-attribution.lua` |
| Prior generated manifest recognition key | yes | `assets/agent-integrations/maki/plugin-before-attribution.toml` |
| Current permission manifest with `session_write` | yes | `assets/agent-integrations/maki/plugin.toml` |
| `MAKI_INTEGRATIONS` includes the module | yes | `src/skills/targets.rs` |
| Attribution fence and atomic fence merge | yes | `src/skills/targets.rs` |
| Exact-prior-manifest migration and warning | yes | `src/skills/targets.rs` |
| Initialization report line | yes | `src/cli/init.rs` |
| Strict-rule ignore/discovery paths | yes | `src/skills/rules.rs` |
| Maki driver and step definitions | yes | `tests/steps/development_timeline_support.rs`, `tests/steps/llm_attribution_e2e_steps.rs` |
| Manifest fixtures and fence assertion | yes | `tests/steps/initialize_a_project_steps.rs` |

**Strict mode.** `tests/features_runner.rs` builds with `.fail_on_skipped()` and
exits nonzero on failed or skipped scenarios. The setup task records the
undefined-step probe: the runner exited 1 with `Step doesn't match any
function` and `1 scenario (1 failed)`.

**E2E fidelity.**

| @e2e scenario | Real interface assertion | Downgraded |
|---|---|---|
| Consumer initializes a project and receives shared attribution plumbing | real `givn init` subprocess; generated files and hook launcher asserted | no |
| Consumer sees the commit attribution installation report | real `givn init` stdout asserted for every selected integration | no |
| Consumer upgrades an existing project with attribution plumbing missing | real `givn upgrade`; managed revision paths asserted | no |
| Maki attributes a commit using its current model | real Maki 0.7.0 prepares the command, a real shell runs it, and Git's committer/message are read back | no |

No `@e2e` tag was removed. The delta keeps four e2e scenarios and the
canonical real-interface assertions.

## Verification evidence

- Change-scoped non-e2e: 9/9 passed (`./run-tests.sh --non-e2e` with
  `GIVN_FEATURES=givn/changes/maki-commit-attribution/specs`).
- Change-scoped e2e: 4/4 passed (`./run-tests.sh --e2e` with the same
  `GIVN_FEATURES`).
- `givn check tasks --change maki-commit-attribution`: green, change
  verification receipt for 9 scenarios at `5bcb413c`.
- Coverage: `./measure-coverage.sh` with the change features produced distinct
  `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`;
  `./merge-coverages.sh` exited 0.
- `givn check review --change maki-commit-attribution`: static review gates,
  overlap dispositions, routing, and receipt are green after the design
  Step Definitions table and the capability routing row were corrected.
- Full permanent suite: the only failure is the stale pre-merge
  `The generated Maki permission manifest names the session read` scenario,
  which the delta modifies and the archive merge replaces; the change-scoped
  suite that runs that modified scenario is green. The archive gate runs the
  merged full suite.

## Coverage disposition

Every delta scenario has an implementation or regression test in its recorded
revision. No uncovered behavior was left unclassified:

- The four prepare/partial scenarios are covered by the real-Maki driver and
  the prepared-command assertions.
- The five installation/migration scenarios are covered by the generated asset,
  the merge functions, the exact-prior-manifest path, and the warning path.
- The four e2e scenarios are covered by the real CLI, upgrade fixture, and
  real-Git commit.
- The retired `Synchronizing an existing Maki manifest adds only the session
  read grant` scenario is replaced by the reporter/attribution grant scenario;
  its user-key preservation and no-new-revocation checks remain in the
  replacement.

## Findings

1. **Pre-existing interaction inventory mismatch (advisory, out of scope).**
   `initialize-project/usecase.md` declares
   `Consumer runs init non-interactively and gets opencode skills`, while the
   permanent feature carries that scenario without an `@e2e` tag. The design
   matrix records it as an existing corpus mismatch; this change does not
   remove or add that tag. No action in this change.
2. **Focused-session model for a subagent (accepted limitation).**
   `maki.model.get()` reports the focused session. A subagent committing under
   a different model is attributed to the focused session. The proposal,
   design, and R-134 record the limitation; the top-level commit path is
   covered by the e2e scenario.
3. **Headless Maki has no model identity (accepted best effort).**
   A headless run stamps `maki` with `unavailable` LLM and reasoning facts and
   no committer override. The command is still attributed to the harness and a
   commit is not blocked.

No blocking finding remains.

## Persona disposition

- `developer--primary-user`: relevant. Reads the installation report and owns
  the permission manifest; evidence: installation report and edited-manifest
  warning scenarios.
- `agent--technical-role`: relevant. Issues the prepared Maki command and the
  attributed commit; evidence: prepare and attributed-commit scenarios.
- `external-contributor--open-source-consumer`: not relevant. The integration
  is local-only generated infrastructure with no published consumer surface.
- `givn-maintainer--owner`: not a confirmed Persona for this change; the
  ADR-013 amendment and the risk record were reviewed directly.

## Interface classification

`Interface: none` is correct: the change adds generated files and one stdout
report line. No HTML, CSS, or JavaScript is rendered, so the opposite browser
classification does not hold. The CLI report is covered by the existing
terminal transcript in the installation scenario.

PERSONAS: developer--primary-user

README-IMPACT: none

VERDICT: PASS

REVIEW: PASS
