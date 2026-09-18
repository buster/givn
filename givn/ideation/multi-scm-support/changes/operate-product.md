# Change seed: operate-product

Status: implemented in `operate-product`

- Use-case ID: `operate-product` (existing, modified)
- Topic: `multi-scm-support`
- Personas: `agent--technical-role`, `developer--primary-user`
- Suggested start: after `maintain-project`; could be folded into it
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Backend-neutral upgrade-revision wording

- Context: the upgrade-revision content scenario named Git mechanics.
- Options:
  1. **Reword to "the latest revision contains …"** — backend-neutral
     wording; no new runtime behavior and no new scenario.
     In practice: the scenario reads the same on Git and Fossil.
     Gain: one contract statement.
     Cost: none.
  2. **Keep the Git wording** — per-backend scenario variants.
     In practice: two scenarios diverge.
     Gain: none.
     Cost: duplicated behavior statements.
- Chosen: **Option 1**. Rejected: Option 2.

## Open questions

None.

## Scope note

Reworded the upgrade-revision content scenario in
`givn/specs/operate-product/run-command.feature`.
