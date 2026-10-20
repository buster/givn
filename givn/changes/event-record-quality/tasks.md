# Tasks — event-record-quality

Release note: Make development records unambiguous by recording one usage representation per request, explicit answers to questions, bounded failure text and known model identity.

## Setup

- [x] Add the additive intake fields and the recorder error path.

## Scenario tasks

- [x] Scenario: A linked usage entry is marked derived
  - [x] COMMIT: ae4d333a05f7576131b71f8b7fd9f42045167bd7 -- feat(development-record): mark a linked usage entry as derived

- [x] Scenario: An answered question records selection, source, and time
  - [x] COMMIT: 726902e5ee91d201bbfc3137f0b6142d81f6d52f -- feat(development-record): record an answered question

- [x] Scenario: A failed command records a bounded error line
  - [x] COMMIT: 637f57d5322253bb869fd9c6ecc4429b2e52fec0 -- feat(development-record): record a bounded error

- [x] Scenario: Usage identity names provider, model, and variant
  - [x] COMMIT: 149449d7640b808ef9602dac5cdbe25136e5c492 -- feat(development-record): record identity

## Readiness

- [x] Full suites and review artifacts.
