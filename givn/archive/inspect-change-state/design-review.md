# Design Review: inspect-change-state

## Grilling

In-session per the standing assumption directive (A3). The seam already
answers the backend question; status only renders it.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| B1 | Blocker | The SCM line shifts every human-readable line number, breaking the permanent positional assertions ("stdout line 3/4"). | The permanent scenario is aligned (+1) in the implementation commit; the delta carries the same modification. |
| N2 | Note | Selection failures other than "no backend" (ambiguity, missing executable) would report `SCM: none`. | Accepted and recorded as A19: the note names the unavailable commands, and the specific refusal surfaces from the command that needs the backend. |

## Branch dispositions

- Scope: the seed's one decision, nothing else; JSON untouched.
- Testability: the outline asserts the literal `SCM: <scm>` line; the
  no-backend scenario asserts exit 0 plus the unavailable phrase.
- Personas: the three confirmed Personas are review lenses.
- ADR: none. Arc42: chapter 06's status section (6.5) describes the output —
  updated to name the SCM line.
- Ubiquitous language: SCM backend is the glossary term.

DESIGN-REVIEW: PASS
