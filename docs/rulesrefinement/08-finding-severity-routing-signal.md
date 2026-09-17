# 08 — finding-severity-routing-signal

Status: ready for `/givn-propose`
Source: ignored advisory output (M6 options A and B)
Depends on: none (lands last to avoid template churn)

## Problem

givn mixes hard gates and hints in the same output channel without severity
labels, and the weakest tool — route, based on content-word overlap — is
given the most prominent per-change ritual. Every proposal records a routing
verdict with a verbatim "its ranking is advisory" disclaimer, and lint
reports around twenty advisories that exit 0 and are ignored. Agents learn
that givn output is boilerplate, which degrades every current and future
check.

## Evidence

kiekste side:

- All 18 active proposals record a routing verdict of the form
  "`change-preview` is not in the route's top ten. The route ranks by
  content-word overlap only; its ranking is advisory" — while the recorded
  decision was correct. The disclaimer appears verbatim across the corpus.
- `givn/changes/async-mutation-requests/proposal.md`: six-way tie at 5.00.
- `givn lint` on the worktree reports about twenty `[SHAPE]`/`[SUBST]`
  advisories, many self-evidently fine, exit 0; all changes archived
  regardless.

givn side:

- `assets/templates/proposal.md` lines 24–33: the Capability Routing table
  is mandatory for every change.
- `src/spec/route.rs`: affinity from command surface (`count_subcommands`)
  and content-word overlap (`content_words`, `usecase_affinity`).
- `src/spec/routing_table.rs`: checks only that a decision was recorded.
- `src/cli/lint.rs` `render_prose` (line 251): level labels exist
  (`DUP`, `SHAPE`, `SUBST`, `WIP`, `ERROR`) but carry no severity semantics;
  exit codes 0/1/2/3 are structural, not severity-based.
- `givn/specs/fragments/spec-explore-change.feature` covers `givn spec
  route --change` behavior.

## Decided scope

1. Severity labels and confidence gating: label findings
   `advisory | finding | blocking`; route reports confidence and the
   proposal omits the routing table below the confidence threshold. (M6 A)
2. Better route signal: weight use-case Goal/Rules ownership and the
   Interactions inventory over raw scenario-word overlap; penalize
   cross-use-case scatter. (M6 B)

## Required changes

### 1. Severity labels

- Classify every existing lint finding kind into `advisory`, `finding`, or
  `blocking`. Preserve the existing exit-code contract and document the
  mapping; do not silently change exit codes.
- Render the severity in the human-readable output and expose it in JSON
  output where lint already emits structured findings.
- `givn lint` must not treat `advisory` as a failure.
- This is also the classification that the new checks from changes 2, 3, 6,
  and 7 adopt. Keep the mapping in one place so those changes can register
  their kinds.

### 2. Route confidence and conditional routing table

- `src/spec/route.rs`: compute and report a confidence value. Report "no
  signal" when the top candidates tie or when overlap is below a defined
  threshold.
- `assets/templates/proposal.md` and `assets/commands/givn-propose.md`:
  when route has signal, keep the Capability Routing table; when route
  reports no signal, the proposal omits the route recommendation column and
  records the decision with its rationale directly. Use the existing
  conditional-template mechanism (see
  `givn/specs/fragments/template-conditional-rendering.feature`); if it
  cannot express this, record the decision in the design instead of
  inventing a new mechanism.
- The decision itself remains mandatory when route has signal: the table
  still records route's recommendation and the chosen decision.

### 3. Better route signal

- Weight ownership evidence above lexical overlap:
  - the use-case `Goal` and `Rules` text of candidate owners;
  - the existing `Interactions` inventory of the candidate capabilities;
  - the capability name and its owning use case.
- Penalize candidates that scatter across many use cases (a capability
  proposed for a use case whose goal it does not advance should rank below a
  goal-aligned candidate).
- Keep the report honest: if the improved signal still yields no clear
  winner, say "no signal" rather than ranking noise.

## Files to change

| Path | Change |
|---|---|
| `src/cli/lint.rs` | Severity classification and rendering |
| `src/spec/route.rs` | Confidence; ownership-weighted affinity; scatter penalty |
| `assets/templates/proposal.md` | Conditional routing table |
| `assets/commands/givn-propose.md` | Routing rule text for the no-signal case |

## Executable-spec impact

- `givn/specs/verify-change/lint-command.feature`: scenarios for the
  severity labels and for advisory findings not affecting the exit code.
- `givn/specs/fragments/spec-explore-change.feature`: scenarios for route
  confidence, the no-signal case, and the ownership-weighted ranking.
- `givn/specs/fragments/template-conditional-rendering.feature`: scenario
  for the conditional routing table if that mechanism is used.
- `givn/specs/fragments/instruction-integrity.feature`: update if the
  proposal instruction text changes.

## Acceptance criteria

- [ ] `givn lint` output labels every finding with its severity; advisory
      findings do not change the exit code.
- [ ] A six-way tie reports "no signal" instead of a ranked list.
- [ ] A no-signal route produces a proposal without a route recommendation
      while still recording the decision.
- [ ] A capability whose owning use-case Goal and Rules match ranks above a
      lexically similar but goal-mismatched candidate.
- [ ] The verbatim "its ranking is advisory" disclaimer is no longer needed
      because confidence is explicit.

## Out of scope

- Removing `givn spec route` or the Capability Routing decision record.
- Hard enforcement of routing correctness; the check that a decision was
  recorded stays as is.
- Re-running route during review to compare decisions (the recorded decision
  remains the declaration).
- Any new LLM-based semantic routing.

## Risks and notes

- Threshold tuning is delicate: too high and useful nudges disappear, too
  low and noise returns. Calibrate against givn's own change corpus and the
  kiekste examples cited above.
- The conditional routing table must not remove the decision requirement for
  changes where route has signal.
- Keep severity classification data-driven (a table of kinds to severities)
  so changes 2, 3, 6, and 7 register their new kinds without touching
  rendering logic.
