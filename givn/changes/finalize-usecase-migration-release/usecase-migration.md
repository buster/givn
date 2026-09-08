# Givn Use-Case Migration Ledger

The pre-migration snapshot was generated from the parent of commit `92975e3`,
the commit that introduced the canonical use-case corpus. The after snapshot is
the current permanent corpus. Unchanged behavior remains accounted for by the
behavior-hash comparison; the rows below record the intentional semantic
exceptions visible in the comparison.

| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|
| all unchanged entries | legacy use-case directories | MOVE | canonical use-case or fragment owner | unchanged capability | preserved | preserved | no | semantic ownership migration |
| `support/spec-explore-permanent.feature — Lexical search locates a permanent scenario by a step text token` | support | RETIRE | none | none | yes | no | yes | assertion vocabulary changed from group ownership to use-case ownership |
| `support/spec-explore-permanent.feature — The spec tree reports per-group counts and the group narrative` | support | RETIRE | none | none | yes | no | yes | replaced by the use-case/fragment contract |
| `support/spec-explore-permanent.feature — Discovery commands never modify permanent files` | support | RETIRE | none | none | no | no | yes | fixture vocabulary changed from groups and `group.md` to use cases and `usecase.md` |
| `archival/grouped-merge-for-new-capabilities.feature — Consumer archives a new capability and the merged spec lands in its declared group` | archival | RETIRE | none | none | yes | no | yes | routing assertion changed from group ownership to use-case ownership |
| canonical use-case contract | none | CREATE | make-usecases-canonical | canonical-usecase-contract | none | yes | yes | new release contract |

The four retired entries are intentional contract replacements, not silently
dropped behavior. Their current use-case vocabulary is covered by the active
canonical corpus and the archived canonical migration feature.

## Source evidence

The source coverage receipt records a line-coverage ratio decrease from
`7162/13704` to `7439/18235`. Branch counters are `0/0` in both reports, so
the branch dimension is not materially measured. This remains a blocking
release finding until the source coverage baseline is reconciled or the added
canonical migration surfaces receive sufficient executable coverage.
