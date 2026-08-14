# 12. Glossary

| Term | Definition |
| --- | --- |
| **Addon** | A pre-packaged artifact (or set) shipped embedded but off by default; enabled via the `addons:` overlay sugar (e.g. `arc42`). givn's extension model — no separate plugin runtime. |
| **Agent target** | One of the agent directories givn can install generated skills/commands to: `opencode`, `agents` (`.agents/`), `claude` (`.claude/`). The enabled set is stored in `skills.targets` in `givn/config.yaml`; default `[opencode]`. |
| **arc42-docs** | The bundled addon artifact that creates and maintains durable project-level arc42 documentation (default after `design`, output `docs/arc42/`). |
| **Artifact** | A produced document in the lifecycle (proposal, spec, design, tasks, review). Each has facets: a template, an instruction, and dependencies. May be `optional` and is fully configurable via overlay. |
| **Capability** | A named unit of behaviour, kebab-case (e.g. `user-auth`), corresponding to one `.feature` file. |
| **Cascade** | The CSS-style resolution where a project override file wins over the embedded default, per facet. |
| **Change** | A unit of work moving through the lifecycle, living in `givn/changes/<id>/` and ending in `givn/archive/<id>/`. |
| **Delta** | A `.feature` file tagged `@givn.delta` describing additions/modifications/removals to be merged into the permanent spec. |
| **Disposition table** | The review.md table in which every shape match involving the change's delta is classified (`merge`/`delete`/`boundary`/`related`); the review gate blocks until it is complete. See ADR-037. |
| **Fingerprint** | The deterministic, masked assertion sequence plus extracted facts (exit code, contains polarity, data-table contents, `[DONE]`, TTY) of a scenario; the basis of shape-match and subset detection. See ADR-037. |
| **Net-delta receipt** | The review output counting added/modified/removed scenarios of a change (prose + JSON line), acknowledging the net effect of removed+added pairs. |
| **Eject** | Materialise an embedded default into the project `givn/` dir so it can be edited (`givn eject`). |
| **Facet** | The smallest overridable unit of an artifact: template, instructions, or deps. |
| **Feature file** | A Gherkin `.feature` file. In givn it IS the specification (no `spec.md`). |
| **Manifest** | The YAML document defining the artifact graph, `apply`, and `verify`. Embedded default + project overlay. |
| **Overlay / patch** | The project's `givn/config.yaml`, merged onto the embedded default manifest (add/modify/remove). |
| **Permanent spec** | The merged, tag-stripped `.feature` under `givn/specs/` — the source of truth after archive. |
| **Scenario** | A Gherkin scenario (Given/When/Then). The unit of the TDD loop. |
| **Shape match** | Two scenarios whose masked assertion sequences are equal but whose titles differ; a lint warning that requires a disposition at review unless their facts contradict. See ADR-037. |
| **Skill** | A `SKILL.md` (frontmatter + markdown) that an agent can load to perform a workflow action. |
| **TDD loop** | Scenario-by-scenario RED (failing assertions) → GREEN (minimum code) → REFACTOR. |
| **Hook** (`post`) | A command an artifact runs after completion: built-in `verify` (runs the test runner) or `lint`, or a shell command. Run by `givn check` and as the `givn archive` gate; failure blocks (unless `allow_failure`). See ADR-028, ch.8.12. |
| **Check** (`givn check`) | Built-in command that runs an artifact's `post` hooks (e.g. the test suite via `verify.command`). Exits non-zero if a required hook fails. |
| **Lint** (`givn lint`) | Built-in command that statically checks `.feature` syntax, `@wip`/PENDING, and delta tags. Exit 0/1/2. **Runs no tests.** (Renamed from `givn verify` — ADR-027.) |
| **Test runner** | The command that actually executes step definitions against code, configured in `verify.command`. Invoked by the agent/CI, not by givn. Confirms GREEN. |
| **`verify.command`** | The manifest config key holding the test-runner command (default sentinel `givn missing-testrunner`). Read after overlay resolution; written by `init`/`upgrade`; invoked by agent/CI. |
| **Coverage measurement command** | The project-owned command in `coverage.measure_command` that runs the configured test subsets and writes the non-E2E and E2E source coverage reports. |
| **Coverage merge command** | The project-owned command in `coverage.merge_command` that combines the two source reports into `coverage.output_path`. Its fresh output is authoritative for README coverage. |
| **Merged coverage report** | The Cobertura XML emitted by the coverage merge command. givn validates this report and uses its line and branch counters for both README badges and the managed summary. |
| **Coverage badge block** | The managed `coverage-badge` README block directly below the title, containing line and branch badges linked to the merged report. |
| **`@givn.delta`** | Feature-level tag marking a delta document. |
| **`@givn.added` / `@givn.modified` / `@givn.removed`** | Scenario-level merge directives. |
| **`@wip`** | Scenario tag: step definitions not yet implemented. |
| **`GIVN_FEATURES`** | Env var overriding the feature path for the test runner (default `givn/specs`). |
| **arc42** | The architecture documentation template this `docs/arc42/` folder follows. |
| **Arc42 implementation conformance** | The completion-review comparison of durable Arc42 facts and the change-level assessment against the design, mapped tasks, and completed implementation. Required only when the `arc42` addon is enabled. |
| **Architecture fact** | A documented architectural constraint, decision, structure, flow, quality requirement, risk, or domain term that a change may need to preserve or update. |
| **ADR** | Architecture Decision Record (chapter 9). |
| **OpenSpec** | The TypeScript tool whose fork prototyped this workflow; givn is a clean-break successor. |
