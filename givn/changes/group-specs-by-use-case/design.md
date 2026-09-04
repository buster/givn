# Design: group-specs-by-use-case

## Overview

Pure filesystem reorganization into the grouped layout
(`givn/specs/<group>/<cap>.feature`) plus one `group.md` per group. No
code changes: archive's `resolve_capability_group` (shipped in the
previous commit) and the overlap analysis' capability/group split make
the grouped layout the supported shape.

## Technology decisions

- `git mv` for every move so history follows files.
- `group.md` scaffolds come from `crate::gherkin::group_md`'s template,
  filled with real Actor/Goal/Main flow/Interactions prose per group —
  no placeholder text left in place.
- The change skips the `specs` artifact (`.givn-skip`): there is no delta
  scenario content; reorganization is the deliverable.

## Step Definitions

None. No scenarios are authored or modified by this change; no step
definitions are added. The full suite (unchanged) is the verification.

## Architecture Impact

- `givn/specs/<group>/<cap>.feature` becomes the canonical layout.
- `givn/specs/<group>/group.md` becomes the durable home of each group's
  Actor/Goal/Interactions narrative (Phase 3 will gate e2e counts against
  it).
- archive merge/retire now resolves the group for every existing
  capability.
