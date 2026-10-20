# Proposal: add-readability-report

## Release note

Givn can run an opt-in readability report that shows the structure of an
existing codebase and the units that need a plan or exception.

## Use-Case Context

- Use-case ID: `assess-code-structure` (new; proposed by this change)
- Ideation topic: `readability-experiment`
- Seed path: none — the topic was developed directly; evidence is under
  `givn/ideation/readability-experiment/reports/`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — confirmed for the topic on 2026-10-19

## Problem / Opportunity

Givn's review and archive gates check behaviour and process, not whether the code
stays understandable as it grows. The readability experiment measured the three
Givn-created projects:

- `Engine` is 11,834 code lines, 257 methods, LCOM4 11.
- `SqliteControlStore` is 3,706 code lines; `MemoryControlStore` 2,366;
  `SetupWizard` 1,560; `LocalFossil` 1,105.
- Four historical crossings reached 1,000 physical lines within 0-4 days of
  crossing 500; one file was added at 697 code lines.
- File moves did not split types: `workflow/preview.rs` remains `impl Engine`;
  `Engine`'s LCOM4 stayed 11 before and after the preview extraction.

A maintainer has no project-wide view of this structural debt. The report gives a
snapshot of the current tree, not a stored classification. The later review gate
(second change) re-evaluates the units a change touches at the change base and
head, so no historical record is needed to distinguish new debt from existing
debt.

## Proposed Solution

When the readability experiment is enabled for a project, the maintainer can ask
Givn for a readability report. The experiment is enabled by a default-off addon
named `readability`; a project opts in with
`givn addons enable readability`.

The report scans production sources and reports, per source unit:

- code size, public surface, cohesion and coupling indicators, and method
  complexity;
- a classification: god type, interface/contract, multi-responsibility module,
  multi-type module, data-only type, cohesive-large-type, or method-length;
- the action: watch, plan, or exception;
- the evidence for the classification.

The report is advisory. It does not change review or archive behaviour, does not
block a change, and stores no versioned watchlist or historical classification.
It prints to standard output; a machine-readable form is available for tooling,
and an explicit output path may be requested. The report is deterministic and
offline: no model, no network. The report and its machine-readable form are
independent of the analysis engine, so a later language engine can produce the
same report without changing its format. When the experiment is disabled, no
command or behaviour is added.

## Capability Routing

`givn spec route` reported no signal: the top candidates tie at 16.00 and no
ranked list is printed.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `readability-report` | no signal | `NEW in assess-code-structure` | No existing use case owns a project-wide code-structure report. `maintain-project` is a summary and cannot own a direct capability. `verify-change` is a per-change review, not a project-wide report. |
| `enable-optional-addons` | no signal | `MODIFIED in initialize-project` | The addon-default rule changes from all-known to default-on; `readability` is the first default-off addon. |

## Out of Scope

- The review gate and its `review.md` enforcement; that is the second change,
  `add-readability-review-gate`.
- A stored baseline, watchlist, or historical classification. The report is
  generated on demand; the later gate uses change-relative analysis.
- Making the experiment default-on or permanent before the feedback review.
- Languages other than Rust in the first report. Tree-sitter is a later adapter
  behind the same report contract.
- Automatic refactoring or code movement.
- An LLM or network dependency inside Givn.
- Test and tooling sources in the first report; production sources first.
- Any change to existing review, archive, or verification behavior.

## Open Questions

### D1 — How is the experiment enabled?

- Status: answered 2026-10-19 (user)
- Answer: **default-off addon `readability`**. The addon is registered but not
  enabled by `givn init`; `givn addons enable readability` turns it on. The
  design implements the default-off flag and the `givn init` behavior.

### D2 — Is the report stored or generated on demand?

- Status: answered 2026-10-19 (user)
- Question: should the report be a versioned baseline, local state,
  documentation, or an on-demand report with no stored classification?
- Options:
  1. **Versioned baseline** — a committed report and watchlist. Gain: shared
     existing-debt record. Cost: stale classifications, merge conflicts, frozen
     model/rules, migration when the rules change.
  2. **Local baseline** — ignored local state. Gain: no repository noise. Cost:
     not shared; lost on clone; the later gate cannot rely on it.
  3. **Documentation under `docs/`** — a durable report, not machine state.
     Gain: visible. Cost: no stable machine contract.
  4. **On-demand report, no stored baseline** — generate the report when asked;
     the later gate evaluates the touched units at the change base and head.
     Gain: no stale state; current model/rules; no merge conflicts; the owner
     re-decides per change. Cost: repeated classification when an already
     bloated unit is touched; base analysis in the gate.
- Answer: **On-demand report, no stored baseline.** The report is a snapshot for
  humans; the later review gate uses change-relative analysis. Rejected:
  versioned baseline because it freezes an older model and rules and adds
  committed state; local baseline because it is not shared and is lost on
  clone; documentation-only because it has no stable machine contract.

### D3 — What is the language scope and analyzer contract?

- Status: answered 2026-10-19 (user)
- Answer: Rust-only first with a neutral analysis contract. The first adapter is
  `syn`; a later tree-sitter adapter must produce the same report without
  changing the report format, metrics, classification, or later review markers.
  `syn` types stay inside the adapter; metrics and classification run above the
  contract. Source: user decision 2026-10-19.

### D4 — What is the feedback and exit rule?

- Status: answered 2026-10-19 (user)
- Answer: Owner-judged exit with a manual feedback record. The `readability`
  addon stays default-off; the owner judges after using the report on real
  changes and records feedback manually in the ideation topic. No ledger is
  added now; `add-readability-review-gate` owns any structured ledger.
  Promotion to default-on/opt-out, removal, or full integration is a later
  decision. Source: user decision 2026-10-19.
- Owner: user
- Deciding artifact: `design.md` and the experiment record
- Options:
  1. **Bounded trial** — after a fixed number of changes and one owner review,
     decide. Gain: clear end. Cost: the number is arbitrary.
  2. **Ledger-driven owner review** — keep using the feature until the owner
     sees enough evidence; no fixed count. Gain: flexible. Cost: no end date.
  3. **No structured feedback** — decide from the reports alone. Gain: less
     work. Cost: no record of false positives or owner judgment.

### D5 — What are the report output and names?

- Status: carried
- Owner: user
- Deciding artifact: `design.md`
- Options:
  1. **Print to stdout; `--json` for machine output; `--out <path>` optional.**
     Gain: no stored state; composable. Cost: the user chooses where to keep a
     snapshot.
  2. **Always write a file; print the path.** Gain: easy to share. Cost: an
     implicit storage decision.
  3. **Only JSON.** Gain: tool-friendly. Cost: less readable by default.
- Names to decide: addon `readability`, command `givn readability report`, and
  the later review markers.
