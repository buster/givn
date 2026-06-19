# 10. Quality Requirements

## 10.1 Quality Tree

```
givn quality
├── Configurability (Goal 1)
│   ├── Per-facet override
│   ├── Manifest overlay
│   └── No source fork required
├── Installability (Goal 2)
│   ├── No npm / no root / no runtime
│   ├── Multiple channels (release, curl|sh, brew, cargo)
│   └── Multi-platform
├── Upgradeability (Goal 3)
│   ├── Project files never clobbered
│   └── Defaults live only in the binary
├── Correctness
│   ├── Gherkin merge integrity (added/modified/removed)
│   ├── No partial writes on archive
│   └── Accurate filesystem-derived state
└── Agent ergonomics
    ├── Standards-aligned (AGENTS.md, .agents/skills)
    └── Machine-readable instructions (--json)
```

## 10.2 Quality Scenarios

| ID | Quality | Scenario | Expected response |
| --- | --- | --- | --- |
| Q-1 | Configurability | A user wants a different `design.md` template but the default instructions. | `givn eject design:template`; edit the file; `givn instructions design` still returns the embedded instruction; the ejected template is used. |
| Q-2 | Configurability | A user adds a new artifact `risk-assessment` to the graph. | Add it via overlay in `givn/config.yaml`; it appears in the lifecycle and `givn instructions risk-assessment` works. |
| Q-3 | Installability | A user with no Node and no root wants to install. | `curl \| sh` places the binary in `~/.local/bin`; it runs without any runtime. |
| Q-4 | Upgradeability | A user upgrades the binary after ejecting two templates. | The two ejected files are untouched; non-ejected facets use the new embedded defaults. |
| Q-5 | Upgradeability | After upgrade, generated skills are stale. | `givn skills sync` regenerates AGENTS.md + skills from the new templates. |
| Q-6 | Correctness | An archive includes a `@givn.modified` scenario whose title is missing in the permanent spec. | Archive aborts with a clear error; no files are modified. |
| Q-7 | Correctness | An archive merges added/modified/removed across several `.feature` files. | Permanent specs updated correctly; all `@givn.*` tags stripped; result is valid Gherkin. |
| Q-8 | Correctness | CI runs `givn verify` on specs containing a `@wip` scenario. | Exit code 2; CI can distinguish wip from parse error (1) and clean (0). |
| Q-9 | Correctness | State is queried mid-change. | `givn status` reports the correct stage purely from file presence, even after manual edits. |
| Q-10 | Agent ergonomics | An agent requests instructions for the spec artifact. | `givn instructions specs --change <id> --json` returns id, generates, requires, instruction. |
| Q-11 | Configurability | A fresh project tries to run tests without configuring a runner. | The sentinel `givn missing-testrunner` prints per-language setup guidance and exits non-zero; nothing falsely reports GREEN. |
| Q-12 | Configurability | A user opens `givn/config.yaml` to discover what they can change. | Every key is present (commented) with description, default, and allowed values; no external docs needed. |
| Q-13 | Upgradeability | A project made with 1.0.0 runs `givn upgrade` after installing 1.1.0. | Managed comment scaffold is refreshed, user overrides preserved, `givn_config_version` bumped to 1.1.0; changes reviewable via `git diff`. |
| Q-14 | Upgradeability | `givn upgrade` is run twice on the same binary. | Second run produces no diff (idempotent); no user value is ever deleted. |

## 10.3 Acceptance signals per phase

- **Phase 1–2:** cascade + overlay unit tests pass; `givn init`/`eject` work.
- **Phase 3:** Gherkin merge round-trip tests pass; verify exit codes correct.
- **Phase 4:** lifecycle commands produce correct state and archive results.
- **Phase 5:** AGENTS.md + three skill targets generated and re-syncable.
- **Phase 6:** binaries build for all targets; installer works on a clean machine.
- **Phase 7:** full loop validated end-to-end by dogfooding on gherkintest.
