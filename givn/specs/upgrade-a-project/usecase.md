<!-- givn:base-sha256:0845502a519846c706dd1ce9598259650221abb77579dca42fb29b5ed80bb32b -->
# Use case: upgrade-a-project

## Level

!

## Actors

- Project maintainer
- Supporting: Authoring agent

## Goal

Cross a supported release gap with reviewable managed revisions, preserving user-owned content and accurately reporting recovery from a committed checkpoint.

## Trigger

The maintainer upgrades Givn.

## Preconditions

- The project configuration and repository state are available.
- Git or Fossil is selected, with a clean-state witness and a committed version-control checkpoint.
- 1.0 is the supported migration baseline; configuration format is a separate concern. During 0.9.10 preparation a manually current-format 0.x project can receive a migration-free managed refresh. Older configurations may require manual repair before managed writes.

## Main flow

1. The maintainer requests an upgrade; the system records the committed checkpoint and refuses before writes for dirty projects or missing version control.
2. The system detects the project version and validates configuration ownership. Pre-1.0 projects receive manual transition guidance, not embedded old migration bundles or compatibility aliases; unsupported old keys refuse before writes.
3. For a supported project, the system synchronizes the managed scaffold and thin skills, removes only recognizable generated legacy commands, and leaves project-owned content authoritative.
4. The system publishes the calculated allowlist-only managed revision, excluding project extensions and custom replacements; improved built-in rules are available at the next instruction request.
5. An applicable future supported migration route produces a reviewable plan recording the actual managed upgrade revision. An existing plan is preserved and reported.
6. The system reports completed writes and revisions, pending work, and any removed commands or manual transitions accurately.

## Extensions

- 1: uncommitted changes exist → refuse before writes, naming dirty paths and preserving user work; protects the minimal guarantee.
- 1: no supported version-control system or committed checkpoint exists → refuse before writes and name the checkpoint requirement; protects the minimal guarantee.
- 2: unsupported old configuration exists → name offending keys and applicable manual communication_policy, challenge, and skills-only transitions without converting them; protects the minimal guarantee.
- 3: the project is already current → report state and publish no revision; protects the success guarantee.
- 3: a project-owned replacement, extension, or customized legacy command exists → preserve it byte-for-byte and report any manual transition; protects the minimal guarantee.
- 4: writing or publication fails after some work completed → exit nonzero and identify the failed operation, checkpoint, completed writes, actual published revisions, and pending work; offer operator restoration or fix-forward without automatic reset or clean; protects the minimal guarantee.
- 5: a migration plan already exists → preserve and identify its proposal and design; protects the success guarantee.
- 6: a consumer invokes a removed command → report it unavailable rather than silently substituting another action; protects the success guarantee.

## Rules

- 1.0 is the supported migration baseline. Embedded pre-1.0 bundles and catalog routes are removed; 0.x configurations may break and need manual repair, not automatic conversion or compatibility aliases.
- Version detection and the future migration mechanism remain available for applicable supported routes; historical migration rationale remains reviewable.
- Managed upgrades use a clean committed checkpoint on both Git and Fossil. Ignored managed paths are staged only through the calculated allowlist.
- The managed revision refreshes thin skills and managed integration only; it never stages, merges, rewrites, or injects into project extensions or custom replacements and never destroys specifications or user-owned files.
- Recognizable generated legacy commands may be removed, but no new command surface is generated. Custom commands stay byte-for-byte and are reported for manual transition.
- Improved built-in rules are served at the next request without another rule-content synchronization revision.
- A future supported migration plan records the actual managed upgrade revision. A migration-free supported upgrade publishes exactly one allowlist-only revision and no plan revision.
- Local-only upgrades can remove generated guidance from tracking while keeping refreshed local skills, without modifying committed human-authored AGENTS.md.
- A failure can leave partial writes or published revisions. Report actual state and recovery from the checkpoint, distinguishing pending from completed work; never perform automatic reset/clean or promise complete rollback.
- Operator-controlled restoration or fix-forward is the recovery contract; no expanded transaction or history-rewrite subsystem is implied.

## Examples

- A clean Git project at the supported baseline receives one managed revision containing only allowlisted integration changes; a Fossil project receives the corresponding check-in shape.
- A 0.7 project receives 1.0-baseline and manual key/skills transition guidance, not an aggregate embedded 0.x plan.
- A future supported route adds a reviewable migration plan naming the actual upgrade revision; an existing proposal and design are never overwritten.
- A dirty project or project without a committed checkpoint refuses before writes.
- Publication failure after managed files were written names those files, any actual revisions, and the checkpoint; the operator restores deliberately or repairs forward.
- A custom legacy command and an extension saying "Keep proposals under 500 words" remain byte-for-byte unchanged.

## Minimal guarantee

Preflight refusal preserves project files. Later failures preserve user-owned specifications, extensions, and replacements and report actual partial state and checkpoint-based operator recovery without claiming rollback.

## Success guarantee

Supported upgrade work is explicit in version history, future migration plans are reviewable and revision-bound, and thin skills reach current effective rules while user-owned content remains unchanged; pre-1.0 transitions are explicitly manual.

## Personas

- developer--primary-user
- givn-maintainer--owner

## Capabilities

- apply-a-managed-upgrade

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| apply-a-managed-upgrade | run a managed upgrade | A clean project without migration requirements receives one managed upgrade commit |
| apply-a-managed-upgrade | invoke a retired engine command | A consumer invoking a retired engine command is told it does not exist |
| apply-a-managed-upgrade | invoke the retired history rewrite command | A consumer invoking the retired history rewrite command is told it does not exist |
| apply-a-managed-upgrade | archive a maintenance migration | A completed maintenance migration can be archived without a product specification |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Changing product specifications during an upgrade; publishing releases (`publish-a-release`).

## Diagram

```mermaid
flowchart LR
  Maintainer((Maintainer)) --> Witness[[Witness the clean state]]
  Witness --> Upgrade[[Publish the upgrade revision]]
  Upgrade --> Plan[[Publish the plan revision]]
```
