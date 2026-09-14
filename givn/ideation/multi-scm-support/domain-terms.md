# Domain terms: multi-scm-support

Terms as they crystallize. Candidates are merged into the durable term list
at handoff by user decision.

| Term | Candidate definition | Collision or note |
|---|---|---|
| SCM backend | The concrete version-control system givn talks to in a project: Git or Fossil. | Anti-term: provider, driver (implementation words). |
| Revision | One immutable recorded state of the project in the active SCM. Git: commit hash. Fossil: check-in hash. | Collides with Git "commit" and Fossil "check-in"; the domain needs one neutral term. |
| Checkout | The working directory givn operates in, as seen by the active SCM. Git calls it a worktree; Fossil calls it a checkout. | Collides with Git "worktree"; the domain currently says worktree. |
| Repository root | The directory the SCM considers the project boundary. Git: worktree top level. Fossil: the directory holding `.fslckout`/`_FOSSIL_`; the repository file itself lives elsewhere. | TC-32 currently equates this with the Git worktree top level. |
| Clean state | The state in which the SCM reports no changes that would enter the next revision. | Git and Fossil disagree on whether untracked/`EXTRA` files count. |
| Candidate revision | The revision that publishes the merged archive before verification. | Currently `candidate_commit` in the receipt. |
| Tested revision | The revision against which every verification scope passed. | Currently `tested_commit` in the receipt. |
| Revision binding | The receipt property that the tested revision is the published candidate revision. | Currently expressed as `HEAD == tested_commit`. |
| Hook | An extension point of the active SCM. Git offers `commit-msg`; Fossil 2.26 offers only `before-commit` and `after-receive`. | Collides with givn's own artifact `post` hooks; never use the bare word without the qualifier. |
| Index / staging area | A Git-only concept: the intermediate area that `git add` fills and `git commit` consumes. Fossil commits named paths directly. | The domain must not require an index; "publish a subset" is the portable concept. |
| Upgrade revision / plan revision | The two automatic revisions of a managed upgrade, in neutral vocabulary. | Collides with the glossary's "Upgrade Commit A" / "Migration Commit B" and the `upgrade_commit` metadata key — Q10. |
| Check-in (Fossil) | One immutable recorded revision in Fossil. | Unrelated to givn's `givn check` commands and mandatory checks; never shorten either to "check". |

Q10 resolved 2026-09-14: the neutral terms (`revision`, `checkout`, `clean
state`, `candidate revision`, `tested revision`, `revision binding`, `upgrade
revision`, `plan revision`) were merged into the durable term list
(`docs/arc42/12-glossary.md`); per-SCM terms stay in the backend seam.
