# Persona: Primary user (developer / maintainer) — topic record

Reuse: developer--primary-user, base 7bf38024; this file adds record entries only

## Running record

- 2026-11-13 — Need: `ise` stays opt-in, so an ordinary feature change pays
  no new ceremony. (Frame, vision)
- 2026-11-13 — Need: a replacement keeps persisted user data and state across
  versions, not just compilable code. (Frame, Ise consequence "Data, state, and
  contracts outlive the code")
- 2026-11-13 — Need: `@e2e` suites strong enough to rebuild a module or the
  whole product without reading the old source. (Frame, gaps G4/G7)

## Topic stances

- 2026-11-13 — ise-rebuildability: a replacement that loses user data, or
  needs a manual state migration step the specification never named, is not a
  successful replacement.
- 2026-11-13 — ise-rebuildability: a clean-room rebuild and a single-unit
  replacement must be runnable from the same commands on Git and Fossil; the
  SCM difference stays inside givn. (open: Q3)
- 2026-11-13 — ise-rebuildability: wants the ordinary feature workflow
  unchanged until `givn addons enable ise`, and no gate that fires on projects
  that never opted in.
