# 12. Glossary

| Term | Definition |
| --- | --- |
| **Addon** | A pre-packaged artifact (or set) shipped embedded but off by default; enabled via the `addons:` overlay sugar (e.g. `arc42`). givn's extension model — no separate plugin runtime. |
| **arc42-docs** | The bundled addon artifact that creates and maintains durable project-level arc42 documentation (default after `design`, output `docs/arc42/`). |
| **Artifact** | A produced document in the lifecycle (proposal, spec, design, tasks, review). Each has facets: a template, an instruction, and dependencies. May be `optional` and is fully configurable via overlay. |
| **Capability** | A named unit of behaviour, kebab-case (e.g. `user-auth`), corresponding to one `.feature` file. |
| **Cascade** | The CSS-style resolution where a project override file wins over the embedded default, per facet. |
| **Change** | A unit of work moving through the lifecycle, living in `givn/changes/<id>/` and ending in `givn/archive/<id>/`. |
| **Delta** | A `.feature` file tagged `@givn.delta` describing additions/modifications/removals to be merged into the permanent spec. |
| **Eject** | Materialise an embedded default into the project `givn/` dir so it can be edited (`givn eject`). |
| **Facet** | The smallest overridable unit of an artifact: template, instructions, or deps. |
| **Feature file** | A Gherkin `.feature` file. In givn it IS the specification (no `spec.md`). |
| **Manifest** | The YAML document defining the artifact graph, `apply`, and `verify`. Embedded default + project overlay. |
| **Overlay / patch** | The project's `givn/config.yaml`, merged onto the embedded default manifest (add/modify/remove). |
| **Permanent spec** | The merged, tag-stripped `.feature` under `givn/specs/` — the source of truth after archive. |
| **Scenario** | A Gherkin scenario (Given/When/Then). The unit of the TDD loop. |
| **Skill** | A `SKILL.md` (frontmatter + markdown) that an agent can load to perform a workflow action. |
| **TDD loop** | Scenario-by-scenario RED (failing assertions) → GREEN (minimum code) → REFACTOR. |
| **Verify** | `givn verify`: lints `.feature` syntax and `@wip` only. Exit 0/1/2. Does not run tests. |
| **`verify.command`** | The manifest-declared test-runner command that executes step definitions. |
| **`@givn.delta`** | Feature-level tag marking a delta document. |
| **`@givn.added` / `@givn.modified` / `@givn.removed`** | Scenario-level merge directives. |
| **`@wip`** | Scenario tag: step definitions not yet implemented. |
| **`GIVN_FEATURES`** | Env var overriding the feature path for the test runner (default `givn/specs`). |
| **arc42** | The architecture documentation template this `docs/arc42/` folder follows. |
| **ADR** | Architecture Decision Record (chapter 9). |
| **OpenSpec** | The TypeScript tool whose fork prototyped this workflow; givn is a clean-break successor. |
