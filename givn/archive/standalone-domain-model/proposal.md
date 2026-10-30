# Proposal: standalone-domain-model

## Release note

Domain modeling now keeps the project's glossary and structural model in standalone documents that work without arc42.

## Use-Case Context

- Use-case ID: `obtain-effective-guidance` — supporting owner: `initialize-project`.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `agent--technical-role`, `developer--primary-user`, `givn-maintainer--owner` — their existing authority and goals apply; no Persona is invented or reconfirmed.
- Source: `docs/improvements/0.9.12/domain-model.md` records the problem evidence and confirmed direction. The operator resolved the open vocabulary, migration, template-delivery, and scaffold-trigger decisions in this proposal.

## Review Context

- Review kind: self-review
- Reason: served-guidance and initialization-scaffolding behavior; no new command, no configuration schema, and no executable product behavior beyond creating missing documentation from embedded templates.

## Problem / Opportunity

Givn currently tells agents that the only durable domain knowledge lives in the arc42 glossary. The domain-modeling skill is generated and routed only when both `domain_modeling` and `arc42` are enabled, and the served instruction requires `docs/arc42/12-glossary.md` before any term can be recorded.

That placement hides two different navigation needs behind one vocabulary table:

- A reader asking what a term means should consult a glossary.
- A reader asking how aggregates relate, where a consistency boundary lies, or what an external system is should consult a domain model.

The inspected harness output shows aggregate boundaries, invariants, and lifecycle events inside `12-glossary.md`. The shipped chapter template itself has only a term/definition table, so the richer content sits behind a misleading title. A project that disables arc42 cannot model its domain at all, even though the structural knowledge is not architecture documentation.

User impact: domain structure is hard to find, projects without arc42 lose the vocabulary discipline, and changes that alter a boundary have no durable place to record it.

## Proposed Solution

Givn now treats vocabulary and structure as two durable documents with separate owners:

- `docs/glossary.md` is the project's sole vocabulary authority in every project. When arc42 is enabled, `docs/arc42/12-glossary.md` is a glossary heading and a link to that file, not a second definition table.
- `docs/domain-model.md` is the project's durable structural model: bounded contexts, collapsed aggregates, external systems, boundaries, and relationships. It links to governing use-case contracts and specifications for behaviour instead of copying rules. It is usable when arc42 is disabled.
- A change never receives a per-change copy of either document. Repeated domain modeling reuses the durable files and edits them only when their maintained knowledge changes.

The durable documents are scaffolded from shipped templates, never overwriting existing content:

- `givn init` creates `docs/glossary.md` when either the arc42 or domain_modeling addon is effective, and `docs/domain-model.md` when domain_modeling is effective.
- `givn addons enable arc42` creates the glossary when it is missing.
- `givn addons enable domain_modeling` creates both files when they are missing.
- A project with every addon disabled receives neither file.
- Existing `docs/glossary.md`, `docs/domain-model.md`, and arc42 chapter files are preserved byte-for-byte.

The domain-modeling skill is generated and routed whenever the domain_modeling addon is enabled, independent of arc42. Its served instruction owns the document lifecycle and the two ownership checks:

- **Ownership:** vocabulary belongs in the glossary; structure belongs in the domain model; behaviour belongs to use-case contracts and specifications.
- **Change impact:** edit the durable model only when this change altered structure or an included explanatory view; do not manufacture no-op edits, timestamps, or boilerplate rewrites.

The structural template carries a scope/status section, a separate notation legend, one Mermaid overview scaffold using top-down layout, collapsed aggregate and external-system notation, boundaries and responsibilities tables, relationships with a link to each governing contract, and open modeling questions. It does not require lifecycle, event, policy, aggregate-detail, or value-object sections. A rendered-diagram review is required; syntactically plausible Mermaid source is not evidence of readability.

Existing vocabularies migrate by agent direction, not by an automatic rewrite. The served guidance requires an agent that finds term definitions in `docs/arc42/12-glossary.md` to move the entries into `docs/glossary.md`, preserve their wording and ordering, and leave chapter 12 as a link. A missing glossary is created only when the scaffold trigger applies. Givn never silently drops consumer-authored definitions.

Event Storming, design-review, review, and arc42 guidance point at the correct owner: glossary for vocabulary, domain model for boundaries and relationships, use cases and specifications for behaviour.

## Capability Routing

`givn spec route --change standalone-domain-model` reports `EXTEND enable-optional-addons` (signal), with `initialize-a-project` as the next candidate. That is the initialization owner, not an instruction to concentrate unrelated guidance content there.

| Proposed capability | Route's recommendation | Decision | Rationale (if deviated from route) |
|---|---|---|---|
| `guide-domain-modeling` | `EXTEND enable-optional-addons` | `EXTEND guide-domain-modeling` | The instruction content, template lifecycle, vocabulary/structure ownership, and migration guidance are owned by the existing domain-modeling capability. |
| `match-guidance-to-enabled-addons` | `EXTEND enable-optional-addons` | `EXTEND match-guidance-to-enabled-addons` | The domain-modeling skill no longer depends on arc42; conditional glossary guidance is owned by the support fragment. |
| `enable-optional-addons` | `EXTEND enable-optional-addons` | `EXTEND enable-optional-addons` | Scaffolding `docs/glossary.md` and `docs/domain-model.md` on init and addon enable belongs to `initialize-project`. |
| `review-architecture-documentation` | `EXTEND enable-optional-addons` | `EXTEND review-architecture-documentation` | Review and design-review must check the right document owner and the structural-impact assessment. |

## Out of Scope

- Automatic migration or Markdown parsing of existing glossary content; migration remains agent-directed and preservation-first.
- A new command, configuration schema, artifact, lifecycle gate, or generator that derives the model from Gherkin.
- A per-change domain artifact or a copy of the durable documents inside `givn/changes/<id>/`.
- Enforcement that a project must have multiple bounded contexts, an external system, or every supported notation type.
- Rendering guarantees for Mermaid hosts that do not support the requested layout; the guidance states the limitation instead of promising identical output.
- Changing the domain-modeling addon default or the change lifecycle order.

## Open Questions

### D1 — Where does the durable vocabulary live?

- Status: answered 2026-10-30 (user)
- Question: should the glossary stay in arc42 and fall back to use-case contracts when arc42 is disabled, or move to a standalone `docs/glossary.md`?
- Options:
  1. **Keep the arc42 glossary, use-case fallback** — smallest change; no cross-use-case vocabulary page without arc42. In practice: a project without arc42 finds terms only in each owning use case.
  2. **Standalone `docs/glossary.md` for every project** — one vocabulary authority independent of addons; arc42 chapter 12 links to it. In practice: a project without arc42 still has one glossary, and chapter 12 carries no duplicate table.
- Disposition: answered — option 2. `docs/glossary.md` is the sole vocabulary authority; chapter 12 becomes a link. Rejected: the arc42-only glossary leaves projects without arc42 without a durable cross-use-case vocabulary source and keeps vocabulary coupled to optional architecture documentation.

### D2 — How does an existing arc42 glossary migrate?

- Status: answered 2026-10-30 (user)
- Question: should migration be agent-directed or an automatic CLI rewrite?
- Options:
  1. **Agent-directed with preservation rules** — the agent moves entries into `docs/glossary.md`, preserves wording and ordering, and replaces chapter 12 with a link. In practice: this repository's table moves when an agent follows the guidance.
  2. **Automatic CLI rewrite** — `givn init`, addon enable, or upgrade transforms the files deterministically. In practice: every project reaches the new layout without agent cooperation.
- Disposition: answered — option 1. No CLI rewrites consumer documentation. Rejected: automatic Markdown transformation risks dropping arbitrary tables or prose, writes user-owned files outside an explicit request, and adds recovery obligations to this change.

### D3 — How do the templates reach the authoring agent?

- Status: answered 2026-10-30 (user)
- Question: embed the templates in the served instruction, serve them through new policy instruction ids, or scaffold the files automatically?
- Options:
  1. **Embed in the instruction** — one request carries rules and scaffolds; no new ids.
  2. **New policy instruction ids** — separate template assets fetched by explicit id.
  3. **Scaffold empty files automatically** — Givn writes the durable files from embedded templates at init or addon enable.
- Disposition: answered — option 3. The templates ship as embedded assets and materialize as project files without a fetch step. Rejected: embedding bloats the served instruction and leaves the template indivisible from the rules; new policy ids add public instruction names and extra requests.

### D4 — Which addon state triggers scaffolding?

- Status: answered 2026-10-30 (user)
- Question: scaffold both files unconditionally, or only when their governing guidance is effective?
- Options:
  1. **Scaffold when governing guidance is effective** — glossary when arc42 or domain_modeling is enabled; domain model when domain_modeling is enabled. In practice: `givn init --no-addons` writes neither file.
  2. **Scaffold both at every init** — even an addon-free project receives empty domain documents. In practice: `--no-addons` still writes both files.
- Disposition: answered — option 1. Files exist exactly when guidance points at them; enabling an addon later fills a missing file. Rejected: unconditional scaffolding leaves unused domain documents in opt-out projects and weakens what `--no-addons` means.

### D5 — How honest must Mermaid host compatibility be?

- Status: answered by the improvement direction
- Question: should the guidance promise identical rendering across Markdown hosts, or state the ELK limitation?
- Disposition: answered — the template requests the top-down ELK layout and the guidance says a host must include the ELK renderer and honour the configuration; unsupported hosts may fall back or fail. A rendered-diagram review is required. No D2 or browser installation is required for ordinary readers.

### D6 — How does a review require structural maintenance without another artifact?

- Status: answered by the improvement direction
- Question: add a structural-impact report, or fold the assessment into existing review guidance?
- Disposition: answered — fold it into the existing review and design-review guidance. The reviewer asks whether the change altered structure; a no-op edit, timestamp, or boilerplate rewrite is not evidence. No new marker, artifact, or gate.

### D7 — How much template structure is required?

- Status: answered by the improvement direction
- Question: require every notation type, or keep the template minimal and honest about unknowns?
- Disposition: answered — keep the candidate thin-map template. Inapplicable sections are removed, unknown structure stays qualified, and no bounded context, external system, aggregate, event, policy, or lifecycle section is invented to complete the document.
