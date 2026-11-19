# Proposal: ise-compare-runs-trial

## Release note

`givn ise compare` now runs the trial when the frozen inputs match, refuses a missing component runner, and keeps a read-only `--inputs-only` inspection path.

## Use-Case Context

- Use-case ID: prepare-an-ise-test
- Ideation topic: ise-rebuildability
- Seed path: none
- Confirmed Personas: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Review Context

- Review kind: self-review
- Reason: focused correction to an existing command contract; the operator supplied the direction and the research conclusion.

## Problem / Opportunity

A whole-system Passuff rebuild produced a spec copy with 215 component
scenarios, ran none of them, and still reported a green outcome. `givn ise
compare` compared the frozen inputs, printed `Trial result: unverified`, and
exited 0; `givn ise verify-trial` was optional, so a harness that treated the
comparison's exit 0 as success never reached the component corpus. The rebuilt
product also removed the component runner, and nothing in the guidance said
that the rebuild must write the runner, steps, and fakes the frozen scenarios
need. The ordinary workflow already says a project with specs executes them;
the trial command did not.

## Proposed Solution

`givn ise compare` is the normal trial command. It reports the two frozen-input
subjects; when they match, it executes the generated verification contract,
writes the `ise-trial-result/v1` record beside the manifest unless `--result`
names a file, and exits 0 only when the trial is clean. A missing component
runner, a zero-unit runner, a count mismatch, a foreign product, or a failed or
skipped scenario refuses with `component_executed=<executed>/<declared>` and
`clean: false`; no pass line prints.

The result path is validated before the comparison: it must not be the
manifest, lie inside the checkout or the copy, or be a symlink. A previous
file is removed only when it is a regular Givn trial result, and a raw sibling
only when it is a regular Givn runner result; an unrelated or malformed file
refuses and survives. This revocation also runs when the inputs differ.

`--inputs-only` preserves the read-only inspection: it executes no contract
command, writes no result, prints
`Trial result: unverified (inputs only; no trial command was executed)`, and
exits 0 only when the frozen inputs match. `givn ise verify-trial` remains the
explicit executor with a named result path; both commands share one gate.

The rebuilt product's End-to-end run uses the existing product binding. The
served ise guidance now states that a rebuild is ordinary development: the
builder writes the component runner, step definitions, fakes, and fixture
builders the frozen scenarios need, executes the component feature files in the
copy, preserves the component scenario corpus, records per-tier coverage when
declared, and never substitutes a coverage number or a green End-to-end suite
for the declared scenario execution.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| trial-input-verification | `EXTEND givn-workflow-enforcement` (score 17) | `EXTEND trial-input-verification` | The routed candidate is a generic workflow-enforcement use case. The changed action is the existing `trial-input-verification` capability under `prepare-an-ise-test`. |
| serve-extended-rules | no direct signal | `EXTEND serve-extended-rules` | The served ise guidance already belongs to this capability; the change modifies its assertions. |

## Out of Scope

- No numeric coverage non-decline gate. Research showed pre/post source-line
  percentages compare different files and denominators (4293 vs 4965), the
  End-to-end subprocess is uninstrumented, and a zero branch denominator can
  pass vacuously. The existing exact scenario multiset and non-zero component
  line measurement stay.
- No Passuff checkout edit and no claim that a real Passuff trial passes. The
  current Passuff consumer lacks result writing and `product_path`; the fixture
  proves the Givn gate, not the consumer.
- No removal of `givn ise verify-trial`; it remains the explicit executor.
- No change to the manifest or result schema.

## Open Questions

### D1 — Does the normal comparison execute the trial?

- Question: Should `givn ise compare` stay read-only and require a second
  command for the trial, or execute the contract after the frozen inputs match?
- Options:
  1. **Execute by default** — compare is the normal trial command; gain: the
     green exit means the corpus ran; cost: compare now runs copy commands; in
     practice: `givn ise compare --manifest ... --copy ...` prints the tier
     lines and exits 0 only for `clean: true`.
  2. **Stay read-only** — keep the 2026-11-18 shape; gain: safe on an untrusted
     copy; cost: the observed Passuff green with zero component scenarios
     remains possible; in practice: a harness ignores the unverified line and
     accepts exit 0.
- Disposition: answered — operator direction, 2026-11-19.

### D2 — What preserves the read-only inspection?

- Question: Which switch keeps a safe input-only check?
- Options:
  1. **`--inputs-only`** — explicit opt-out; gain: one command, no second
     verification path; cost: the caller must read the unverified line; in
     practice: `givn ise compare ... --inputs-only` writes no result.
  2. **Keep `verify-trial` only** — gain: no new flag; cost: normal compare is
     still green on inputs alone.
- Disposition: answered — `--inputs-only` is the read-only form.

### D3 — Keep or remove `givn ise verify-trial`?

- Question: Does folding execution into compare retire the explicit command?
- Options:
  1. **Keep it** — explicit result path and `--component-not-run`; gain:
     compatible with existing callers and tests; cost: two names for one gate;
     in practice: `verify-trial` calls the same `ise::verify_trial`.
  2. **Remove it** — gain: one command; cost: breaks callers and the not-run
     decision surface.
- Disposition: answered — keep it as the explicit executor; compare is the
  normal path.

### D4 — Where does the default result go?

- Question: Without `--result`, where is the record written?
- Options:
  1. **Beside the manifest** — `<manifest>.trial-result.json`; gain: outside
     the copy by the manifest rule; cost: a sibling file; in practice:
     `/tmp/ise-passuff.yaml.trial-result.json`.
  2. **Inside the checkout** — gain: discoverable; cost: dirties the checkout
     and can enter the post-run comparison.
- Disposition: answered — beside the manifest; `--result` overrides; the path
  must not be the manifest, lie inside the checkout or the copy, or be a
  symlink; only a regular Givn-owned result is removed, before the comparison.

### D5 — How is the security change exposed?

- Question: Compare now executes the copy's declared commands. How does a user
  learn that?
- Options:
  1. **Document and default-off execution flag** — gain: safe default; cost:
     ordinary compare is still unverified, which is the bug.
  2. **Default execution, documented, with `--inputs-only`** — gain: the normal
     path executes; cost: an untrusted copy must use the flag; in practice: the
     help, reference, and served guidance name the execution and the safe path.
- Disposition: answered — default execution, documented; `--inputs-only` is the
  safe path. R-172 records the risk.

### D6 — Is a numeric coverage non-decline gate part of the fix?

- Question: Should the rebuilt product refuse when coverage falls below the
  previous build?
- Options:
  1. **No numeric gate** — gain: valid comparison; cost: coverage is evidence,
     not a gate; in practice: exact scenario execution and non-zero component
     lines remain.
  2. **Baseline percentage** — gain: catches broad regressions; cost: different
     files and denominators (4293 vs 4965), uninstrumented e2e, vacuous zero
     branch denominators; in practice: a false green or a false red.
- Disposition: answered — no numeric non-decline; research conclusion accepted.

### D7 — Does the guidance require the component runner, steps, and fakes?

- Question: Does the ise guidance state that the builder writes the component
  runner, steps, and fakes and executes the frozen component feature files?
- Options:
  1. **Explicit obligation** — gain: the Passuff failure is addressed at the
     workflow level; cost: more served prose; in practice: the guidance names
     the ordinary development step and the `component_executed` refusal.
  2. **Rely on the gate alone** — gain: shorter guidance; cost: a builder can
     read the gate and still not know what to write.
- Disposition: answered — add the **Component workflow** obligation.
