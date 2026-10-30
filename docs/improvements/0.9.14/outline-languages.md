# Broaden outline language support with Tree-sitter

## Status and purpose

Discussion draft started on 2026-10-30. Givn 0.9.13 is the intended improvement target, not a release commitment.

The user requested an improvement document discussing Tree-sitter as the parser for creating outlines. The confirmed direction is to evaluate Tree-sitter for Rust and Python alongside the current implementations, compare the outline features Givn needs, decide on replacement only after that comparison, and then add additional languages. The additional language set remains open.

This is documentation of future work only. The user explicitly deferred the feasibility assessment and implementation; neither is authorized now. Feature parity has not been established.

This document is not a formal proposal, executable specification, technical design, or authorization to implement. No new Givn change was started. At recording time, `givn instructions` reported the unrelated release-preparation change (`prepare-0-9-10`) as verification-pending and ready to archive; this improvement does not authorize resuming or archiving it.

Before continuing in another context, run `givn instructions`, interpret its state against the current request, and load `givn instructions dialog` plus the applicable companion examples before asking questions. Preserve unrelated working-tree edits. This discussion changes only this document, not the glossary, release metadata, source, or existing specifications.

## Problem and current behavior

The outline addon currently supports production Rust and Python:

- Rust uses the linked `syn` parser.
- Python runs a bundled standard-library `ast` extractor through `python3`; Python-only analysis therefore requires an installed interpreter. Project source is parsed, not imported or executed.
- Both adapters supply a parser-neutral analysis. Shared code resolves calls, calculates metrics, classifies source units, and renders human and JSON reports.
- Other languages require direct source assessment rather than a generated outline.

Sources: `docs/outline.md:206` describes language support and runtime requirements; `src/outline/analyzer.rs:6` defines the analyzer interface; `src/outline/analyzer.rs:16` composes the current adapters. `Cargo.toml:17` lists the current dependencies and contains no Tree-sitter dependency.

Adding an unrelated parser stack for every language would repeat integration work. Tree-sitter offers a common parsing runtime with language-specific grammars, making broader coverage possible without requiring each project's compiler or interpreter to run the report.

However, a grammar is not an outline implementation. Givn still needs language-specific rules for declarations, ownership, signatures, calls, production-source exclusions, and unavailable measurements.

## What Tree-sitter contributes

Tree-sitter creates a concrete syntax tree from source text. Queries can capture declarations and references; tree traversal can establish their enclosing declarations, source ranges, and structural order.

For example, a language adapter could extract a class, its method signatures, and calls inside each method from the same syntax tree. The existing report would then show the class as a source unit, its methods as branches, and evidenced call targets as nested branches or cross-component leaves.

Potential benefits:

- **Broader language coverage through one runtime.** Each supported language adds a reviewed grammar and extraction rules rather than an entirely separate parser integration mechanism.
- **In-process analysis.** Bundled grammars can run without installing the analyzed language's toolchain. Migrating Python could remove its report-time `python3` dependency.
- **Syntax-derived source ranges.** Declaration boundaries can come from the tree instead of textual brace counting or backward searches. Any resulting measurement correction still needs explicit verification.
- **Useful syntax around errors.** Recovery trees can retain intact declarations near malformed code, if Givn first defines how partial results are disclosed.

These are potential gains, not measured performance claims. Incremental parsing is not an immediate benefit for a command that starts fresh for each report. This improvement does not require a daemon, editor integration, or persistent tree cache.

## What Tree-sitter does not contribute

Tree-sitter does not supply compiler-level symbol resolution, receiver type inference, import resolution, macro expansion, or a complete runtime call graph.

For example, parsing `worker.run()` identifies a method call named `run`. It does not prove which project's `run` method the receiver denotes. A unique matching name must not become a confidently resolved target merely because Tree-sitter found the call.

Likewise, a declaration query alone does not establish:

- Which class or module owns a nested declaration or call.
- Whether an assigned arrow function counts as a named function in the outline.
- How overloaded methods receive distinct stable identities.
- Whether an access modifier or language convention means public visibility.
- Which source files are production code rather than tests, generated output, or dependencies.
- Whether cohesion or complexity measurements are meaningful and actually calculated for that language.

Upstream `tags.scm` queries are useful starting points for declarations and references, not a complete implementation of Givn's outline contract. Syntax-highlighting queries are not outline queries.

## Vocabulary and behavior to preserve

Use the existing vocabulary in `docs/arc42/12-glossary.md:215` and `docs/arc42/12-glossary.md:225`:

- **Outline report:** the advisory human or JSON snapshot, not a quality verdict or review gate.
- **Source unit:** the type or module whose structure and measurements the report describes, not an arbitrary syntax-tree node.
- **Outline:** the structured signatures and declared member types rendered as a per-component call tree, not a flat symbol inventory.
- **Call resolution:** the evidence label attached to a call target; uncertainty remains explicit.

New language coverage must preserve deterministic ordering, explicit requested source roots, production-source exclusions, and offline operation. Unavailable measurements remain `unavailable` in text and `null` in JSON, never measured zero. Calls must not resolve across languages merely because names match.

The current advisory behavior is recorded in the code-structure assessment use case for release preparation (`givn/changes/prepare-0-9-10/specs/assess-code-structure/usecase.md:54`). It excludes languages beyond Rust and Python at line 114. A future language expansion must deliberately amend that scope through its own change. Older architecture prose mentioning a review gate is historical, not authority to reintroduce blocking behavior.

## Proposed integration boundary

Keep Tree-sitter behind the existing `SourceAnalyzer` interface. It should supply the same neutral facts consumed by shared resolution, metrics, classification, and rendering. Tree-sitter node types and query captures must not leak into those consumers.

The neutral-analyzer decision (`docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md:115`) already anticipates a later Tree-sitter adapter. It requires equivalent analyses to produce equivalent reports and keeps parser-specific types inside adapters. This is an existing extension seam, not a reason to invent another plugin framework.

A proposed extraction path is:

1. Discover supported production sources within the requested roots using explicit file-extension and dialect rules.
2. Select a bundled grammar and parse source text in-process.
3. Apply language-specific queries and traversal to extract declarations, signatures, ownership, calls, fields, source spans, and unsupported constructs where supported.
4. Convert those facts into the neutral model, retaining uncertainty and measurement availability.
5. Reuse shared call resolution, measurement, classification, and rendering.

A small shared Tree-sitter module with explicit language mappings is sufficient initially. Do not add runtime grammar downloads, user-supplied executable parsers, dynamic plugin discovery, or grammar generation during report execution.

## Language coverage must be explicit

Do not advertise support for every language with a published Tree-sitter grammar. The supported set is the set with tested Givn mappings, production-scope rules, and honest limitations.

Illustrative candidates, not an approved delivery list:

| Candidate | Useful outline coverage | Mapping questions that a grammar does not settle |
|---|---|---|
| JavaScript and TypeScript | Modules, classes, methods, named functions, interfaces where applicable | JS/JSX and TS/TSX dialect selection; assigned arrow functions; exported declarations; runtime versus type-only declarations |
| Go | Packages/files, named types, functions, receiver methods | Package versus file ownership; attaching receiver methods across files; interfaces; generated and test-source exclusions |
| Java | Classes, interfaces, constructors, methods | Nested and anonymous types; overload identities; packages; visibility and source-set exclusions |
| Rust and Python migration | Preserve existing coverage using the common runtime | Cross-file Rust impls, trait defaults, attributes and macros; Python decorators, async definitions, ownership, and removal of interpreter-failure behavior |

The current model supports type and module source units, with Rust-influenced visibility categories and only type, method, and trait-method outline entries (`src/outline/model.rs:257`, `src/outline/model.rs:269`, `src/outline/model.rs:377`). A construct that does not fit must not be silently relabeled. Either document a faithful mapping, disclose that it is unsupported, or propose a neutral-contract extension.

For every selected language, define the supported declarations, signature formatting, ownership and identity rules, call evidence, source exclusions, unavailable measurements, and unsupported syntax. Capability disclosure must distinguish languages; a merged list must not imply that every measurement or construct is supported everywhere.

## Confirmed evaluation-first delivery order

Resolution: evaluate Rust and Python first, decide replacement from the comparison, then expand language coverage. Recorded from the user's direction on 2026-10-30. This establishes the future sequence, not authorization to perform it in this session.

1. **Build parallel Rust and Python Tree-sitter adapters for comparison.** Keep the current `syn` and Python `ast` implementations as the reference paths; do not switch defaults or remove either implementation just to begin evaluation.
2. **Compare the outlines Givn needs.** Run the existing and candidate adapters over the same representative sources and compare extracted facts, human reports, JSON reports, diagnostics, and measurement availability. Determine whether Tree-sitter achieves the required feature parity, not merely whether it can parse both languages.
3. **Decide whether to replace the Rust and Python implementations.** Record evidence and a replacement decision for each language after comparison. Building an evaluation adapter does not pre-approve replacing the current adapter. Retaining one or both current implementations remains a valid outcome if required features are missing or migration costs outweigh the gains.
4. **Add additional languages afterwards.** Select languages and dialects separately once the Rust/Python comparison and replacement decisions are recorded. Broader coverage must not conceal unresolved regressions in current support.

Gain: existing Rust/Python behavior provides a concrete reference, so adopting a common runtime is an evidence-based decision rather than an assumption. Cost: parallel adapters and a representative differential corpus require work before additional languages gain outlines. For example, TypeScript remains unsupported during the Rust/Python evaluation; a successful Python comparison may later justify removing `python3`, but no such change is decided now.

Rejected:

- **Additional languages first:** would deliver broader coverage sooner, but the user chose to establish Rust/Python parity and settle replacement before expansion.
- **Immediate replacement of Rust and Python:** would establish one runtime sooner, but assumes the answer to the parity question and removes the reference implementations before a supported decision.
- **Replacement and new-language expansion together:** combines existing-behavior regression risk with new-language work, contrary to the confirmed order.
- **Treating evaluation as a commitment to migrate:** compilation or successful parsing is not evidence that the required outline features are preserved. Replacement remains a later decision.

Do not retain old adapters as silent runtime fallbacks if replacement is selected. Keep them as explicit reference implementations during evaluation; remove an obsolete path only after its replacement is approved and verified. If replacement is rejected, the retained implementation is the selected implementation, not a hidden fallback.

## Required feature-parity comparison

The future evaluation must answer: **does Tree-sitter preserve the outline Givn needs for Rust and Python?** No answer is asserted in this document. Grammar availability, syntax feasibility, implemented extraction, and verified report parity are separate levels of evidence.

Record a comparison matrix for each language covering:

- Supported declarations and source-unit ownership, including Rust impl aggregation across files and Python class/module ownership.
- Signature fidelity, visibility, declaration order, stable identities, and source spans.
- Calls and their structural order, receiver/qualifier evidence, uncertainty labels, same-unit nesting, and cross-component leaves.
- Fields, relations, flow facts, measurement availability, and classifications consumed by the existing report.
- Production/test exclusions, unsupported constructs, parse failures, and continued reporting for other files.
- Deterministic human and JSON output and the runtime/distribution requirements of each implementation.

Use representative fixtures and real project sources, with expected outputs checked against the required behavior rather than treating every current-parser result as correct. Classify each difference as equivalent behavior, a missing required feature, an intentional correction or improvement, an accepted limitation, or an operational change. Record evidence and reasons; do not hide differences by broadly normalizing output or copying existing bugs solely to claim parity.

Parser replacement, richer extraction, and metric corrections remain distinct decisions. A grammar may expose more syntax without requiring Givn to change the report contract. Conversely, removing Python's interpreter requirement is an operational benefit, not proof that its signatures and calls match.

The resulting evidence must support a separate replace-or-retain decision for Rust and Python, with unresolved gaps and migration costs visible before approval.

## Error recovery and honest partial analysis

A returned syntax tree does not mean the file parsed cleanly. Tree-sitter represents errors using `ERROR` nodes and inserted missing tokens; both must be checked.

The conservative starting recommendation is to keep file-level parse-failure behavior: diagnose the malformed file, omit its units, and continue analyzing other files. This preserves the current completeness boundary during the Rust/Python comparison.

A later partial-outline mode could retain unaffected declarations, but that is an explicit behavior and model change, not a free consequence of adopting Tree-sitter. It must identify incomplete units and prevent omitted calls or fields from producing apparently complete measurements and classifications.

For example, an intact class before a broken method might still provide useful signatures, but missing method bodies must not make the class appear to have zero complexity or fully evidenced cohesion. The current file status distinguishes only analyzed and failed, while source units have a completeness flag (`src/outline/model.rs:166`, `src/outline/model.rs:298`); inspect all consumers before promising partial reports.

## Distribution, maintenance, and compatibility costs

- Bundle selected generated grammars and pin compatible versions of the Rust binding, runtime, and grammar crates through the dependency policy and lockfile. Verify their language ABI compatibility; do not assume arbitrary versions work together.
- Check each grammar's license, attribution requirements, maintenance activity, supported syntax versions, and external scanner implementation. A shared runtime does not make the grammar supply chain uniform.
- Account for the C build toolchain and any external scanner requirements on supported build targets. End users of a prebuilt binary should not need Node, the Tree-sitter CLI, a compiler, or a language interpreter just to request an outline.
- Measure release binary size, build time, report latency, and memory on a fixed corpus before and after. More grammars have costs; no performance or size improvement is claimed here.
- Never execute or import project code, run project build scripts, or fetch grammars from the network during analysis. Test malformed and deeply nested input; define bounded handling for parser/query failures rather than letting one file abort the entire report.
- Grammar upgrades can change tree shapes and query matches. Keep extraction fixtures alongside each supported language and review changes to neutral facts, not merely whether parsing succeeds.

The current JSON report uses schema version 3 and rules version 1 (`src/outline/report.rs:11`). An internal parser substitution with equivalent facts should not force a schema change. A neutral-model extension, new public language values, changed diagnostics, or corrected measurement semantics needs an explicit compatibility decision under the existing versioning contract.

Removing the Python subprocess also changes observable behavior: missing `python3` would no longer produce `analyzer-unavailable`. Update that requirement deliberately rather than preserving an obsolete diagnostic through a fake dependency check.

## Verification expectations

Before implementation is considered complete:

- Test each selected language with representative declarations, visibility/export forms, async/generic constructs where applicable, nested ownership, duplicate names, and dialect-specific files.
- Verify signature fidelity, stable source ordering, source ranges, and identities. Include comments and strings containing braces, non-ASCII text, and multiline declarations.
- Verify calls separately from declarations: same-unit calls, cross-component calls, repeated calls, unknown receivers, ambiguous names, and dynamic callees. A name-only match must remain a leaf, not a confidently nested call.
- Cover test, dependency, generated, and tooling-source exclusions for each supported layout. Mixed-language reports must disclose unsupported or failed coverage without implying all production code was analyzed.
- Exercise malformed files with both error nodes and missing tokens, unsupported constructs, unreadable files, and unavailable measurements. Other files must continue to report.
- Compare actual existing and Tree-sitter adapters over a representative Rust/Python corpus before replacing either. Compare neutral facts and both report formats; classify every intentional difference. A fake-adapter equivalence test alone is not migration evidence.
- Verify Python analysis without an installed interpreter if Python migration is selected, and verify Rust-only/new-language reports do not start it unnecessarily.
- Verify repeated runs produce deterministic human and JSON output, without network access, runtime grammar generation, project-code execution, or persistent caches.
- Preserve advisory behavior: no outline verdict, waiver, baseline, review blocker, or archive blocker is introduced.

Existing entry points include `givn/specs/assess-code-structure/outline-report.feature`, `tests/steps/outline_report_support.rs`, and adapter-local tests under `src/outline/`. Recheck their current locations and governing specifications when starting the future change.

## Open decisions

- **Replacement after evaluation:** whether to replace or retain each existing Rust/Python implementation. This decision waits for the required feature-parity comparison; the evaluation itself is deferred.
- **Additional language scope:** which languages and dialects to add after the comparison and replacement decisions. The candidate table is illustrative, not a release promise.
- **Error-recovery behavior:** whether the current file-level parse-failure boundary is sufficient or useful output from malformed files warrants a separate completeness-contract change. Conservative behavior above remains a recommendation, not a confirmed requirement.

The evaluation-first sequence is settled and must not be re-asked without new evidence. Exact dependency versions, query organization, extraction fixtures, capability representation, and any necessary neutral-model changes belong to the future design. None of this work is authorized now.

## Upstream references

- [Tree-sitter introduction](https://tree-sitter.github.io/tree-sitter/): concrete syntax trees, incremental parsing, error tolerance, and language bindings.
- [Code navigation](https://tree-sitter.github.io/tree-sitter/4-code-navigation.html): declaration/reference captures and language-specific tags queries.
- [Query syntax](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html): query patterns and the distinction between `ERROR` and `MISSING`.
- [Implementation](https://tree-sitter.github.io/tree-sitter/5-implementation.html): embedded C runtime versus the parser-generation CLI.
