# Givn Design System

The durable style source for Givn's own browser screens. A change that
introduces or visibly changes a screen records a `## Visual Design Contract` in
its `design.md` that references this document; a conforming improvement updates
this document in the same change.

The first screen is the development report (`givn timeline`), which renders one
change or ideation as an interactive, zoomable timeline. This document defines
the tokens, primitives, and reference views that screen uses, and that any later
Givn browser surface inherits.

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
| `--color-band-command` | `#6b7bd6` | Lifecycle command spans. |
| `--color-band-step` | `#2f9e8f` | Gate phases, hook runs, verification scopes. |
| `--color-band-agent` | `#b06ad0` | Agent turns, tool calls, subagent spans. |
| `--color-band-wait` | `#d08a2f` | Question waits. |

The four band hues stay distinguishable in greyscale (they differ in lightness,
not only in hue), so a printed or monochrome report still separates the bands.

Spacing scale: `4 / 8 / 12 / 16 / 24 / 32` px. Every margin and padding is one
of these values.

Typography scale: `12 / 14 / 16 / 20 / 28` px, one family (the system UI stack:
`system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`), `1.45` line height.
Every duration and token count uses tabular numerals
(`font-variant-numeric: tabular-nums`) so columns and axis labels align.

Radii: `4` px for timeline spans and controls, `8` px for panels and tiles.

Elevation: exactly one level — the details panel raises with
`0 1px 2px rgba(20, 23, 28, .08), 0 8px 24px rgba(20, 23, 28, .12)`. Nothing
else floats.

## Component primitives

| Primitive | States | Rules |
|---|---|---|
| Timeline span | default, hover, focus, selected, label-omitted | One bar on the time axis; the accessible name is always `<name>, <duration>`; a span narrower than its label omits the inline label and keeps the accessible name. |
| Band row | default, collapsed, "outside a scope" | One row per band; the row label is visible text, never an internal kind name. |
| Zoom control | default, hover, focus, disabled at the zoom limit | A visible text label (`Zoom in` / `Zoom out` / `Fit`), never an icon alone. |
| Details panel | closed, open, empty | A labelled region; shows start, end, duration, outcome, model, tokens, and the bounded label; carries a labelled close control. |
| Summary tile | default, zero value | One number per tile with a written label; a zero value reads `0` and renders the tile, never a blank. |
| Empty state | empty, error | A heading, one sentence explaining the state, and a way back to the report root; no chart axes are drawn. |
| Record-gap notice | default, dismissed | Shown when the reporter dropped events or the record could not be written; names the count and the reason. |

Every primitive has a visible label requirement: no control is icon-only, and no
internal identifier or state name appears in visible text.

## Breakpoints

| Viewport | Layout |
|---|---|
| below 720 px (mobile, 360x780 contract viewport) | One column: header, summary tiles stacked, band rows with a horizontally scrollable time axis, details panel as a full-width sheet below the axis. |
| 720-1199 px | Two columns: summary beside the timeline; the details panel is an inline region under the timeline. |
| 1200 px and above (desktop, 1920x1080 contract viewport) | Two columns with a wider axis gutter; the details panel is a right-hand side panel. |

## Accessibility

- Contrast: body text against its surface is at least `4.5:1`
  (`--color-text` on `--color-surface` is 15.0:1, `--color-text-muted` is
  4.8:1); non-text UI (span outlines, control borders, focus ring) is at least
  `3:1`.
- Keyboard: every interactive element is reachable by `Tab` in document order,
  has a visible focus ring (`2 px` `--color-accent` with a `1 px` offset), and
  the details panel opens on `Enter`/`Space` and closes on `Escape`.
- Semantics: one `h1`, `header`/`main`/`aside` landmarks, the timeline `svg`
  carries `role="img"` with an accessible name, and each span is exposed with
  its accessible name rather than as undifferentiated graphics.
- Language: the document declares the configured language; all visible copy is
  English in this project.
- Verification: the change that owns a screen asserts these in its browser
  tier — control labels, heading, landmark presence, focus order, and the
  reduced-motion behaviour — rather than leaving them to review prose.

## Motion

Motion is limited to the zoom transition (`120 ms ease-out`) and the details
panel entrance (`120 ms ease-out`). Under
`prefers-reduced-motion: reduce` both are removed entirely: the zoom applies
instantly and the panel appears without transition. Screenshots are captured
with animations disabled, so the captured state never depends on timing.

## Screens covered

| Screen | Owning change | States |
|---|---|---|
| Development report (`givn timeline`), for a change, an ideation topic, or the project | `development-timeline` | empty (declared and captured as `<viewport>--empty.png`); a populated report is the implicit final state of every other `@e2e` scenario |

## Rendered reference views

Three concrete directions were prepared so the operator could choose one
(`Style tile`: a rendered, concrete design option proposed when no approved
design exists). Each view is a rendered reference, not a pixel specification.
The operator approved the **dense timeline** (view 1) on 2026-10-16; views 2
and 3 remain here as reference alternatives, and `design.md` records the
approved direction in its Visual Design Contract and in its `Decides D6`
ledger entry.

### View 1 — Dense timeline (approved)

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
