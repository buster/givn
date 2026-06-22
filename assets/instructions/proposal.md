# Instruction: proposal

Write a proposal that describes the WHAT and WHY of this change, in domain
language that a non-technical stakeholder can read and validate.

**Rules (enforced by default):**
- Describe observable behaviour, not implementation. No class names, function
  signatures, HTTP routes, DB schemas, step-definition mechanics, or framework
  references.
- The only permitted implementation-flavoured content is an aspect that is
  *itself* an externally observable requirement (e.g. "the response is valid JSON",
  "data persists across restart", "responds within 200 ms"). Heuristic: if it
  would still be true if the system were reimplemented in a different language
  and framework, it belongs in the spec; otherwise it belongs in design/tasks.
- Write from the user's perspective. Use domain language (business terms, not
  technical terms).

**Structure:**
1. Problem / Opportunity — what is wrong or missing?
2. Proposed Solution — what should the system do, observably?
3. Out of Scope — what is not changing?
4. Open Questions — unresolved decisions before moving to specs.
