# Change seed: enforce-ise-e2e-boundary

Status: proposed by enforce-ise-e2e-boundary

- Use-case IDs: `verify-change` (`givn-workflow-enforcement`) and
  `create-change` (`prose-consolidation`)
- Topic: `ise-rebuildability`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: 2; depends on `add-ise-addon-and-specification` for the
  declaration, the `e2e` role, and the terms
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

With `ise` effective, the declared E2E package is enforced as the only frozen
acceptance authority, and its identity is bound into verification evidence.
A missing package, a dependency edge into `source`, a runner that runs a
different target, an unbound step, or a skipped step fails the check, and no
passed receipt is issued. The canonical `@e2e` policy names the package
boundary in one owner.

The seed does not redeclare the roles or the terms. It consumes
`givn/specification.yaml`, `End-to-end (E2E) test`, and `Source code` from
`add-ise-addon-and-specification`.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `verify-change` | `givn-workflow-enforcement` | `givn check tasks` refuses a missing declared `e2e` package, a dependency edge into `source`, and a runner that is not the declared package; the E2E run fails on an unbound or skipped step; the change verification receipt binds the `e2e` package digests, the E2E runner command digest, and the running Givn version; archive freshness compares them. |
| `create-change` | `prose-consolidation` | The canonical `@e2e` policy names the declared package boundary, the no-production-source rule, the runner target, and the unbound or skipped step failure; feature files stay shared; the two `assets/instructions/specs.md` sentences and their pinned scenarios change with it. |

## Accepted decisions

### Q5: One declared `e2e` package

- Context: The `@e2e` tag, the declared role, and the frozen set had to name
  the same thing, or a receipt cannot say what was protected. The existing
  policy allowed in-process library calls and one runner for both scopes.
- Options:
  1. **Tag coincidence and one package**: In practice: a separate `e2e`
     package holds the runner and every `@e2e` step definition, links no
     production source, and `verify.e2e_command` runs it. Gain: one rule
     replaces two, and it matches Ise Pattern 03. Cost: both pilots
     restructure; Givn's runner links `givn`, so its `e2e` target needs its
     own manifest or a consumer artifact.
  2. **Keep the in-process allowance**: In practice: the tag stays broad and
     the frozen set is the scenarios the declared target ran. Gain: no
     restructure. Cost: two meanings of `@e2e`, and the four-count result
     file cannot prove which scenarios ran.
  3. **Multiple E2E targets**: In practice: split the suite by area, each
     target dependency-free. Gain: no single-target bottleneck. Cost:
     multiple frozen digests, and the `e2e` role stops naming one package.
- Chosen: **Tag coincidence and one package**, narrowed to a package boundary.
  Feature files stay shared. Rejected: the in-process allowance because the
  receipt cannot prove the protected set; multiple targets because the role
  must name one frozen authority.

### Q11: `End-to-end (E2E) test`

- Context: The boundary names a test layer, and the no-shared-code rule needs
  a qualifier that survives every precise sentence.
- Options:
  1. **`Black-box E2E test`**: In practice: `Black-box E2E tests (@e2e)`.
     Gain: the name and the qualifier come from one sentence. Cost: `... test`
     names only the tests, not the suite plus runner; it makes the current
     in-process allowance false.
  2. **`End-to-end (E2E) test`**: In practice: `@e2e`, `verify.e2e_command`,
     and `Verification scope` keep their names. Gain: the smallest edit; no
     durable sentence becomes false. Cost: `E2E` alone does not carry the
     survival rule, so every precise sentence repeats `black-box`.
  3. **`Black-box verifier`**: In practice: one noun for the suite and its
     runner. Gain: covers what the receipt digests. Cost: no Ise counterpart;
     `verifier` also names the party; it says nothing about component tests.
- Chosen: **`End-to-end (E2E) test`**, with `black-box` as the qualifier.
  Rejected: `Black-box E2E test` because it falsifies the current allowance;
  `Black-box verifier` because it collides with the verifier party.

### Deferred canonical `@e2e` policy amendments

- Context: `assets/instructions/specs.md:135-140` still permits a library
  consumer's real public API calls to run in process, and `:164-169` permits
  regular and E2E verification to share one runner command without naming the
  runner target. The boundary this seed enforces makes the declared `e2e`
  package the only acceptance authority, so both sentences become false when
  the blocking check lands. The two pinned `prose-consolidation` scenarios and
  the two `create-change` rules carry the same claims. `handoff.md` section 3
  records the in-process amendment; the shared-runner paragraph and both
  pinned rules are named in the use-case delta.
- Options:
  1. **Amend at handoff**: In practice: the policy changes before the
     boundary exists. Gain: the prose is ready. Cost: a project reading the
     policy loses the in-process allowance before any check enforces the
     package.
  2. **Defer to this seed with the exact edits recorded**: In practice: the
     two sentences, two scenarios, and two rules change in the same batch as
     the blocking boundary. Gain: prose and enforcement land together. Cost:
     the implementer must apply six exact edits, listed below.
  3. **Keep the current policy**: In practice: the in-process allowance and
     the shared-runner paragraph stay. Gain: no prose edit. Cost: the
     enforced boundary contradicts the served policy, and the pinned
     scenarios keep asserting the allowance.
- Chosen: **Defer to this seed with the exact edits recorded**. Rejected:
  amending at handoff because it removes an allowance before the replacing
  rule exists; keeping the current policy because it contradicts the
  enforced boundary.

**Landing rule.** The two `assets/instructions/specs.md` sentences, the two
pinned scenarios, and the two `create-change` rules change in one behavior
batch with the blocking boundary and the receipt binding.

**Two `assets/instructions/specs.md` amendments.**

| Location | Current text | Replacement |
|---|---|---|
| `assets/instructions/specs.md:135-140` | "A library consumer's real public API calls may run in process; do not require artificial services, containers, or browser infrastructure the product does not have." | "A library consumer's real public API calls may run in process only from the declared `e2e` package, which links no production source; a call that needs production source becomes a `@component` scenario. Do not require artificial services, containers, or browser infrastructure the product does not have." |
| `assets/instructions/specs.md:164-169` | "When regular and E2E verification share one runner command, an explicit scope selection such as `GIVN_RESULT_SCOPE` distinguishes the executions." | Keep the sentence and add: "The E2E execution runs the declared `e2e` package, which links no production source; a shared runner that links production source is not a valid E2E target." Keep the equal-count and zero-failure sentences unchanged. |

**Two pinned scenarios.**

| Location | Current assertion | Required change |
|---|---|---|
| `givn/specs/create-change/prose-consolidation.feature:124-128` `Scenario: Canonical policy accepts the library public API as its real interface` | "consumer calls to the public library API may run in process as real-interface evidence" | Change to "may run in process inside the declared e2e package as real-interface evidence" and add "a call that needs production source becomes a @component scenario". |
| `givn/specs/create-change/prose-consolidation.feature:129-135` `Scenario: Canonical policy selects E2E scope explicitly rather than by command shape` | The scenario accepts a shared runner command and explicit `GIVN_RESULT_SCOPE`. | Keep the explicit-scope and equal-count steps; add "the E2E execution runs the declared e2e package, which links no production source". |

**Two `create-change` rules.**

| Location | Current rule | Replacement |
|---|---|---|
| `givn/specs/create-change/usecase.md:46` | "Regular and E2E executions select scope explicitly, including a shared command using GIVN_RESULT_SCOPE. Equal counts or command strings do not invalidate honest isolation; required executions still require zero failures and skips without an extra isolation run." | Add after the scope sentence: "The E2E execution runs the declared `e2e` package and links no production source." Keep the remaining sentences. |
| `givn/specs/create-change/usecase.md:48` | "a library's public API may be called in-process" | "a library's public API may be called in-process only from the declared `e2e` package; a call that needs production source becomes a `@component` scenario" |

## Open questions

### Which proof scope carries the E2E identity?

- Blocks: `S3`'s receipt writer and archive freshness.
- Context: The existing receipt is `change-proof/v1`: it hashes tested
  content, the delta features, and `verify.command`, and reads four runner
  counts. The seed adds the `e2e` package digests, the E2E runner command
  digest, and the Givn version. The topic settled what must be bound, not
  whether the existing scope version can carry it.
- Question: Does the E2E identity extend `change-proof/v1` with additive
  fields, or does it require a new proof scope version?
- Options:
  1. **Additive fields in `change-proof/v1`**: In practice: the receipt gains
     the three fields; an `ise`-effective change refuses a receipt without
     them. Gain: no version bump and no consumer migration. Cost: one scope
     version has two shapes, so every consumer must treat missing fields as
     stale rather than optional.
  2. **A new `change-proof/v2` scope**: In practice: an `ise`-effective
     change requires v2; a non-`ise` change keeps v1. Gain: the shape is
     explicit and an old receipt is refused by name. Cost: two live scopes,
     and archive, review, and any external consumer must learn the new
     version.
- Example: a receipt written before this seed has no `e2e` field. On an
  `ise`-effective change, archive either refuses it as incomplete under v1 or
  refuses it as the wrong scope version under v2.

### Which package manifests must the E2E dependency-edge check understand?

- Blocks: this seed's blocking dependency-edge refusal and the fourth
  advisory finding shipped by `add-ise-addon-and-specification`.
- Context: Q4 and Q5 settled that both checks read the package manifest graph,
  never source imports, and that the declaration is project-level. The topic's
  pilots are Cargo projects, but the declaration does not name a package
  manager. The advisory finding and the blocking refusal share this reader, so
  one answer governs both.
- Question: In the first cut, which package manifests does the `e2e` package
  dependency-edge check read?
- Options:
  1. **Cargo only**: In practice: `cargo metadata` decides whether the `e2e`
     package reaches `source`. Gain: one tested graph for the Givn, Passuff,
     and Catopia pilots. Cost: a Python, Java, TypeScript, or JavaScript
     project receives no dependency-edge finding and the advisory is silent.
  2. **Cargo plus the outline's other stacks**: In practice: Cargo,
     `pyproject.toml` or `requirements.txt`, Maven or Gradle, and
     `package.json`. Gain: the finding covers the stacks Givn already
     analyzes. Cost: four graph readers and their failure modes; the check
     can refuse where it cannot read.
  3. **A project-declared dependency command**: In practice: the declaration
     names a command that prints the `e2e` package's dependencies. Gain: one
     contract for any stack. Cost: a command surface and a trust boundary;
     a wrong command reports a false boundary.
- Example: a Java project enables `ise`; with Cargo only, `givn lint` says
  nothing about its `e2e` package's dependency on production classes, so the
  blocking boundary arrives only at `givn check tasks` on the first change.
