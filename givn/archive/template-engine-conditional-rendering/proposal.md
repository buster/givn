# Proposal: template-engine-conditional-rendering

## Problem / Opportunity

givn generates agent integration files — skills and slash commands — from
templates. Some of those files must serve projects with an addon enabled and
projects without it. Today that is expressed as prose inside the file
("record `N/A when the arc42 addon is disabled`"). In a project where the
addon is not enabled, that guidance stays in the generated file forever:

- It is noise: it consumes prompt tokens on every session and dilutes the
  instructions the agent actually needs.
- It describes a state of the world ("the addon is disabled") that cannot
  change while the file is being read, yet the reader must reason around it.

Separately, a skill can be generated even when a companion addon it requires is
disabled, leaving a file that is present but inert.

The user expectation: when an addon is not enabled, the generated skills and
commands should not mention that addon or its workflow at all. Not "skip these
steps because the addon is off" — simply no such content.

## Proposed Solution

At generation time givn takes the project's current addon configuration into
account, and template authors mark which sections of a template apply to which
configuration. Generation keeps only the sections that match the current
configuration, and drops the rest.

Observable behaviour:

- After `givn init` or `givn skills sync` in a project where the arc42 addon is
  disabled, the generated review skill and review command contain no arc42
  guidance; in a project where it is enabled they do.
- Enabling or disabling an addon via givn re-generates the affected files
  automatically, so they always match the current configuration.
- A skill that needs a companion addon (the domain-modeling skill needs the
  project's arc42 glossary) is generated only when both the skill's addon and
  the companion addon are enabled.
- No generated skill or command contains a reference to an addon that is not
  enabled in the project.

## Out of Scope

- Runtime instructions (`givn instructions <artifact>`): these are printed on
  demand and keep their two-state guidance.
- Routing: which skill the router points the agent at stays unchanged.
- The addon documentation itself (`docs/arc42/...`, addon descriptions).
- The workflow itself: scenario ordering, gates, and artifact rules are
  unchanged.

## Open Questions

- What template syntax is used to mark a section as conditional? (decided in
  design)
- Does "current configuration" mean only the enabled addon set, or also other
  manifest flags (e.g. selected agent targets)? (assumed: addon set)
- Do project-override templates (`givn/skills/...`, `givn/commands/...`)
  follow the same conditional behaviour as the embedded defaults? (assumed: yes)
