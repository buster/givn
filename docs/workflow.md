# Givn workflow

Givn is a spec-driven workflow. The `.feature` file is the executable
specification; its owning use case supplies the domain contract. A change
moves through planning, design review, test-first implementation, review, and
archive. State is derived from files on disk, not stored separately.

Lifecycle rendering: [`workflow.svg`](workflow.svg) (GitHub-safe) and
[`workflow.html`](workflow.html) (self-contained HTML).

```
proposal → specs → design → [arc42-docs] → design-review → tasks → review → archive
```

`arc42-docs` appears when the arc42 addon is enabled.

| Phase | Command | Produces | Leaves when |
|---|---|---|---|
| Propose | `/givn-propose` | `proposal.md`: problem, observable solution, release note, capability placement, open questions | file exists |
| Spec | `/givn-spec` | delta features under `specs/<usecase-slug>/`, tagged `@givn.delta`, `@givn.added/modified/removed`, `@wip` | delta features exist |
| Design | `/givn-design` | `design.md`: runner, strict mode, step files, decisions, interaction matrix, visual contract, local runnability | file exists |
| Design review | `/givn-design-review` | `design-review.md` ending in `DESIGN-REVIEW: PASS` | marker present |
| Tasks | `/givn-tasks` | `tasks.md`: setup task, test-first behavior batches, evidence and commit fields | tasks written |
| Implement | `/givn-implement` | production code, ticked tasks, one batch commit per behavior batch | every scenario has evidence and a commit |
| Review | `/givn-review` | `review.md` ending in `REVIEW: PASS` | marker present |
| Archive | `givn archive` | merged permanent specs, archived change, proven receipt | archive proves |

Ideation (`/givn-ideation`) is an optional pre-phase: event storming that
slices a domain into use cases and change seeds. It produces no manifest
artifact and has no gate.

## The hard gates

Only four commands stop the workflow; everything else is served guidance or
advisory reporting.

| Gate | Command | Stops |
|---|---|---|
| Change verification | `givn check tasks --change <id>` | `@wip` scenarios, missing or unreachable scenario commits, a dirty tree, a red delta suite; writes the change receipt |
| Review | `givn check review --change <id>` | missing artifacts, unsigned visual review, unresolved overlap dispositions, invalid capability placement, stale receipt, blocking lint findings |
| Archive | `givn archive --change <id>` | incomplete tasks/review, unsigned design-review, unmerged step definitions, missing e2e runner, merge conflicts, red regular or e2e scopes, coverage failure, a tested tree that moved |
| Commit | `givn commit` and the commit hook | an uncommitted or conventionally invalid revision |

`givn lint` is blocking when it is invoked: parse errors, duplicate titles,
empty features, invalid use-case documents, and interaction mismatches exit
non-zero. Review runs it for the active change.

## What guides the agent

- `givn instructions` reports the next required artifact and the skill to run.
- `givn instructions <artifact> --change <id>` serves the artifact's rules,
  with addon conditionals and project extensions.
- `givn status --change <id>` shows the checklist, task progress, and skipped
  artifacts.
- Artifact templates define required sections and sign-off markers.
- The owning `usecase.md`, confirmed Personas, and ideation seeds supply
  domain context.

## Details

[Workflow details](workflow-details.md) lists every phase, check, failure
condition, archive step, and the limits of enforcement. The
[use-case model](use-case-model.md) defines use-case identity and scoping;
[reference](reference.md) lists commands, skills, addons, and configuration.
