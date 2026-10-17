# Givn Design System

The durable style source for Givn's own browser screens. A change that
introduces or visibly changes a screen records a `## Visual Design Contract` in
its `design.md` that references this document; a conforming improvement updates
this document in the same change.

The first screen is the development report (`givn timeline`), which renders one
change or ideation as a top-to-bottom workflow: overall statistics, the phase
stack, an expandable phase's vertical lane timeline with its lane log, and the
detailed statistics. This document defines the tokens, primitives, and reference
views that screen uses, and that any later Givn browser surface inherits.

## Design tokens

Color roles. Every role is a semantic name first, a value second.

| Token | Value | Role |
|---|---|---|
| `--color-surface` | `#ffffff` | Page background. |
| `--color-surface-raised` | `#f5f6f8` | Panels, summary tiles, band gutter. |
| `--color-border` | `#d5d9e0` | Dividers, span outlines, tile edges. |
| `--color-text` | `#14171c` | Body and heading text. |
| `--color-text-muted` | `#5a6270` | Secondary text, axis labels, meta rows. |
| `--color-accent` | `#2f6fd0` | Focus ring, selected span outline, links. |
| `--color-ok` | `#1f7a4d` | Passed steps and scopes. |
| `--color-warn` | `#9a6100` | Recorded gaps and rejected reports. |
| `--color-error` | `#b3261e` | Failed steps and the error state. |
| `--color-lane-main` | `#2f6fd0` | The main agent lane. |
| `--color-lane-a` | `#2f9e8f` | First recorded session lane. |
| `--color-lane-b` | `#b06ad0` | Second recorded session lane. |
| `--color-lane-c` | `#d08a2f` | Third recorded session lane. |

The lane hues stay distinguishable in greyscale (they differ in lightness, not
only in hue), so a printed or monochrome report still separates lanes; lanes
beyond the four defined hues cycle them, and the lane name — never the hue —
carries identity. Phase
status uses `ok` (done), `warn` (active), `text-muted` (queued), and
`surface-raised` with a dashed border (skipped).

Spacing scale: `4 / 8 / 12 / 16 / 24 / 32` px. Every margin and padding is one
of these values.

Typography scale: `12 / 14 / 16 / 20 / 28` px, one family (the system UI stack:
`system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`), `1.45` line height.
Every duration and token count uses tabular numerals
(`font-variant-numeric: tabular-nums`) so columns and axis labels align.

Radii: `4` px for lane ticks and controls, `8` px for the lane log and summary
tiles, `12` px for phase cards.

Elevation: exactly one level — the lane log and the active phase card raise with
`0 1px 2px rgba(20, 23, 28, .08), 0 8px 24px rgba(20, 23, 28, .12)`. Nothing
else floats.

## Component primitives

| Primitive | States | Rules |
|---|---|---|
| Phase card | collapsed, expanded, active, done, queued, skipped, keyboard focus | One card per workflow phase, top to bottom; the header is an expander button with `aria-expanded`; the accessible name is `<phase>, <status>, <duration>`; the card shows wall/active/waiting, tools, subagents, and a decorative activity sparkline. |
| Ideation group | present, no recorded activity | A labeled second phase stack above the change workflow when the proposal or seed links an existing ideation topic; it uses the same phase-card, lane, and log primitives with the ideation phase names and its own totals; a linked topic with no entries states that instead of rendering phases. |
| Phase lane | default, hover, focus, selected, no-attributable-calls | One lane per recorded session inside an expanded phase; a lane is named by a subagent span only when the span's `data.subagent` equals the lane's session id, otherwise by the recorded session title, else by the session's short id; a lane that merely contains a subagent span keeps its own name; the lane header (not the lane column) is the button, with accessible name `<lane>, <duration>`; a lane with no attributable entries states that in its log instead of inventing them. |
| Lane tick | default, failed, repeated, aggregate, focus | One tick per entry when the current zoom's time-per-pixel permits; a failing entry uses `error`, a repeated-step member uses `warn`; an aggregate tick names how many entries and their total duration and is applied at every zoom, so a dense 15 m window still renders a bounded number of ticks; the accessible name carries kind, label, and duration. |
| Phase sparkline | decorative | Activity density across the phase; `aria-hidden`, never the only carrier of a fact. |
| Zoom control | default, hover, focus, disabled at the zoom limit | Per expanded phase; visible text labels `Fit` / `1 h` / `15 m`, never an icon alone. |
| Lane log | open, empty | A labelled region below the phase axis; lists the selected lane's entries with time, kind, label, duration, and outcome, and renders bounded content when captured; an empty lane names why. |
| Summary tile | default, zero value | One number per tile with a written label; a zero value reads `0` and renders the tile, never a blank. |
| Empty state | empty, error | A heading, one sentence explaining the state, and a way back to the report root; no workflow phase is drawn. |
| Record-gap notice | default, dismissed | Shown when the reporter dropped events or the record could not be written; names the count and the reason. |

Every primitive has a visible label requirement: no control is icon-only, and no
internal identifier or state name appears in visible text.

## Breakpoints

| Viewport | Layout |
|---|---|
| below 720 px (mobile, 360x780 contract viewport) | One column: header, current-state line, two-column KPI strip, full-width phase cards; an expanded phase scrolls its lanes horizontally inside the card with the lane log below the axis; statistics and tables stack and scroll inside their cards. |
| 720-1199 px | One column with a two-column statistics grid; phase cards full width; the lane log below the axis. |
| 1200 px and above (desktop, 1920x1080 contract viewport) | One column with a three-column statistics grid; phase cards full width; the lane log below the axis with a wider lane gutter. |

## Accessibility

- Contrast: body text against its surface is at least `4.5:1`
  (`--color-text` on `--color-surface` is 15.0:1, `--color-text-muted` is
  4.8:1); non-text UI (span outlines, control borders, focus ring) is at least
  `3:1`.
- Keyboard: every interactive element is reachable by `Tab` in document order
  and has a visible focus ring (`2 px` `--color-accent` with a `1 px` offset);
  a phase expands/collapses on `Enter`/`Space`; a lane selects on
  `Enter`/`Space`; the expanded phase can be collapsed with `Escape`.
- Semantics: one `h1`, `header`/`main` landmarks, the phase expanders carry
  `aria-expanded`, each expanded phase and the lane log are labelled regions,
  the lane header (not the lane column) is a button and ticks are separate
  focusable elements, and every lane and tick is exposed with its accessible
  name rather than as undifferentiated graphics.
- Language: the document declares the configured language; all visible copy is
  English in this project.
- Verification: the change that owns a screen asserts these in its browser
  tier — control labels, heading, landmark presence, focus order, the
  reduced-motion behaviour, and the mobile one-column layout without page-level
  horizontal overflow — rather than leaving them to review prose.

## Motion

Motion is limited to the phase-expansion transition, the lane selection, and
the per-phase zoom transition (all `120 ms ease-out`). Under
`prefers-reduced-motion: reduce` all three are removed entirely: expansion, lane
selection, and zoom apply instantly. Screenshots are captured with animations
disabled, so the captured state never depends on timing.

## Screens covered

| Screen | Owning change | States |
|---|---|---|
| Development report (`givn timeline`), for a change, an ideation topic, or the project | `development-timeline` | empty (declared and captured as `<viewport>--empty.png`); a populated report is the implicit final state of every other `@e2e` scenario |

## Rendered reference views

Five concrete directions were prepared so the operator could choose one
(`Style tile`: a rendered, concrete design option proposed when no approved
design exists). Each view is a rendered reference, not a pixel specification;
the views are illustrative, and the authoritative phase list is the nine
workflow phases in `design.md` (the view-4 drawing shows a subset at a small
canvas size). The operator approved the **hybrid workflow** (view 4) on
2026-10-17; the dense timeline (view 1), phase lanes (view 2), and dashboard
(view 3) remain here as reference alternatives, and `design.md` records the
approved direction in its Visual Design Contract and in its `Decides D6`
ledger entry.

### View 1 — Dense timeline (reference alternative)

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 360 220" role="img" aria-label="Reference view: dense timeline" font-family="system-ui, sans-serif">
  <rect width="360" height="220" fill="#ffffff"/>
  <text x="12" y="22" font-size="14" fill="#14171c">Development report — demo change</text>
  <text x="12" y="38" font-size="10" fill="#5a6270">09:12 → 14:48 · longest step: archive verification (41 m)</text>
  <rect x="12" y="52" width="336" height="26" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="69" font-size="9" fill="#5a6270">commands</text>
  <rect x="110" y="57" width="40" height="16" fill="#6b7bd6"/>
  <rect x="180" y="57" width="70" height="16" fill="#6b7bd6"/>
  <rect x="270" y="57" width="60" height="16" fill="#6b7bd6"/>
  <rect x="12" y="82" width="336" height="34" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="99" font-size="9" fill="#5a6270">steps</text>
  <rect x="150" y="87" width="120" height="10" fill="#2f9e8f"/>
  <rect x="150" y="99" width="120" height="10" fill="#2f9e8f" opacity="0.55"/>
  <rect x="12" y="120" width="336" height="44" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="137" font-size="9" fill="#5a6270">agent</text>
  <rect x="110" y="126" width="30" height="8" fill="#b06ad0"/>
  <rect x="145" y="126" width="22" height="8" fill="#b06ad0"/>
  <rect x="172" y="126" width="16" height="8" fill="#b06ad0"/>
  <rect x="193" y="126" width="14" height="8" fill="#b06ad0"/>
  <rect x="212" y="126" width="12" height="8" fill="#b06ad0"/>
  <rect x="178" y="140" width="60" height="8" fill="#b06ad0" opacity="0.6"/>
  <text x="240" y="147" font-size="8" fill="#5a6270">subagent: scout</text>
  <rect x="12" y="168" width="336" height="24" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="183" font-size="9" fill="#5a6270">waits</text>
  <rect x="230" y="173" width="96" height="14" fill="#d08a2f"/>
  <text x="238" y="184" font-size="8" fill="#ffffff">question: 8 m</text>
  <rect x="12" y="200" width="80" height="14" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="211" font-size="9" fill="#14171c">Zoom in</text>
  <rect x="96" y="200" width="80" height="14" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="102" y="211" font-size="9" fill="#14171c">Zoom out</text>
  <rect x="180" y="200" width="50" height="14" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="190" y="211" font-size="9" fill="#14171c">Fit</text>
</svg>
```

### View 2 — Phase lanes

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 360 220" role="img" aria-label="Reference view: phase lanes" font-family="system-ui, sans-serif">
  <rect width="360" height="220" fill="#ffffff"/>
  <text x="12" y="22" font-size="14" fill="#14171c">Development report — demo change</text>
  <text x="12" y="38" font-size="10" fill="#5a6270">one lane per workflow phase</text>
  <rect x="12" y="52" width="336" height="20" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="66" font-size="10" fill="#14171c">Propose</text>
  <text x="280" y="66" font-size="10" fill="#5a6270">12 m</text>
  <rect x="12" y="76" width="336" height="20" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="90" font-size="10" fill="#14171c">Specs + design</text>
  <text x="280" y="90" font-size="10" fill="#5a6270">1 h 05 m</text>
  <rect x="12" y="100" width="336" height="20" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="114" font-size="10" fill="#14171c">Tasks</text>
  <text x="280" y="114" font-size="10" fill="#5a6270">22 m</text>
  <rect x="12" y="124" width="336" height="40" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="138" font-size="10" fill="#14171c">Implement</text>
  <text x="280" y="138" font-size="10" fill="#5a6270">3 h 10 m</text>
  <rect x="150" y="146" width="60" height="8" fill="#b06ad0"/>
  <rect x="214" y="146" width="60" height="8" fill="#b06ad0"/>
  <text x="150" y="160" font-size="8" fill="#5a6270">19 tool calls · 6 subagents</text>
  <rect x="12" y="168" width="336" height="20" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="182" font-size="10" fill="#14171c">Archive</text>
  <text x="280" y="182" font-size="10" fill="#5a6270">41 m</text>
  <rect x="12" y="192" width="336" height="20" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="16" y="206" font-size="10" fill="#14171c">Waits</text>
  <text x="280" y="206" font-size="10" fill="#5a6270">8 m</text>
</svg>
```

### View 3 — Dashboard

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 360 220" role="img" aria-label="Reference view: dashboard" font-family="system-ui, sans-serif">
  <rect width="360" height="220" fill="#ffffff"/>
  <text x="12" y="22" font-size="14" fill="#14171c">Development report — demo change</text>
  <rect x="12" y="34" width="106" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="20" y="52" font-size="9" fill="#5a6270">Elapsed</text>
  <text x="20" y="70" font-size="16" fill="#14171c">5 h 36 m</text>
  <rect x="126" y="34" width="106" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="134" y="52" font-size="9" fill="#5a6270">Longest step</text>
  <text x="134" y="70" font-size="16" fill="#14171c">41 m</text>
  <rect x="240" y="34" width="108" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="248" y="52" font-size="9" fill="#5a6270">Waiting</text>
  <text x="248" y="70" font-size="16" fill="#14171c">8 m</text>
  <rect x="12" y="90" width="106" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="20" y="108" font-size="9" fill="#5a6270">Subagents</text>
  <text x="20" y="126" font-size="16" fill="#14171c">6 · 18 m</text>
  <rect x="126" y="90" width="106" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="134" y="108" font-size="9" fill="#5a6270">Tokens</text>
  <text x="134" y="126" font-size="16" fill="#14171c">218 k</text>
  <rect x="240" y="90" width="108" height="46" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="248" y="108" font-size="9" fill="#5a6270">Questions</text>
  <text x="248" y="126" font-size="16" fill="#14171c">2</text>
  <rect x="12" y="150" width="336" height="28" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="20" y="168" font-size="10" fill="#14171c">Show the timeline</text>
  <rect x="12" y="186" width="336" height="22" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="20" y="201" font-size="9" fill="#5a6270">Sorted by duration: archive verification, coverage merge, …</text>
</svg>
```

### View 4 — Hybrid workflow (approved, 2026-10-17)

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 360 220" role="img" aria-label="Reference view: hybrid workflow report" font-family="system-ui, sans-serif">
  <rect width="360" height="220" fill="#ffffff"/>
  <text x="12" y="20" font-size="13" fill="#14171c">Development report — demo change</text>
  <text x="12" y="32" font-size="8.5" fill="#9a6100">now: review active · 38 m · next: sign-off</text>
  <rect x="12" y="38" width="52" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="49" font-size="7" fill="#5a6270">elapsed</text>
  <rect x="68" y="38" width="52" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="74" y="49" font-size="7" fill="#5a6270">active</text>
  <rect x="124" y="38" width="52" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="130" y="49" font-size="7" fill="#5a6270">waiting</text>
  <rect x="180" y="38" width="52" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="186" y="49" font-size="7" fill="#5a6270">tools</text>
  <rect x="236" y="38" width="52" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="242" y="49" font-size="7" fill="#5a6270">spans</text>
  <rect x="292" y="38" width="56" height="16" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="298" y="49" font-size="7" fill="#5a6270">cost</text>

  <rect x="12" y="60" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="67.5" font-size="7.5" fill="#14171c">Propose · done · 42 m</text>
  <rect x="12" y="72" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="79.5" font-size="7.5" fill="#14171c">Specify · done · 26 m</text>
  <rect x="12" y="84" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="91.5" font-size="7.5" fill="#14171c">Design · done · 1 h 38 m</text>
  <rect x="12" y="96" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="103.5" font-size="7.5" fill="#14171c">Design review · done · 2 h 12 m</text>
  <rect x="12" y="108" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0"/>
  <text x="18" y="115.5" font-size="7.5" fill="#14171c">Tasks · done · 1 h 05 m</text>

  <rect x="12" y="120" width="336" height="56" fill="#f5f6f8" stroke="#2f6fd0"/>
  <text x="18" y="130" font-size="8" fill="#14171c">Implement · done · 4 h 46 m</text>
  <text x="18" y="140" font-size="6.5" fill="#5a6270">time ↓</text>
  <rect x="40" y="134" width="34" height="36" fill="none" stroke="#d5d9e0"/>
  <rect x="78" y="134" width="34" height="36" fill="none" stroke="#d5d9e0"/>
  <rect x="116" y="134" width="34" height="36" fill="none" stroke="#d5d9e0"/>
  <rect x="154" y="134" width="34" height="36" fill="none" stroke="#d5d9e0"/>
  <text x="41" y="141" font-size="5.5" fill="#5a6270">scout</text>
  <text x="79" y="141" font-size="5.5" fill="#5a6270">core</text>
  <text x="117" y="141" font-size="5.5" fill="#5a6270">report</text>
  <text x="155" y="141" font-size="5.5" fill="#5a6270">tests</text>
  <rect x="52" y="145" width="4" height="18" fill="#2f9e8f" opacity="0.6"/>
  <rect x="90" y="145" width="4" height="22" fill="#b06ad0" opacity="0.6"/>
  <rect x="128" y="145" width="4" height="20" fill="#d08a2f" opacity="0.6"/>
  <rect x="166" y="145" width="4" height="16" fill="#2f6fd0" opacity="0.6"/>
  <rect x="196" y="130" width="144" height="40" fill="#ffffff" stroke="#d5d9e0"/>
  <text x="202" y="139" font-size="6.5" fill="#14171c">Architect.Core — log</text>
  <text x="202" y="147" font-size="6" fill="#5a6270">07:16 edit src/cli/history.rs · 19 s</text>
  <text x="202" y="154" font-size="6" fill="#5a6270">07:21 commit · test(development-record)</text>
  <text x="202" y="161" font-size="6" fill="#b06ad0">07:22 read src/history.rs</text>
  <text x="202" y="167" font-size="6" fill="#5a6270">07:24 cargo test · 3 m · fail</text>

  <rect x="12" y="178" width="336" height="10" fill="#fffbeb" stroke="#d08a2f"/>
  <text x="18" y="185.5" font-size="7.5" fill="#14171c">Review · active · 38 m</text>
  <rect x="12" y="190" width="336" height="10" fill="#f5f6f8" stroke="#d5d9e0" stroke-dasharray="3 2"/>
  <text x="18" y="197.5" font-size="7.5" fill="#5a6270">Archive · queued</text>
  <rect x="12" y="204" width="336" height="10" fill="#ffffff" stroke="#d5d9e0"/>
  <text x="18" y="211.5" font-size="7" fill="#5a6270">longest: Architect.Core 2 h 51 m · waits: 1 h 18 m · loops: 4</text>
</svg>
```
