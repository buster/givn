# 3. Context and Scope

## 3.1 Business Context

`givn` sits between a developer or AI coding agent and a project filesystem. It
manages Gherkin specifications, change artifacts, generated guidance, and local
review evidence.

```mermaid
flowchart LR
    user[Developer or agent] --> cli[givn CLI]
    cli --> project[Project filesystem]
    cli --> defaults[Embedded defaults]
    cli --> targets[Selected agent targets]
    runner[Test runner] -->|reads GIVN_FEATURES| project
    cli -->|runs configured commands at gates| runner
```

### External entities

| Entity | Interaction |
|---|---|
| Developer | Runs lifecycle, lint, check, archive, retrieval, and addon commands. |
| AI coding agent | Reads effective instructions, generated skills, review reports, and Arc42 docs; edits project artifacts and source. |
| Project filesystem | Stores `givn/`, source code, `AGENTS.md`, generated agent files, and durable `docs/arc42/`. |
| Test runner | Is configured in `givn/commands.yaml`; receives `GIVN_FEATURES` and executes feature scenarios. |
| CI | Runs static lint, configured test commands, mandatory checks, and archive/review gates. |
| Model artifact source and XDG cache | Supply E5, BGE, or NLI artifacts on explicit semantic use and retain them in the user's cache. No hosted inference service is part of the boundary. |

`AGENTS.md` is user-owned except for named givn fenced blocks. Givn may create or
replace the content between matching markers such as
`<!-- givn:begin:givn-project -->` and `<!-- givn:end:givn-project -->`; it does
not alter other content.

## 3.2 Technical Context - Interfaces

| Interface | Direction | Contract |
|---|---|---|
| CLI arguments | in | `clap` subcommands, options, and exit status. |
| `givn/config.yaml` | in/out | Project overlay, addon flags, version marker, and skill targets. |
| `givn/commands.yaml` | in/out | Project-owned `verify`, E2E, and coverage commands/paths; applied as a later overlay. |
| `givn/**` artifacts | in/out | Markdown and `.feature` lifecycle files, plus generated semantic reports and failed-review findings documents. |
| `GIVN_FEATURES` | in | Feature path exported to project command processes. |
| `givn instructions --json` | out | One object with `id`, `generates`, `requires`, and resolved `instruction`. |
| `givn instructions arc42-docs` / `design-review` | out | Effective ADR qualification and review guidance; the project cascade may override the default source. |
| `givn spec index/search/explore/review/duplicates` | in/out | Local E5 index, advisory retrieval, surface-specific second-stage evidence (explore, search with `--evidence`), complete permanent audit, and advisory change review with decision-first text, verbose, or structured JSON output. |
| `givn check review --change <id>` | in/out | Mandatory checks and, with retrieval, change-local semantic report generation. |
| `givn upgrade` on a 0.2.0 project | in/out | Clean-worktree preflight, managed scaffold synchronization, two Git commits, and creation of a maintenance migration change. |
| `assets/migrations/catalog.yaml` | in | Release-owned migration bundle policy used to resolve version gaps and aggregate ordered migration phases. |
| `givn/archive/<id>/` | out | Copy/remove archive of the complete change directory, retaining semantic-review evidence. |
| Agent target directories | out | Selected `.opencode`, `.agents`, and `.claude` skill paths; opencode commands use `.opencode/commands`. |
| Durable Arc42 path | out | Fixed `docs/arc42/`; no effective output-dir setting exists. |

## 3.3 Scope

### In scope

- Gherkin-first change lifecycle and delta merge.
- YAML manifest overlay and per-facet cascade.
- Filesystem-derived state and visible skip markers.
- Static lint and explicit project test-runner commands.
- Mandatory task/review checks plus additional post hooks.
- Addon management, including fixed-path durable Arc42 scaffolding and review.
- Agent skill/command generation with selected targets and fenced-block ownership.
- Local E5 retrieval, complete Explore, change-aware Explore, semantic report,
  projected remediation, and permanent duplicate audit.
- Optional BGE/NLI local evidence with actual-tokenizer cap handling.
- Automatic validated model acquisition into the user-level XDG cache, with a project-local permanent index.
- Repository-owned compiled-CLI acceptance fixtures.
- Strict ADR qualification guidance and canonical routing of architecture rationale.
- Version 0.3.0 release metadata and the 0.2.0-to-0.3.0 LLM migration path.

### Out of scope

- Hosted model inference or a remote retrieval service.
- Automatic classification, automatic feature edits, automatic chunking, or
  automatic scenario splitting for token caps.
- A configurable Arc42 output directory or chapter selection cascade.
- OpenSpec backward-compatible reads or migrations from versions other than
  the documented 0.2.0-to-0.3.0 route.
- Multi-repository workflow coordination.
- Release, Homebrew, and shell-installer infrastructure before it is implemented.
- Treating BGE/NLI evidence as a domain classification.
- Downloading model artifacts during installation, nonsemantic commands, or implicit background work.
