# Design: Explore Scenarios in the Current Change

## Boundary

This is a read-only CLI capability. The real interface is the `givn` terminal
process. The command accepts the complete scenario input already supported by
PR 1 and an optional active change identifier:

```text
givn spec explore --change <id> --file <draft.feature>
givn spec explore --change <id> --stdin
```

Without `--change`, the PR 1 permanent-only behavior remains unchanged. With a
change, the command compares the query with the permanent index and with the
active scenarios under `givn/changes/<id>/specs/`. It never edits feature files
or creates a permanent index entry for a delta scenario.

"Unchanged" means that no-change requests preserve PR 1 input validation,
permanent candidate selection, ranking, and all existing output fields and their
ordering. The output change is intentionally additive rather than byte-for-byte
PR 1 compatibility: JSON results add `scope` and `change` for every explore
request, including no-change requests, and text results add source metadata.
With `--change`, the same permanent results are retained and active candidates
are merged into the request only.

## Architecture Impact

| File | Change |
| --- | --- |
| `src/main.rs` | Add the optional `--change` argument to `spec explore` and pass it through dispatch. |
| `src/cli/spec.rs` | Preserve the exactly-one-input validation and pass the change identifier to retrieval. |
| `src/spec/retrieval.rs` | Validate and collect active delta scenarios before refresh, normalize every query/source/target/index identity through one project-relative path helper, compute request-local vectors, merge permanent and delta candidates for ranking, apply lifecycle identity filters, and render additive source metadata in text and JSON. |
| `tests/steps/mod.rs` | Register the new regular and e2e step modules and store the permanent-index snapshot used by the non-mutation scenario. |
| `tests/steps/spec_explore_change_steps.rs` | Regular black-box fixtures and assertions for active, removed, modified, and index-persistence behavior. |
| `tests/steps/spec_explore_change_e2e_steps.rs` | Separate real-process drivers and terminal/JSON assertions for the two new CLI interactions. |
| `givn/changes/spec-explore-change/specs/spec-explore-change/spec-explore-change.feature` | Executable scenarios for this slice. |

No new runtime or test dependency is required. The existing Gherkin parser,
E5-compatible embedding backend, JSON serializer, and Cucumber runner are
retained.

The runtime flow is:

```mermaid
sequenceDiagram
    participant User
    participant CLI as givn spec explore
    participant Parser as Gherkin parser
    participant Index as permanent index
    participant Embedder as embedding backend
    participant Delta as active change specs

    User->>CLI: complete scenario, optional change id
    CLI->>Parser: parse exactly one scenario
    Parser-->>CLI: query identity and normalized text
    opt active change supplied
        CLI->>Delta: validate id and specs directory; collect delta files
        Delta-->>CLI: active added and modified scenarios, normalized identities
    end
    CLI->>Index: refresh permanent entries by content hash
    Index->>Embedder: embed changed permanent entries only
    Embedder-->>Index: permanent vectors
    opt active change supplied and validation succeeded
        CLI->>Embedder: embed active delta text for this request
        Embedder-->>CLI: transient delta vectors
    end
    CLI->>Embedder: embed query with query role
    Embedder-->>CLI: query vector
    CLI->>CLI: rank both scopes and filter lifecycle identities
    CLI-->>User: top-five source-labeled text or JSON
```

Change-id validation, change existence/specs-directory validation, and delta
parsing happen before the permanent refresh. A rejected change-aware request
therefore returns a clear non-zero error without writing the permanent index.

## Change Resolution and Delta Collection

The optional identifier is validated with the existing kebab-case change
resolver. The command then requires an existing change and
`givn/changes/<id>/specs/`, using the same delta path validation as
`spec review --change`. Missing, unsafe, or incomplete changes fail before the
permanent refresh and before a result is rendered. A malformed delta feature
fails at collection with an error that identifies the invalid Gherkin input.

Existing delta collection is reused for parsing direct and `Rule:`-nested
scenarios. Each collected item retains:

- the active change source file relative to the project root;
- the scenario title, normalized text, and content hash;
- the projected permanent target file derived from the capability tag; and
- its `Added`, `Modified`, or `Removed` operation.

Permanent index files, delta source files, projected permanent target files, and
file-query identities all pass through one project-relative, forward-slash path
normalization helper. The helper accepts both relative and absolute inputs, so
identity comparisons do not depend on how a path was supplied.

Only `Added` and `Modified` items enter the exploration corpus. `Removed`
items are ignored. A modified item has two identities for filtering: its
source `(delta file, title)` and its projected permanent `(target file, title)`
identity. Both represent the same scenario for this request and are excluded
when they equal the query identity. This prevents a modified scenario from
matching itself whether the query came from the delta file or the permanent
target file. Exact source identity is also excluded for added scenarios.

## Candidate Model and Ranking

The permanent `SpecIndex` remains the only persisted catalogue. Its
`IndexEntry` values continue to contain title, project-relative file,
content hash, and embedding. The active delta vectors are held only in memory
for the current invocation.

The exploration path uses an internal candidate record with:

- title and project-relative source file;
- embedding score;
- `scope`, either `permanent` or `current-change`;
- optional change identifier for current-change candidates; and
- optional projected target identity and delta operation for filtering.

Permanent records use the existing index embeddings. Active records use their
normalized scenario text and the existing passage embedding role. The query
continues to use the existing query embedding role. All records are combined,
then sorted by descending cosine score with deterministic ties by scope,
source file, and title. Exact query identities are removed before the top-five
limit is applied. The no-change path continues to use the existing permanent
candidate set, ranking, and identity behavior. Its result records are still
labeled as `permanent`; change-aware records are labeled `current-change`.

The permanent refresh runs before transient delta embedding. It writes only
the permanent entries returned by the existing content-hash refresh. Delta
vectors are never passed to `write_index`, so the index metadata and persisted
entry set remain permanent-only.

## Output Contract

Human output retains the PR 1 query, model, index, reuse, and embedding-score
information. Each ranked line adds source metadata for every explore request:

```text
rank 1: 'Provider name is unknown' (scope permanent, source givn/specs/provider.feature, embedding score 0.9000)
rank 2: 'Current change rejects an unknown provider' (scope current-change, change current-change, source givn/changes/current-change/specs/current-change/current-change.feature, embedding score 0.8500)
```

JSON retains every PR 1 field and its ordering, then appends `scope` and `change`
to each result. This additive contract applies to no-change JSON as well as
change-aware JSON; it is not a promise of byte-for-byte PR 1 output:

```json
{
  "query": { "title": "...", "source": "file", "file": "draft.feature", "content_hash": "..." },
  "index": { "model": "...", "version": 1, "entries": 2 },
  "results": [
    {
      "rank": 1,
      "title": "...",
      "file": "givn/specs/provider.feature",
      "score": 0.9000,
      "scope": "permanent",
      "change": null
    },
    {
      "rank": 2,
      "title": "...",
      "file": "givn/changes/current-change/specs/current-change/current-change.feature",
      "score": 0.8500,
      "scope": "current-change",
      "change": "current-change"
    }
  ]
}
```

`index.entries` reports permanent entries only. The result `file` for a
current-change candidate identifies the active delta source, while `change`
identifies the owning change. No refresh diagnostics are printed before JSON.

## CLI Surface

```text
givn spec explore --file <draft.feature> [--format text|json]
givn spec explore --stdin [--format text|json]
givn spec explore --change <id> --file <draft.feature> [--format text|json]
givn spec explore --change <id> --stdin [--format text|json]
```

`--change` is optional and does not alter input-source validation. `--file` and
`--stdin` remain mutually exclusive, and exactly one is still required. All
invalid input and no-feature errors retain the PR 1 contracts.

## Step Definitions and Test Runner

The feature runner is the existing `features_runner` test binary using the
existing Cucumber/Gherkin dependency. It uses `.fail_on_skipped()` so undefined
or skipped steps fail the process. Every new Rust step body starts with
`unimplemented!()` during RED; no empty step body is permitted.

Regular bindings are in `tests/steps/spec_explore_change_steps.rs`. E2E
bindings are in the separate `tests/steps/spec_explore_change_e2e_steps.rs`.
Both invoke the compiled `givn` process in an isolated temporary project;
the e2e bindings are separate because they are the real CLI smoke-test seam.
Existing `run_givn_with_envs` and `run_givn_with_stdin_and_envs` helpers are
reused. The deterministic `GIVN_SPEC_EMBEDDER=stub` backend avoids model
downloads and network access.

The repository runner remains:

```sh
./run-tests.sh
```

The change-specific executable-spec command is:

```sh
GIVN_FEATURES=givn/changes/spec-explore-change/specs \
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
cargo test --features spec-retrieval --test features_runner
```

The single-scenario RED/GREEN command is:

```sh
GIVN_FEATURES=givn/changes/spec-explore-change/specs \
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
cargo test --features spec-retrieval --test features_runner -- \
  --name "Change-aware file exploration finds permanent and active change candidates"
```

The change-specific e2e command is the same runner with `-- --tags @e2e`.
The configured repository e2e command is `./run-tests.sh --e2e`; it runs the
real CLI process and asserts on captured stdout, not on retrieval functions.

## Local Runnability and External Dependencies

The complete local command is `./run-tests.sh`. This capability has a CLI
interface only; it has no server, database, queue, browser, or external
service. Therefore no container network or digital twin is required. The
temporary-project fixture and deterministic embedding stub are the local test
infrastructure. The only process-lifecycle concern is stdin: the helper writes
the complete feature document to a pipe and closes stdin before waiting for the
child process. File-based e2e scenarios use a temporary file under the isolated
project root.

Every query fixture used by these scenarios contains exactly one scenario. The
identity cases use one-scenario delta/source files or distinct fixture files so
the query cardinality rule is never relaxed. The added and modified identity
cases also include a separate eligible candidate, proving that filtering
removes only the queried identity and does not suppress the rest of the ranked
set.

Regular executable coverage includes added self-identity, modified queries from
the delta source and projected permanent target identities, relative/absolute
path parity, change-aware `--stdin` parity, removed-scenario exclusion, and
permanent-index non-persistence. It also includes unsafe change ids, missing
changes, changes without a specs directory, and malformed delta features; each
asserts a clear non-zero error, with index non-mutation asserted where the
request reached an observable index boundary.

## Interaction Coverage Matrix

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism |
| --- | --- | --- | --- |
| invoke `givn spec explore --change <id> --file <draft.feature>` and read source-labeled text results | Change-aware file exploration finds permanent and active change candidates | CLI | `std::process::Command` starts the compiled `givn` binary with `spec explore --change ... --file ...` and the step asserts ranked terminal stdout. |
| invoke `givn spec explore --change <id> --file <draft.feature> --format json` and consume source metadata | Change-aware JSON exploration labels candidate sources | CLI | `std::process::Command` starts the compiled `givn` binary with the change and JSON flags; the step parses the returned stdout document and asserts result metadata. |

These remain two `@e2e` scenarios because they are distinct consumer
interactions for this capability: one consumer reads source-labeled terminal
text, while another consumes a machine-readable JSON document. Both drive the
same real CLI boundary, but their output contracts and consumption assertions
are different. Standard-input support and all validation/error variants remain
regular scenarios and are intentionally absent from the inventory.

The primary assertions for both e2e scenarios are on the real CLI output and
exit status. Filesystem assertions are limited to regular non-mutation
scenarios and are not a substitute for either CLI smoke test.

## Compatibility and Boundaries

`spec index`, `spec search`, and `spec review --change` retain their existing
dispatch and output paths. No-change `spec explore` retains its PR 1
validation, ranking, candidate set, and old field ordering while adding the
source metadata described above. This slice does not generate a review report,
classify candidates, project archive state, invoke BGE or NLI evidence, gate a
review, or edit feature files. The existing permanent-only PR 1 scenarios stay
in the repository suite as regression coverage.
