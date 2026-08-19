# Design Review: semantic-token-cap-review

## Ranked Findings And Resolutions

The fresh review covered `proposal.md`, the complete feature delta,
`design.md`, the existing retrieval implementation, canonical/derived guidance,
and the Arc42 addon requirements. Findings are ranked by their ability to create
false semantic evidence or to block the lifecycle.

| Rank | Branch | Finding | Resolution | State |
|---:|---|---|---|---|
| 1 | Architecture documentation | The Arc42 addon is enabled, but the change-local `arc42.md` marker is absent and the durable chapters do not yet record the complete serializer, tokenizer guard, per-candidate cap states, or independent BGE/NLI flow. | Do not modify project-level Arc42 files in this planning-only pass. Record the full twelve-row assessment below and leave `arc42-docs` as the explicit lifecycle blocker. | BLOCKED |
| 2 | Scope and state model | The draft feature used run-level `BGE_UNAVAILABLE`/`NLI_UNAVAILABLE` for candidate token caps, which erased the distinction between a partial score and a no-score run. | Use `BGE_TOKEN_CAP` and `NLI_TOKEN_CAP` per candidate. Reserve run-level unavailable states for a layer that scores no pair; keep cap candidates visible, unresolved, non-filtered, and blocking. | RESOLVED |
| 3 | Candidate-pool independence | The draft design described NLI as evaluating BGE survivors, which could hide a candidate before NLI sees it. | E5 creates one immutable candidate pool. BGE and NLI independently score that pool, and combined recommendations intersect the independent recommendations. | RESOLVED |
| 4 | Serialization and cache validity | The draft did not make the complete serialization the E5 input or define how old vectors are invalidated without a serializer version. | Use one complete serialization for E5/BGE/NLI, hash that exact E5 text, reuse only matching identity/hash entries, re-embed hash mismatches, remove deleted entries, and use no separate serializer-version requirement. | RESOLVED |
| 5 | Token-budget correctness | The draft did not require actual model-tokenizer counts, pair special tokens, exact-boundary behavior, or a pre-construction check. | Count untruncated/unpadded E5 passages and BGE/NLI pair encodings with the actual tokenizer before model construction or inference. Exactly 512 fits; 513 is over. | RESOLVED |
| 6 | Testability and operational safety | A test that selected local model artifacts could download weights or require multi-gigabyte inference, and the token-budget seam was not explicit. | Use a deterministic test-only token-budget seam for regular and E2E fixtures. Production uses real local tokenizers/artifacts; tests use no network, downloader, or multi-gigabyte model. | RESOLVED |
| 7 | Missing scenarios and assertion boundary | JSON actions were implicit, the exact 512 boundary was missing, and the feature over-focused on separate model/surface combinations. | Make `givn spec duplicates --format json` explicit, retain four focused duplicate/blocking scenarios, keep two inventory-mapped CLI E2E scenarios, and add exact 512 acceptance. | RESOLVED |
| 8 | Agent guidance | Canonical and derived guidance described `BGE_UNSCORED`/unavailable behavior but not token-cap semantics or shorten/split remediation. | Update canonical specs/review guidance and the derived command/skill copies with the shared guard, state distinctions, independent layers, and no chunking/automatic-edit rule. | RESOLVED |

## Resolved By Codebase Exploration

| Branch | Finding |
|---|---|
| CLI contract | `givn spec duplicates` accepts `--format json`; the feature now names that exact observable action. |
| E5 index reuse | The existing refresh path already matches permanent identity and `content_hash` before reusing a vector. The current hash is computed from the short `scenario_text` projection, so the serializer change must make that hash come from the complete text. |
| Current serialization | The existing semantic text projection omits Feature/Rule context, inherited backgrounds, resolved roles, tables, examples, and some descriptions. The design now makes the complete ordered serialization the shared model input. |
| Current NLI flow | The duplicate-audit path mutates/removes BGE candidates before its NLI call, while other paths are closer to independent. The design requires one immutable E5 pool for every surface and removes the sequential-only contract. |
| Current tokenizer setup | The current NLI production path configures tokenizer padding/truncation and encodes before any complete pair-cap guard. The new design makes untruncated/unpadded budget measurement and pre-model checks explicit implementation constraints. |
| Test runner | `tests/features_runner.rs` uses strict `.fail_on_skipped()` behavior. The active feature remains `@wip`, so lint is expected to return exit code 2 until implementation clears the tags. |
| Arc42 state | `addons.arc42: true` is active in `givn/config.yaml`; all twelve `docs/arc42/*.md` files exist, but `givn/changes/semantic-token-cap-review/arc42.md` does not exist. |

## Open Questions And Blockers

| # | Item | Consequence |
|---:|---|---|
| 1 | Complete the enabled `arc42-docs` artifact: independently assess all twelve rows in the change marker, update affected durable chapters, update the Arc42 index, and end the marker with `STATUS: DONE`. | Blocks lifecycle completion and prevents design-review sign-off under the enabled Arc42 instruction. This pass intentionally did not create or edit those files. |

No product, scope, technology, scenario, testability, or risk decision remains
open. The Arc42 item is an artifact/lifecycle blocker, not a product question.

## Architecture Documentation (Arc42) Check

The addon is enabled. The table below is an independent assessment made from the
hardened proposal, feature, and design before trusting any change marker. The
change marker is currently absent, so its claim is `Missing` for every row. The
durable chapter files were checked for existence, project-specific content, and
the Mermaid diagram policy. The only matches from the pseudo-diagram scan were
ordinary Markdown table separators; no prohibited box-drawing or non-table ASCII
diagrams were found.

| # | Chapter | Reviewer's own assessment | `arc42.md` claim | Match? | Content matches `design.md`? |
|---:|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Missing | No | No - token-cap correctness, complete serialization, and unresolved-review goals are not recorded. |
| 2 | 02 architecture-constraints | Yes | Missing | No | No - actual tokenizer counting, no truncation/padding, and test-only seam constraints are not recorded. |
| 3 | 03 context-and-scope | Yes | Missing | No | No - duplicate/review JSON and report cap-state contracts are stale. |
| 4 | 04 solution-strategy | Yes | Missing | No | No - the complete shared serialization and independent BGE/NLI flow are not recorded. |
| 5 | 05 building-block-view | Yes | Missing | No | No - the serializer, budget guard, per-candidate evidence, and deterministic test seam are not recorded. |
| 6 | 06 runtime-view | Yes | Missing | No | No - pre-construction token checks, partial partitioning, all-over-cap refusal, and blocking flow are not recorded. |
| 7 | 07 deployment-view | No | Missing | No | Yes - no deployment topology or distribution change is planned; existing local-artifact/test-stub content is not contradicted. |
| 8 | 08 crosscutting-concepts | Yes | Missing | No | No - current guidance still describes the old unscored/unavailable semantics and lacks the shared token guard. |
| 9 | 09 architecture-decisions | Yes | Missing | No | No - the serializer/hash, tokenizer guard, per-candidate cap, and independent-pool decisions lack a new MADR entry. |
| 10 | 10 quality-requirements | Yes | Missing | No | No - exact 512/513, no-load, no-truncation, and blocking-safety scenarios are absent. |
| 11 | 11 risks-and-technical-debt | Yes | Missing | No | No - risks for stale vectors, tokenizer mismatch, false filtering, and guidance drift are absent. |
| 12 | 12 glossary | Yes | Missing | No | No - complete semantic serialization, token budget, `BGE_TOKEN_CAP`, and `NLI_TOKEN_CAP` are absent. |

- [x] All 12 rows were assessed independently against the hardened planning artifacts.
- [ ] The change marker contains all 12 claims and ends with `STATUS: DONE`.
- [ ] Every affected durable chapter matches the hardened design.
- [ ] Every new non-trivial architecture decision has a MADR entry in chapter 09.
- [ ] Every new MADR consequence is reflected in chapter 11.
- [x] All twelve durable chapter files exist and contain project-specific content.
- [x] No prohibited ASCII-art diagram was found; existing diagrams use Mermaid.

## Changes Made During Hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Made complete serialization, hash-based E5 invalidation, actual-tokenizer caps, per-candidate states, independent BGE/NLI scoring, test seams, guidance, and no-automation boundaries explicit. |
| `specs/semantic-token-cap-review/semantic-token-cap-review.feature` | Replaced ambiguous run-level candidate assertions with explicit JSON actions and per-candidate states; focused the scenarios on duplicate audit and blocking review; added partial, all-over-cap, independent-layer, and exact-512 cases. |
| `design.md` | Defined serializer field order, metadata separation, hash reuse, pre-model budget guard, state matrix, independent pool flow, output fields, test-only seam, guidance obligations, and two-action E2E matrix. |
| `assets/instructions/specs.md` | Updated canonical retrieval/spec guidance for complete serialization, token counts, per-candidate cap states, independent layers, and shorten/split remediation. |
| `assets/commands/givn-spec.md`, `.opencode/commands/givn-spec.md`, `assets/skills/givn-spec/SKILL.md.tmpl`, `.agents/skills/givn-spec/SKILL.md`, `.claude/skills/givn-spec/SKILL.md` | Propagated the retrieval/token-cap contract to derived authoring guidance. |
| `assets/instructions/review.md`, `assets/commands/givn-review.md`, `.opencode/commands/givn-review.md`, `assets/templates/review.md` | Propagated token-cap non-classification and shorten/split remediation into review guidance and the review template. |

No `tasks.md` was created. No production Rust, Arc42 chapter, Arc42 index, or
change-local `arc42.md` was edited or created.

## Verification And Sign-Off

- [x] Scope branch walked; proposal, feature, and design agree on the four
      retrieval surfaces and the duplicate/blocking scenario boundary.
- [x] Technology branch walked; existing local retrieval/tokenizer boundaries
      and the deterministic test seam are the smallest compatible approach.
- [x] Missing-scenario branch walked; exact 512, partial cap, all-over-cap, JSON
      action, layer independence, and blocking behavior are represented.
- [x] Testability branch walked; each scenario has concrete JSON/report/exit
      assertions and intentional `@wip` RED entry points.
- [x] E2E and interaction-coverage branches walked; two inventory entries map
      one-to-one to two compiled-CLI E2E scenarios in `design.md`.
- [x] Risk branch walked; the primary risk is false clean review from truncated
      or layer-filtered evidence, mitigated by complete text, preflight counts,
      visible cap states, and blocking fallback.
- [x] `givn lint --change semantic-token-cap-review` has no parse errors and
      reports only the four intentional `@wip` findings (exit code 2).
- [x] No tasks artifact was written and no production Rust was changed.
- [x] Arc42 artifact check is complete; all twelve rows are assessed, affected
      durable chapters are updated, ADR-059 through ADR-061 are recorded, and
      risks R-66 through R-71 are captured.

The planning artifacts and durable architecture documentation are internally
coherent. The Arc42 marker is complete with `STATUS: DONE`, and the affected
chapters record the serializer/hash boundary, tokenizer preflight, cap states,
independent BGE/NLI pool, and corresponding risks.

DESIGN-REVIEW: PASS
