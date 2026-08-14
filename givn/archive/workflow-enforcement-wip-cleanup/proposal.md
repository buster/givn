# Proposal: workflow-enforcement-wip-cleanup

## Problem / Opportunity

Die permanente Workflow-Enforcement-Spec enthält 16 Szenarien, die mit
`@wip` markiert sind. Die Szenarien und ihre Bindings sind bereits grün, aber
`@wip` signalisiert weiterhin, dass sie noch nicht fertig seien. Dadurch
werden fertige Prüfungen aus dem normalen Workflow ausgeschlossen und der
Zustand der Spec ist für Autoren und Reviewer irreführend.

## Proposed Solution

Die 16 bereits implementierten Workflow-Enforcement-Szenarien werden als
fertiggestellt behandelt. Ihre `@wip`-Markierungen werden entfernt, ohne
Szenariotitel, Schritte oder beobachtbares Verhalten zu ändern. Nach dem
Archivieren enthält die permanente Spec nur aktive Szenarien, und die
vollständige Spec-Suite bleibt grün.

## Out of Scope

- Neue Workflow-Enforcement-Regeln oder neue Szenarien.
- Änderungen an den Rust-Bindings, dem givn-Binary oder dem Test-Runner.
- Änderungen an `@wip`-Text, der nur innerhalb von Fixture-Inhalten anderer
  Szenarien vorkommt.
- Bereinigung von `@wip`-Markierungen in anderen permanenten Specs.
- Änderungen an archivierten Specs.

## Open Questions

- Keine. Die betroffenen Szenarien sind durch ihre vorhandenen Bindings bereits
  ausführbar; die Änderung aktiviert ausschließlich deren bestehenden Zustand.
