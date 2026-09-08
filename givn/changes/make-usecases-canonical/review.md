# Review: make-usecases-canonical

## Fabrication Audit

The implementation is covered by Rust unit tests and the repository's
Cucumber runner. The feature runner uses `.fail_on_skipped()` and the changed
use-case behavior is exercised through the real `givn spec tree` subprocess.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `usecase.md` parser, graph, and migrated corpus | yes |
| Capability ownership and typed relationships preserved | routing and graph checks | yes |
| Confirmed Persona outcomes preserved | no Personas declared by this change | N/A |
| Actors remain distinct from Persona biographies | feature and guidance contracts | yes |
| Migration evidence | coverage inventory and release bundle | yes |

## Coverage

Specification and E2E coverage is inventoried by `givn spec coverage --format
json`. Historical archives are excluded from the active corpus migration.

## Sign-off

- [x] Unit suite passes.
- [x] Feature suite passes.
- [x] Active corpus has no `group.md` narrative files.
- [x] Generated guidance is synchronized.

REVIEW: PASS
