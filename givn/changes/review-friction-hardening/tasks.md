# Tasks: review-friction-hardening

Release note: Make `givn check review` report every use-case schema and capability-routing problem in one pass, and let `givn archive` reuse a green change proof when only documentation changed after it was recorded.

Use-case constraints carried into every scenario:

- `verify-change`: structural failures outrank other findings; a review PASS requires a fresh green change-scoped receipt; a review report names every use-case schema and capability-routing problem in one pass, and the schema stays strict.
- `archive-change`: archive validates the current green receipt before mutation and does not rerun the change suite; a receipt that changes only documentation or evidence stays fresh and the merged suite still runs once; tested-content changes are stale and refuse before mutation; only a proven archive is complete.
- Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. No Persona is invented or promoted.
- Terminal gate: `Interface: terminal` requires `VISUAL-REVIEW: PASS` and committed transcripts; no `NOT APPLICABLE` path.
- One atomic `givn commit` per scenario with the release note as subject and the scenario title in the body.

## Setup — runner, strict mode, and e2e configuration

- [ ] Setup task

Design constraints:

- Runner: `tests/features_runner.rs`, cucumber-rs, `.fail_on_skipped()` on both the normal and single-scenario paths (`tests/features_runner.rs:55,151`). Undefined and pending steps fail.
- `verify.command`: `./run-tests.sh --non-e2e`. `verify.e2e_command`: `./run-tests.sh --e2e`. They are not equal.
- Feature filter: `GIVN_FEATURES=givn/changes/review-friction-hardening/specs`.
- Step files: `tests/steps/workflow_enforcement_steps.rs`, `tests/steps/workflow_enforcement_e2e_steps.rs`, `tests/steps/archive_proof_steps.rs`, `tests/steps/archive_proof_e2e_steps.rs`, shared fixtures in `tests/steps/archive_proof_support.rs` and `tests/steps/mod.rs`.
- Step bodies are never empty; RED uses undefined steps or `unimplemented!()`.

Strict-mode proof:

```sh
scratch=$(mktemp -d)
printf '%s\n' 'Feature: Strictness probe' '  Scenario: Undefined step fails' '    Given no step definition exists for this sentence' > "$scratch/probe.feature"
GIVN_FEATURES="$scratch" cargo test --locked --test features_runner -- --name "Undefined step fails"
```

- [x] Probe exits non-zero; paste command and output:
  ```
  GIVN_FEATURES="$scratch" cargo test --locked --test features_runner -- --name "Undefined step fails"
  ...
  Scenario: Undefined step fails
   ✘  Given no step definition exists for this sentence
      Step doesn't match any function
  [Summary] 1 feature, 1 scenario (1 failed), 1 step (1 failed)
  exit=1
  ```
- [x] Remove the probe and re-run the delta suite so the only failures are the `@wip` scenarios.
- [x] Paste `verify.command` scenario count and `verify.e2e_command` scenario count; e2e count is strictly less than the full count:
  ```
  GIVN_FEATURES=... --list | grep -c ': test$'            -> 12
  GIVN_FEATURES=... GIVN_CUCUMBER_TAGS=@e2e --list | ...  -> 2
  ```

## Scenario 1 — A case-mismatched document title is refused with the exact correction

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Parser: `src/gherkin/usecase_md.rs`. Title must be exactly `# Use case: <kebab-case-id>` or `# Fragment: <id>`. A case-insensitive prefix match recovers the id and sections, emits one diagnostic at line 1, and continues.
- Diagnostic text: expected title, `headings are case-sensitive`, and `did you mean '# Use case: review-diagnostics-demo'?`.
- No schema variant is accepted. The diagnostic is one line; later section diagnostics still follow in the same run.
- Routing gate: `src/spec/routing_table.rs` groups the parser diagnostics under the document path; existing `error: capability routing decisions do not match the built delta:` prefix stays.
- Steps: `tests/steps/workflow_enforcement_steps.rs`; the fixture commits the change and runs the real `givn check review`.
- Files likely touched: `src/gherkin/usecase_md.rs`, `src/spec/routing_table.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 2 — A near-miss section heading is refused with the exact accepted spellings

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Parser: `src/gherkin/usecase_md.rs`. `## Guarantees` is an unknown heading; skip its body until the next known heading, report `line <n>: unknown heading '## Guarantees'`, and suggest both `## Minimal guarantee` and `## Success guarantee`.
- Suggestions are exact; no case-insensitive heading is accepted.
- The report also lists the accepted schema headings when there is no close match.
- Steps: `tests/steps/workflow_enforcement_steps.rs`; real `givn check review`.
- Files likely touched: `src/gherkin/usecase_md.rs`, `src/spec/routing_table.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 3 — Missing required sections and a malformed interaction table are reported together

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Parser: `src/gherkin/usecase_md.rs`. Missing required headings are collected after the scan (`## Level`, `## Goal`, `## Trigger`, `## Main flow`, `## Minimal guarantee`, `## Success guarantee`, `## Out of scope`, plus the document-specific required set).
- An unusable interaction table header/separator reports one table diagnostic and skips only the row semantics that cannot be read; valid rows still feed capability checks.
- Section checks run independently; one malformed section does not suppress another section's diagnostic.
- Steps: `tests/steps/workflow_enforcement_steps.rs`; real `givn check review`.
- Files likely touched: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 4 — One use-case parse failure is reported once across several delta capabilities

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Routing gate: `src/spec/routing_table.rs` builds one `BTreeMap<PathBuf, ParseResult>` cache keyed by document path, parses each document once, appends each parser diagnostic once, then deduplicates exact error strings.
- Three delta capabilities routed `NEW in` the same use case must not repeat the document failure three times.
- `ParseResult` is `{ doc: Option<UsecaseDoc>, diagnostics: Vec<UsecaseDiagnostic> }`; a non-recoverable parse skips only routing checks that need the missing structure.
- Steps: `tests/steps/workflow_enforcement_steps.rs`.
- Files likely touched: `src/spec/routing_table.rs`, `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 5 — Review validates an active delta use-case narrative even for an extension

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Routing gate: collect every active delta narrative under `change_dir/specs/**` into the document cache before either early return (no delta capabilities, or no filled routing rows). An `EXTEND` narrative is parsed even though the routing decision extends a permanent capability.
- The report names the replacement narrative's path and its line-1 title diagnostic and unknown-heading diagnostic.
- Routing-table advisory behavior is unchanged when the table is missing; this change does not turn that advisory into a blocking gate.
- Steps: `tests/steps/workflow_enforcement_steps.rs`.
- Files likely touched: `src/spec/routing_table.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 6 — Review accepts a green proof after documentation-only review edits

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Proof: `src/proof.rs`. `ChangeReceipt` schema 2 adds `tested_content_digest` and `content_scope: "change-proof/v1"`. `write_change_receipt` accepts the captured digest and scope.
- Freshness: `Current` when head and all digests match; `DocumentationOnly` when the tested content, features, and command match and the tested revision is available; `Stale` otherwise with every reason.
- Digest: sorted `/` paths, `path NUL executable-marker NUL file-bytes NUL`; tracked plus untracked non-ignored; documentation/evidence allowlist excluded; gitlink object id and symlink target bytes.
- `is_documentation_or_evidence_path(project_root, change, manifest, path)` reuses `is_evidence_path` for evidence; README is unconditional documentation.
- Review: `src/cli/check.rs` uses the shared freshness result and prints `✓ change verification receipt reused across documentation-only revision <current> (tested at <tested>, N scenario(s))`; no test runs.
- SCM seam: `Backend::content_paths()` in `src/scm.rs`, `src/git.rs`, `src/fossil.rs`.
- Steps: `tests/steps/archive_proof_steps.rs`; set `skip_receipt_autowrite`, commit documentation-only edits, run the real `givn check review`.
- Files likely touched: `src/proof.rs`, `src/scm.rs`, `src/git.rs`, `src/fossil.rs`, `src/cli/check.rs`, `tests/steps/archive_proof_support.rs`, `tests/steps/archive_proof_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 7 — Archive reuses a proof committed with its receipt

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Archive: `src/cli/archive.rs` uses the shared freshness result before mutation; `DocumentationOnly` prints the reused message and continues; the merge, candidate publication, merged regular and e2e scopes, coverage, and finalization are unchanged and run exactly once.
- No change-suite invocation is added. The change directory still moves, the archive is proven, and the candidate binds its own tested revision.
- Freshness order and stale reasons are in `src/proof.rs` (schema 1 strict head equality; unknown scope/schema refused).
- Steps: `tests/steps/archive_proof_steps.rs`; commit only `verification/tasks-verification.json`, run the real `givn archive`, assert proven archive and no change-suite invocation.
- Files likely touched: `src/cli/archive.rs`, `src/proof.rs`, `tests/steps/archive_proof_steps.rs`, `tests/steps/archive_proof_support.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 8 — Archive refuses a proof after tested content changed

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Outline rows: delta feature, verify command, source code. Each row commits the changed content after the green receipt, then runs `givn archive`.
- Archive: `src/cli/archive.rs` refuses before mutation with `verification receipt is stale`, names the changed boundary, and leaves no candidate. The active change directory remains under `givn/changes/`.
- Reasons: `FeaturesChanged`, `CommandChanged`, `TestedContentChanged`; all independently checkable reasons are collected. A feature or command change may also change the tested-content digest; both lines are acceptable because each names a distinct boundary.
- Steps: `tests/steps/archive_proof_steps.rs`; real `givn archive`; assert the change directory and absence of an archive candidate.
- Files likely touched: `src/cli/archive.rs`, `src/proof.rs`, `tests/steps/archive_proof_steps.rs`.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 9 — Consumer runs review and receives every use-case schema problem in one pass (e2e)

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Real interface: `givn check review --change review-diagnostics-demo` subprocess in a committed Git fixture. Assert exit code and raw stderr; assert no test runner ran.
- Fixture: the delta capability is routed `NEW in review-diagnostics-demo`; the use-case contract has a case-mismatched title, `## Guarantees`, a non-bullet capability line, an undeclared interaction capability, and a non-bullet example line.
- One run reports all five, each distinct problem once, grouped under the document path. The schema stays strict and the command exits non-zero.
- Steps: `tests/steps/workflow_enforcement_e2e_steps.rs`. Capture the final transcript at `givn/changes/review-friction-hardening/evidence/visual/consumer-runs-review-and-receives-every-use-case-schema-problem-in-one-pass/transcript.txt` with scenario title, argv, cwd disposition, exit code, and raw stdout/stderr.
- Files likely touched: `tests/steps/workflow_enforcement_e2e_steps.rs`, evidence transcript.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Scenario 10 — Consumer archives a change after documentation-only review edits and the proof is reused (e2e)

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Real interface: `givn archive --change review-proof-demo` subprocess. Assert exit 0, proven archive, the reused-proof message, one merged regular invocation with features path `givn/specs`, and no change-features invocation.
- Fixture: green change verification receipt at an earlier revision; commit only proposal, design, design-review, tasks, review, arc42, use-case narrative, README, and evidence edits; set `skip_receipt_autowrite`; the verify command records every invocation with scope and features path.
- Archive runs the merged regular and e2e scopes exactly once; no extra suite run. The change suite is not invoked.
- Steps: `tests/steps/archive_proof_e2e_steps.rs`. Capture the final transcript at `givn/changes/review-friction-hardening/evidence/visual/consumer-archives-a-change-after-documentation-only-review-edits-and-the-proof-is-reused/transcript.txt` with scenario title, argv, cwd disposition, exit code, raw stdout/stderr, the reused-proof line, the merged-suite invocation, and the proven archive result.
- Files likely touched: `tests/steps/archive_proof_e2e_steps.rs`, evidence transcript.

Evidence:

- RED output: `<paste>`
- GREEN output: `<paste>`
- REFACTOR output: `<paste>`
- Production files: `<list>`
- COMMIT: `<hash>`

## Final gates

- [ ] `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `./run-tests.sh --non-e2e`
- [ ] `./run-tests.sh --e2e`
- [ ] `givn lint --change review-friction-hardening` reports no `@wip`
- [ ] Both transcripts committed under `evidence/visual/`
