# Proposal: arc42-review-alignment

## Problem / Opportunity

Arc42 is durable project documentation, while a change's design, task list, and
implementation are temporary and continually evolve. The existing Arc42 check
at design review verifies the planning assumptions at one point in time, but it
does not verify that the completed work still follows those assumptions. A
change can therefore pass review while its design or implementation contradicts
the architecture documentation, or while the documentation remains stale.

## Proposed Solution

When the Arc42 addon is enabled, the completion review includes a mandatory
Arc42 conformance block. The reviewer compares the durable Arc42 chapters and
the change-level Arc42 impact assessment with the change's design, task list,
and completed implementation. The block records the relevant architectural
facts and decisions, whether the design and tasks account for them, whether the
implementation follows them, and evidence for each conclusion.

The review identifies both directions of drift: architecture changes that are
missing or stale in Arc42, and documented architecture facts that are ignored
or contradicted by the design, tasks, or implementation. Any mismatch is a
finding that prevents a passing review until the affected documentation or
change is corrected and rechecked.

When the Arc42 addon is disabled, the additional block is explicitly marked
not applicable and does not add a review requirement.

## Out of Scope

- Automatically inferring architectural facts from source code.
- Replacing the existing design-review Arc42 impact assessment.
- Changing the Arc42 chapter structure or scaffold format. Factual updates to
  the affected durable chapters are part of this change so the new review rule
  is itself documented.
- Adding a new archive artifact or changing the archive merge behaviour.

## Open Questions

None. The conformance block is a review obligation and remains human-readable;
the existing Arc42 integrity checks continue to handle mechanical scaffold and
change-level completion checks.
