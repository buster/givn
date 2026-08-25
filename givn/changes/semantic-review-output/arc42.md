# Arc42 Impact Assessment: semantic-review-output

This change reworks advisory review reporting into a decision-first policy:
default text lists only model-recommended second-stage pairs, `--verbose`
lifts all listing limits, `--format json` emits one structured review
document, and a failed similarity gate persists that document beside the
change. It touches only the existing retrieval reporting path; no new
building block, dependency, deployment surface, or architecture decision.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | No | Product goals and positioning unchanged; this is reporting usability and auditability within an existing capability. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint changes; no new dependencies. | |
| 3 | 03 context-and-scope | Yes | The CLI boundary gains `--verbose`/`--format json` on advisory review, and the change tree gains the persisted findings document. | Extended the retrieval-commands interface row with the review output modes and the artifacts row with failed-review findings documents. |
| 4 | 04 solution-strategy | No | Strategy (local retrieval, deterministic E5 gate, non-authoritative second-stage evidence) is unchanged. | |
| 5 | 05 building-block-view | Yes | `spec::retrieval` gains the decision-first reporting responsibility (visibility filter, JSON document builder, findings-file writer). | Extended the `spec::retrieval` responsibility row with the reporting policy. |
| 6 | 06 runtime-view | Yes | The advisory review flow now has three output modes, chatter gating, and a findings write on gate failure. | Added section 6.8.4.1 documenting default/verbose/JSON behaviour, persist-on-failure-only, and the deprecated environment alias. |
| 7 | 07 deployment-view | No | Binary, feature builds, model caches, and hosts unchanged. | |
| 8 | 08 crosscutting-concepts | No | No new cross-cutting rule: JSON stays on stdout, warnings on stderr, matching the existing channel conventions; the visibility policy is specific to advisory review and lives in 6.8.4.1. | |
| 9 | 09 architecture-decisions | No | No ADR: flag-surface choice, confidence ordering, and persist-on-failure-only are observable behaviour owned by the Gherkin delta plus design.md; none fixes a boundary or passes the durable-consequence dimension. | |
| 10 | 10 quality-requirements | Yes | Review evidence usability and failed-review auditability become explicit quality scenarios. | Added Q-65 (decision-first default output, verbose completeness) and Q-66 (findings persistence matching stdout). |
| 11 | 11 risks-and-technical-debt | Yes | The deprecated report-limit alias is retained as parsed-but-ignored compatibility debt, and corroborated gating introduces a recall risk when both models filter a true duplicate. | Added D-14 recording the shim and its repayment path; added R-39 with the fail-safe and evidence-preservation mitigations. |
| 12 | 12 glossary | Yes | Two new domain terms enter the ubiquitous language. | Added "Decision-first reporting" and "Findings document". |

## Notes

- Retained text lines keep byte-identical wording; only their visibility
  window changes, so existing chapter wording about line formats stays valid.
- When both second-stage layers participate, the raw E5 candidate list is
  verbose-only detail; the default output leads with the recommendation
  worklists and accounting counters. Fallback runs (no participating second
  stage) keep today's output byte-identical, including the raw list.
- Corroborated gating (union of BGE/NLI recommendations) replaces the
  threshold-only verdict on both surfaces whenever both layers participate;
  the E5 threshold stays binding as fail-safe fallback. The blocking report's
  unresolved-set already exempted filtered pairs, so its contract is
  unchanged.
- Token-cap guidance lines are status-class content: they stay visible in
  default output so the shorten-or-split remediation loop keeps its signal.
- In JSON mode the index-refresh banner content embeds in the document as an
  `index` summary; the review document extends the duplicate-audit JSON
  contract rather than introducing a second dialect.
- The NLI truncation disclaimer now prints only when truncation actually
  occurs, which after this change is never in supported modes.
- Design review resolved four findings into the decisions above; the D-9 id
  collision with the pre-existing rule-nested debt row was fixed by taking
  D-14. A stakeholder decision then moved gate semantics into scope: union
  signal, both surfaces, fail-safe fallback.

STATUS: DONE
