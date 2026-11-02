# Proposal: add-javascript-outline

## Release note

Outline analysis now supports production JavaScript, including ES module and CommonJS files, without adding a runtime dependency.

## Use-Case Context

- Use-case ID: `assess-code-structure`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`
- Source: operator request to inspect the production sources in `/tmp/givntwst0916`. Its only source under the default `src/` root is `src/domain/list-store.js`, and the report currently refuses the scope as unsupported.

## Review Context

- Review kind: self-review
- Reason: advisory outline language coverage and a JSON schema version bump; no stored data, security boundary, lifecycle gate, or verdict changes. The report remains deterministic and advisory.

## Problem / Opportunity

The outline report covers production Rust, Python, Java, and TypeScript. JavaScript production code receives no structural outline, so reviewers must assess those components directly from source during REFACTOR and review.

The gap is explicit in the current contract: the use case lists JavaScript as out of scope, the supported-language message omits it, and risk R-114 names JavaScript among the unsupported languages. A project whose production code is JavaScript cannot obtain the advisory call-tree and metric evidence the report exists to provide.

The requesting project demonstrates the concrete cost. Its only production source under `src/` is `src/domain/list-store.js`; the command exits non-zero and names JavaScript as unsupported instead of reporting the functions, class, methods, and calls that are present.

## Proposed Solution

Givn now analyzes production JavaScript in `givn outline report` while keeping the existing Rust, Python, Java, and TypeScript behavior.

- JavaScript is reported as its own language value. Units from `.js`, `.mjs`, and `.cjs` files are labeled `javascript`; they are not relabeled as TypeScript.
- The report covers file modules, named functions, arrow-function bindings, classes, methods, class fields, and calls. A class is a `class` type unit; a file is a module unit.
- JavaScript test and dependency sources are excluded, matching the TypeScript rules: `node_modules`, `dist`, `build`, `.next`, `coverage`, `out`, `__tests__`, and `*.test.js`/`*.spec.js`. The existing global `tests/` exclusion also applies.
- A malformed JavaScript file becomes a per-file parse failure; other files in the scope continue to report.
- JavaScript analysis is offline and in-process. It requires no Node.js, interpreter, or runtime grammar download.
- The report's capability disclosure, supported-language error text, and JSON `capabilities.languages` include `javascript`. The JSON schema version advances from 4 to 5 because the supported language value set changes.
- Calls remain structural. The shared resolver never resolves a call across languages, and an unproven receiver remains name-only.
- JavaScript test and dependency exclusions are consistent with TypeScript. JavaScript type-aware resolution, JSX, and Go remain out of scope and stay disclosed as unsupported.

Example result for the requesting project:

```sh
givn outline report --path .
```

The report names `src/domain/list-store.js` with language `javascript` and includes units such as `cmp`, `maxReg`, `Clock`, `addItem`, and `mergeStates` instead of refusing the scope.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `outline-report` | `EXTEND givn-workflow-enforcement` | `EXTEND outline-report` | The change extends the existing advisory report and its neutral analyzer port; it adds no lifecycle or review capability. |

## Out of Scope

- JSX and Go production sources.
- A JavaScript or TypeScript import/export graph and module-resolution analysis.
- Type-aware or semantic JavaScript resolution; calls remain structural evidence.
- Replacing or rewriting the existing Rust, Python, Java, or TypeScript adapters.
- Cross-language call resolution.
- Partial outlines for malformed files.
- Runtime grammar downloads, user-supplied parsers, or a plugin framework.
- Any outline verdict, waiver, baseline, or review/archive gate.
- CommonJS-specific export modeling; functions are reported from the file regardless of how they are exported.

## Open Questions

### D1 — Which JavaScript file kinds are in scope?

- Question: Which production JavaScript file extensions should the report claim?
- Options:
  1. **`.js` only** — only classic JavaScript files are analyzed; gain: the smallest discovery change; cost: ES module (`.mjs`) and CommonJS (`.cjs`) production files remain unsupported; in practice: the requesting project's `server.mjs` is still skipped.
  2. **`.js`, `.mjs`, and `.cjs`** — all three common production extensions are reported as `javascript`; gain: classic scripts, ES modules, and CommonJS files are covered; cost: three extension cases and their test/spec exclusions; in practice: `--path .` reports both `src/domain/list-store.js` and `server.mjs`.
  3. **Add `.jsx` as well** — JSX files are claimed as JavaScript; gain: React-style component sources; cost: JSX is a distinct dialect with no current request, and the use case keeps it out of scope; in practice: a `.jsx` component becomes a reported unit instead of a disclosed unsupported source.
- Disposition: answered 2026-11-02 (operator) — `.js`, `.mjs`, and `.cjs` are in scope; JSX remains out of scope.

### D2 — Is JavaScript reported as its own language?

- Question: Should JavaScript units carry a `javascript` language value, or reuse the existing `typescript` value?
- Options:
  1. **Separate `javascript` value** — units from all three extensions carry `javascript`; gain: honest labeling, distinct capability disclosure, and no conflation with TypeScript dialect rules; cost: the JSON language value set changes and the schema version must advance; in practice: a `.js` unit reports `"language": "javascript"`.
  2. **Reuse `typescript`** — JavaScript files are reported under the TypeScript language value; gain: no new language value; cost: the report misstates the source language and cannot disclose JavaScript-specific limits separately; in practice: a `.js` unit reports `"language": "typescript"`, which is false.
- Disposition: answered 2026-11-02 (operator) — JavaScript is its own language value.

### D3 — Does the added language value advance the JSON schema?

- Question: Should the JSON report keep schema version 4 or advance to version 5 when `javascript` joins the supported language set?
- Options:
  1. **Advance to version 5** — the schema version changes with the language value set; gain: a consumer can detect the changed capability contract; cost: callers that pin version 4 must update; in practice: `schema_version` is `5` and `capabilities.languages` contains `javascript`.
  2. **Keep version 4** — the version stays fixed; gain: no caller update; cost: the same version describes two different supported-language sets, so a consumer cannot distinguish them; in practice: a consumer that accepts version 4 may silently encounter `javascript`.
- Disposition: answered 2026-11-02 (operator) — advance to version 5, following the precedent that added language values advanced 3 to 4.

### D4 — What happens when a JavaScript file cannot be parsed?

- Question: Should a malformed JavaScript file abort the report, produce a partial unit, or become a per-file failure?
- Options:
  1. **Per-file parse failure** — the file is named with a failure diagnostic and other files continue; gain: one broken file does not hide the rest of the scope; cost: the report is visibly incomplete for that file; in practice: a malformed `src/broken.js` is listed as a failure while `src/ok.js` still reports its units.
  2. **Abort the report** — the command exits non-zero on the first malformed file; gain: no incomplete report is produced; cost: one broken file hides every valid unit in the scope; in practice: a malformed test fixture prevents the report from showing the rest of the project.
  3. **Emit a partial outline** — units are reported from whatever parsed; gain: some evidence remains; cost: the report claims structure the analyzer cannot prove; in practice: a partial class is reported as if complete.
- Disposition: answered 2026-11-02 (operator) — per-file failure with a diagnostic; other files continue, matching Java and TypeScript.
