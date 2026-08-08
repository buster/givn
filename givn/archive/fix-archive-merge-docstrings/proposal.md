# Proposal: fix-archive-merge-docstrings

## Problem / Opportunity

When a change is archived, its specification files are merged into the
project's permanent spec files. An archived change that modifies an existing
scenario (tagged `@givn.modified`) has its scenario block replaced by the new
text. However, the replacement discards any docstring blocks (the `"""..."""`
sections attached to a step). Scenarios whose steps carry docstring arguments
become non-functional after archive: the steps lose the content they were
meant to receive.

There is currently no clean outcome:

- If the archive's verification gate runs before fixing this, it fails because
  the merged scenarios no longer carry their docstrings, so the archive is
  blocked.
- The only workaround is to restore the permanent spec files from version
  control immediately after archiving — which hides the corruption instead of
  fixing it.

## Proposed Solution

When a change is archived, any scenario merged into the permanent specs keeps
every part of its steps intact — the step text, its docstring block content,
and any data-table content. After archiving, a scenario that used docstrings
still has them: nothing is lost, the specified behaviour is unchanged, and the
verification gate passes without manual restoration.

## Out of Scope

- No change to how steps are matched or executed.
- No change to the delta marking tags or archive gating rules.
- No reformatting of existing specs beyond preserving the content that the
  merge currently drops.

## Open Questions

- Should data tables attached to steps also be preserved, or only docstrings?
  (Both are dropped today by the same reconstruction step.)
