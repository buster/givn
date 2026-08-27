# Migration Proposal: 0.3.0 to 0.4.0

## Problem / Opportunity

Givn 0.4.0 adds the automated delivery pipeline (CI, security, prepare-release, and release workflows), an auto-generated changelog, and corrected repository metadata. It changes no Givn-authored file formats, no Cargo feature contract, and no project-facing workflow. A project created with Givn 0.3.x therefore needs no domain or configuration migration to run the 0.4.0 contract — the machine-safe managed refresh already performed by `givn upgrade` is the complete operation.

## Proposed Solution

Verify that the consumer project is running the Givn 0.4.0 managed contract, that the managed upgrade commit (Commit A) contains only the expected allowlisted paths, and that no project-owned specification, architecture, or configuration surface was silently changed. Complete the normal Givn review and archive gates for this maintenance change.

This bundle is one phase in an aggregate migration plan when a project skips additional versions.

## Out of Scope

- Inventing or changing domain behavior without project evidence.
- Automatically editing project specifications, source code, tests, ADRs, Arc42 facts, ejected facets, or user-authored documentation.
- Adding a synthetic migration feature to the permanent specification tree.

## Completion

The LLM must verify the 0.4.0 managed contract and confirm no project migration is required, then complete this phase before the next bundle phase in an aggregate plan.
