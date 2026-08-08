# 11. Risks and Technical Debt

## 11.1 Risks

| ID | Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- | --- |
| R-1 | The `gherkin` crate's AST does not expose enough position info to losslessly merge + strip tags while preserving formatting. | Medium | High | **Resolved (Phase 3):** merge operates on a line-based text model, not a full AST rewrite. This preserves formatting and comments correctly. |
| R-2 | Overlay/patch semantics for the manifest become ambiguous (deep-merge edge cases). | Medium | Medium | **Resolved (Phase 2):** overlay rules specified and unit-tested for add/modify/remove/cycle/dangling-requires. |
| R-3 | `serde_yaml_ng` diverges or is itself abandoned. | Low | Medium | Thin wrapper around (de)serialisation; `serde_norway` is a fallback fork. |
| R-4 | Skill/agent conventions shift (the `.agents/skills` convention is emerging, not ratified). | Medium | Low | Templates are ejectable and targets are pluggable; adding/altering a target is cheap. |
| R-5 | Cross-platform static linking (musl, Windows) causes build friction. | Medium | Medium | Use `cargo-dist`; pin toolchains; test in CI before first release. |
| R-6 | Merge corrupts the permanent spec on a partial failure. | Low | High | All-or-nothing archive: validate every precondition before any write; write atomically. |
| R-7 | Scope creep back toward OpenSpec's dropped features (profiles, context stores). | Medium | Medium | Chapter 3.3 non-goals are explicit; keep them out of v1. |
| R-8 | Filesystem-derived state misreads partially-written or manually-edited dirs. | Low | Medium | Derive conservatively; treat ambiguous states as "in progress"; never auto-destroy on inference. |
| R-9 | Path traversal in `--change` parameter (e.g. `--change "../etc"`). | Low | High | **Resolved:** every command that accepts `--change` now validates the id as kebab-case via the shared `resolve_change_dir` helper before constructing any filesystem path. Invalid ids are rejected with a clear error. |

## 11.2 Deliberate (planned) technical debt

| ID | Debt | Reason | Repayment |
| --- | --- | --- | --- |
| D-1 | No `givn migrate` from `openspec/`. | MVP focus; clean break. | Add as a later command (ADR-017). |
| D-2 | Only `.agents` + claude + opencode skill targets shipped. | Cover the common cases first. | Users eject/retarget now; add built-ins later. |
| D-3 | No CI/remote initially (local git only). | OC-1. | Wire up `cargo-dist` pipeline when a remote is added. |
| D-4 | Overlay may start with replace-only (not deep-merge) for some fields. | Get a working MVP. | **Partially resolved:** field-level deep-merge implemented for artifacts. Richer nested merge can be added as needs surface. |
| D-5 | `givn new` writes the proposal template verbatim with no `{{CHANGE_ID}}` substitution, so the scaffolded `proposal.md` keeps the literal placeholder text. | Discovered during reverse-engineering characterization of `new-command`; AS-IS behaviour is captured by spec, not fixed. | Future `@givn.modified` delta to substitute the id (or drop the placeholder from the embedded template). Tracked as BUG-1 in change `reverse-engineer-new-command`. |
| D-6 | `givn new` checks "change already exists" before "givn/ not found"; the already-exists branch is unreachable when `givn/` is absent (its parent cannot exist). No wrong error today, but the ordering is brittle. | Discovered during characterization; AS-IS captured, not reordered. | Future `@givn.modified` delta to swap the check order. Tracked as SMELL-1. |
| D-7 | `givn new -foo` (leading-hyphen id, natural invocation) is rejected by clap as an unexpected flag (exit 2) before the id validator runs; the dedicated "must not start or end with a hyphen" message is only reachable via `givn new -- -foo`. Split behaviour. | Discovered during characterization; both paths captured as AS-IS. | Future `@givn.modified` delta to unify the two paths. Tracked as SMELL-2. |
| D-8 | `givn init` does not create `givn/artifacts/`, so project template overrides require manual directory setup. | Discovered during characterization; documented as AS-IS. | Future enhancement (e.g. `givn eject` guidance or init scaffold). Tracked as SMELL-3. |
| D-9 | For `@givn.added`/`@givn.modified` scenarios the merge rebuilds the scenario text from the parsed AST (`render_scenario_stripped`), so a step docstring is re-emitted with `"""` delimiters and the original delimiter style (`` ``` ``) plus any content-type annotation are not preserved on replaced scenarios. | The `gherkin` crate stores only dedented docstring content — it does not retain the delimiter style or annotation; preserving them would require carrying source text for replaced blocks. | Keep the fix minimal (content and data tables preserved); carrying original docstring source text across a replaced scenario is a possible future delta. Impact bounded as of change `fix-archive-merge-docstrings`. |

## 11.3 Open questions — resolved

## 11.3 Open questions — resolved

| Question | Resolution |
| --- | --- |
| Override path convention for facets | **`givn/artifacts/<id>/template.<ext>`** and **`givn/artifacts/<id>/instructions.md`** (Phase 2). |
| Linux build target: musl vs gnu | Deferred to Phase 6 (distribution not yet implemented). |
| `review.md` PASS detection: string marker vs structured field | **String marker `REVIEW: PASS`** anywhere in the file (Phase 4). Simple, git-diffable, human-readable. |
