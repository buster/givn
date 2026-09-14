# Change seed: archive-change

- **Stable use-case ID:** `archive-change` (existing, modified)
- **Topic source:** `givn/ideation/multi-scm-support/`
  (`use-cases/archive-change.md`, eventstorming C1-C10 and E1-E7)
- **Confirmed Persona IDs:** `developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`
- **Accepted decisions:**
  - Q3: mirrored strictness — Fossil `EXTRA` files count as dirty, so
    "an untracked file blocks the archive" stays true on both backends.
  - Q5: givn publishes the candidate revision (containing the unbound pending
    receipt) and the finalization revision; per-scenario revisions stay with
    the developer/agent.
  - Q13: the hook-rejection promise is "unproven and repairable", not an
    identical internal state; the existing Git scenario wording becomes
    backend-neutral.
- **Open questions:** Q10 (vocabulary affects wording only).
- **Handoff status:** ready for `/givn-propose` after seed 1.
- **Scope note:** adds the Fossil write path for the archive proof: clean
  witness, candidate publication, pending/proven receipt binding, evidence-only
  finalization, refusals, forward repair. Modifies
  `givn/specs/archive-change/archive-proof.feature`.
