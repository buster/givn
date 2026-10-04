# Fragment: support

## Capabilities

- generate-ideation-guidance
- match-guidance-to-enabled-addons
- guide-domain-modeling
- find-existing-scenarios
- measure-coverage-per-project
- verify-repository-ownership
- coverage-readme-example
- route-a-change
- show-coverage-in-the-readme
- refresh-generated-guidance
- review-architecture-documentation
- instruction-integrity
- choose-project-license
- adr-qualification-gate
- attribute-commits
- enable-commit-attribution
- detect-the-project-repository
- customize-the-lifecycle-graph
- rules-policy
- local-only-transition
- publication-cleanliness

## Interactions

The reviewed instruction-integrity inventory has two actions: request effective
instructions with an instruction id, and request session-start instructions
without one. Design, review, specs, dialog, JSON, and change context are input
variants of those actions, not separate E2E actions. The retained JSON E2E
requests specs only; it is not exercised dialog evidence. Other support
capabilities and their contracts are unchanged.

| Capability | Consumer action | E2E scenario |
|---|---|---|
| instruction-integrity | request effective instructions by id (artifact, general policy, JSON and change context are variants) | An agent requests machine-readable artifact guidance |
| instruction-integrity | request normal session-start instructions | Author starts a session with the universal dialog obligation |

## Diagram

```mermaid
flowchart LR
  Consumer((Use cases)) -.->|include| Support[[Reusable support behavior]]
```

<!-- givn:base-sha256:db311dbecc71d458d4c2798500191f6e535506dcbef9a8dd9e69053fd2a72888 -->

