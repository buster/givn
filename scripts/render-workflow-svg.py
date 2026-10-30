#!/usr/bin/env python3
"""Generate docs/workflow.svg — GitHub-safe rendering of the givn lifecycle.

Layout mirrors docs/workflow.html: three columns (action/step, artifact,
check). The flow starts at `givn-explore`, which routes either through the
optional `givn-ideation` pre-phase or directly to `givn-propose`, then runs
proposal -> specs -> design -> arc42-docs (addon) -> design-review -> tasks ->
implementation -> review -> archive.

No <style>, no classes, no foreignObject, no filters, no external resources;
solid light background for theme independence.

Run from repo root:  python3 scripts/render-workflow-svg.py
"""

W = 1060
MARGIN = 20
STEP_W = 200
STEP_H = 70
VGAP = 36
ART_X = MARGIN + STEP_W + 36          # horizontal arrow lives in the gap
ART_W = 300
CHK_X = ART_X + ART_W + 28
CHK_W = W - MARGIN - CHK_X
SPINE_X = MARGIN + STEP_W / 2

FONT = "'Segoe UI',-apple-system,Roboto,Helvetica,Arial,sans-serif"
MONO = "ui-monospace,'Cascadia Code',Menlo,Consolas,monospace"

# ---- type scale -----------------------------------------------------------
FS_TITLE = 26
FS_SUB = 14
FS_LEGEND = 13
FS_NUM = 13
FS_STEP = 16
FS_STEP_SUB = 11
FS_ART = 15
FS_ART_LINE = 13
FS_CHK = 15
FS_CHK_LINE = 13
FS_LOOP = 12.5
FS_TAG = 10
FS_PANEL = 13

LH_TITLE = 18
LH_BODY = 18
PAD = 12
TAG_H = 17
CHECK_GAP = 10
CHAR = 0.56

C_BG = "#f1f5f9"
C_CARD = "#ffffff"
C_STEP = "#2563eb"
C_LINE = "#cbd5e1"
C_ART_STROKE = "#94a3b8"
C_ART_NAME = "#334155"
C_MUTED = "#64748b"
C_INK = "#0f172a"
C_CHK_BG = "#fef9c3"
C_CHK_STROKE = "#ca8a04"
C_CHK_INK = "#713f12"
C_ADV_BG = "#fffbeb"
C_ADV_STROKE = "#d97706"
C_LOOP = "#dc2626"
C_TAG_BG = "#b91c1c"
C_TAG_ADV = "#d97706"


class Card:
    def __init__(self, title, lines, blocking=True, loop=None):
        self.title = title
        self.lines = lines
        self.blocking = blocking
        self.loop = loop

    @property
    def height(self):
        h = PAD * 2 + LH_TITLE + len(self.lines) * LH_BODY
        if self.loop:
            h += 19
        return h


def text_width(text, size):
    return len(text) * CHAR * size


def render_card(card, x, y, w):
    parts = []
    dash = "" if card.blocking else ' stroke-dasharray="4 3"'
    bg = C_CHK_BG if card.blocking else C_ADV_BG
    stroke = C_CHK_STROKE if card.blocking else C_ADV_STROKE
    parts.append(
        f'<rect x="{x}" y="{y}" width="{w}" height="{card.height}" rx="10" '
        f'fill="{bg}" stroke="{stroke}" stroke-width="1.5"{dash}/>'
    )
    ty = y + PAD + FS_CHK - 3
    parts.append(
        f'<text x="{x + PAD}" y="{ty}" font-size="{FS_CHK}" font-weight="700" '
        f'fill="{C_CHK_INK}">{card.title}</text>'
    )
    label = "BLOCKING" if card.blocking else "ADVISORY"
    fill = C_TAG_BG if card.blocking else C_TAG_ADV
    tw = len(label) * 6.6 + 18
    parts.append(
        f'<rect x="{x + w - PAD - tw}" y="{y + PAD - 3}" width="{tw}" height="{TAG_H}" '
        f'rx="{TAG_H / 2}" fill="{fill}"/>'
    )
    parts.append(
        f'<text x="{x + w - PAD - tw / 2}" y="{y + PAD + TAG_H / 2 - 4}" text-anchor="middle" '
        f'font-size="{FS_TAG}" font-weight="700" letter-spacing=".06em" fill="#ffffff">{label}</text>'
    )
    ly = ty + 15
    for ln in card.lines:
        parts.append(
            f'<text x="{x + PAD}" y="{ly}" font-size="{FS_CHK_LINE}" '
            f'fill="{C_CHK_INK}">{ln}</text>'
        )
        ly += LH_BODY
    if card.loop:
        parts.append(
            f'<text x="{x + PAD}" y="{ly + 1}" font-size="{FS_LOOP}" font-weight="600" '
            f'fill="{C_LOOP}">&#8617; {card.loop}</text>'
        )
    return "".join(parts)


# ---- lifecycle rows -------------------------------------------------------
ROWS = [
    dict(
        kind="entry",
        name="givn-explore",
        subs=["optional entry &#183; no artifact"],
        optional=True,
        art_title=["no chain artifact"],
        art_lines=[
            "clarify, investigate, compare",
            "route to givn-ideation or",
            "directly to givn-propose",
        ],
        checks=[],
    ),
    dict(
        kind="step",
        name="givn-ideation",
        subs=["optional &#183; skill givn-ideation"],
        optional=True,
        art_title=["givn/ideation/&lt;topic&gt;/"],
        art_lines=[
            "multi-session event storming:",
            "domain events, personas, use cases,",
            "change seeds; no chain artifact",
        ],
        checks=[],
    ),
    dict(
        kind="step",
        num="1",
        name="propose",
        subs=["skill givn-propose"],
        art_title=["proposal.md"],
        art_lines=[
            "WHAT and WHY in domain language;",
            "release note, capability routing,",
            "open questions, review kind",
        ],
        checks=[],
    ),
    dict(
        kind="step",
        num="2",
        name="spec",
        subs=["skill givn-spec"],
        art_title=["specs/&lt;usecase&gt;/", "&lt;capability&gt;.feature"],
        art_lines=[
            "@givn.delta @&lt;capability&gt;;",
            "scenarios @givn.added, @modified,",
            "or @removed; @wip until implemented;",
            "one @e2e per normalized interaction",
        ],
        checks=[
            Card(
                "givn spec duplicates &#183; givn spec coverage",
                [
                    "deterministic duplicate and coverage reports",
                    "over the permanent corpus",
                ],
                blocking=False,
            ),
            Card(
                "givn lint",
                ["static parse, @wip, and overlap check; advisory here"],
                blocking=False,
                loop="enforced change-scoped by givn check review",
            ),
        ],
    ),
    dict(
        kind="step",
        num="3",
        name="design",
        subs=["skill givn-design"],
        art_title=["design.md"],
        art_lines=[
            "technology, step locations, decisions,",
            "interaction matrix, visual contract,",
            "local runnability",
        ],
        checks=[],
    ),
    dict(
        kind="step",
        num="4",
        name="arc42-docs",
        subs=["addon &#183; default on"],
        art_title=["arc42.md &#183; docs/arc42/"],
        art_lines=[
            "12-chapter impact assessment;",
            "affected chapters, ADRs, and glossary",
            "updated; ends with STATUS: DONE",
        ],
        checks=[
            Card(
                "arc42 archive gate",
                [
                    "arc42-docs is required while the addon is on;",
                    "design-review requires it and archive refuses",
                    "an incomplete assessment",
                ],
            ),
        ],
    ),
    dict(
        kind="step",
        num="5",
        name="design-review",
        subs=["skill givn-design-review"],
        art_title=["design-review.md"],
        art_lines=[
            "plan grilled one question at a time,",
            "hardened, signed; review context and",
            "sign-off marker recorded",
        ],
        checks=[
            Card(
                "design-review sign-off",
                [
                    "design-review.md must end with DESIGN-REVIEW: PASS;",
                    "an unsigned review blocks archive; design edits",
                    "require a re-run",
                ],
            ),
        ],
    ),
    dict(
        kind="step",
        num="6",
        name="tasks",
        subs=["skill givn-tasks"],
        art_title=["tasks.md"],
        art_lines=[
            "setup task first; behavior batches;",
            "strict-mode proof; one COMMIT per batch,",
            "ticked as it lands",
        ],
        checks=[],
    ),
    dict(
        kind="step",
        num="7",
        name="implement",
        subs=["skill givn-implement"],
        art_title=["production code &#183; tests", "step definitions"],
        art_lines=[
            "one behavior batch at a time: test,",
            "implement, refactor, then one atomic",
            "commit; tick the task when it lands",
        ],
        checks=[
            Card(
                "givn commit",
                [
                    "explicit paths; empty or outside-checkout refused",
                    "release note is the subject; batch in the body;",
                    "the commit must contain implementation or test",
                ],
            ),
            Card(
                "givn check tasks",
                [
                    "no non-removed @wip; checked scenarios need commits",
                    "clean tree; verify.command green on the delta;",
                    "writes the change verification receipt",
                ],
            ),
        ],
    ),
    dict(
        kind="step",
        num="8",
        name="review",
        subs=["givn check review", "skill givn-review"],
        art_title=["review.md"],
        art_lines=[
            "implementation evidence, E2E integrity,",
            "gap classification, overlap/routing,",
            "review context; ends with REVIEW: PASS",
        ],
        checks=[
            Card(
                "givn check review",
                [
                    "runs no suites; validates the verification receipt;",
                    "integrity, run declaration, change-scoped lint,",
                    "overlap, retirement, routing, scenario commits",
                ],
            ),
            Card(
                "review sign-off",
                [
                    "review.md ends with REVIEW: PASS; every gap classified;",
                    "independent kind enforced when declared in the proposal",
                ],
            ),
        ],
    ),
    dict(
        kind="step",
        num="9",
        name="archive",
        subs=["givn archive --change &lt;id&gt;"],
        art_title=["givn/specs/&lt;usecase&gt;/", "givn/archive/&lt;id&gt;/"],
        art_lines=[
            "deltas merged into permanent specs;",
            "@givn.* stripped; change moved;",
            "proven receipt; evidence-only finalize",
        ],
        checks=[
            Card(
                "archive static gates",
                [
                    "tasks, review, arc42 complete; signed design-review;",
                    "fresh receipt; delta steps bound; archive hooks green;",
                    "run declaration, README, use-case/delta, static review gates",
                ],
            ),
            Card(
                "post-merge proof",
                [
                    "merged candidate: regular + E2E scopes + coverage gate;",
                    "HEAD guard; proven receipt; evidence-only finalization;",
                    "failure leaves the candidate unproven, repairable forward",
                ],
            ),
        ],
    ),
]


def artifact_height(row):
    return (
        PAD * 2
        + len(row["art_title"]) * LH_TITLE
        + 4
        + len(row["art_lines"]) * LH_BODY
    )


def checks_height(checks):
    if not checks:
        return 0
    return sum(c.height for c in checks) + CHECK_GAP * (len(checks) - 1)


def row_height(row):
    return max(STEP_H, artifact_height(row), checks_height(row["checks"]))


def render_step(x, y, w, row):
    parts = []
    optional = row.get("optional", False)
    if optional:
        parts.append(
            f'<rect x="{x}" y="{y}" width="{w}" height="{STEP_H}" rx="12" '
            f'fill="{C_CARD}" stroke="{C_ADV_STROKE}" stroke-width="1.5" '
            f'stroke-dasharray="4 3"/>'
        )
        name_fill = C_INK
        sub_fill = C_MUTED
    else:
        parts.append(
            f'<rect x="{x}" y="{y}" width="{w}" height="{STEP_H}" rx="12" '
            f'fill="url(#stepbg)"/>'
        )
        ccx, ccy = x + 24, y + STEP_H / 2
        parts.append(
            f'<circle cx="{ccx}" cy="{ccy}" r="12" fill="#ffffff" fill-opacity=".25"/>'
        )
        parts.append(
            f'<text x="{ccx}" y="{ccy + 4.5}" text-anchor="middle" font-size="{FS_NUM}" '
            f'font-weight="700" fill="#ffffff">{row["num"]}</text>'
        )
        name_fill = "#ffffff"
        sub_fill = "#dbeafe"

    name_x = x + w / 2 + (14 if not optional else 0)
    subs = row["subs"]
    if len(subs) == 1:
        name_y = y + STEP_H / 2 - 2
        sub_y = y + STEP_H / 2 + 17
        sub_lines = [(subs[0], sub_y)]
    else:
        name_y = y + STEP_H / 2 - 6
        sub_lines = [
            (line, y + STEP_H / 2 + 10 + i * 14) for i, line in enumerate(subs)
        ]
    parts.append(
        f'<text x="{name_x}" y="{name_y}" text-anchor="middle" font-size="{FS_STEP}" '
        f'font-weight="700" fill="{name_fill}">{row["name"]}</text>'
    )
    for line, sy in sub_lines:
        parts.append(
            f'<text x="{name_x}" y="{sy}" text-anchor="middle" font-size="{FS_STEP_SUB}" '
            f'fill="{sub_fill}">{line}</text>'
        )
    return "".join(parts)


def render_artifact(x, y, w, h, row):
    optional = row.get("optional", False)
    dash = ' stroke-dasharray="4 3"' if optional else ""
    parts = [
        f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="12" '
        f'fill="{C_CARD}" stroke="{C_ART_STROKE}" stroke-width="1.5"{dash}/>'
    ]
    ty = y + PAD + FS_ART - 3
    for title in row["art_title"]:
        parts.append(
            f'<text x="{x + PAD}" y="{ty}" font-size="{FS_ART}" '
            f'font-weight="700" font-family="{MONO}" fill="{C_ART_NAME}">{title}</text>'
        )
        ty += LH_TITLE
    if optional:
        tw = len("OPTIONAL") * 6.6 + 18
        parts.append(
            f'<rect x="{x + w - PAD - tw}" y="{y + PAD - 3}" width="{tw}" '
            f'height="{TAG_H}" rx="{TAG_H / 2}" fill="{C_TAG_ADV}"/>'
        )
        parts.append(
            f'<text x="{x + w - PAD - tw / 2}" y="{y + PAD + TAG_H / 2 - 4}" '
            f'text-anchor="middle" font-size="{FS_TAG}" font-weight="700" '
            f'letter-spacing=".06em" fill="#ffffff">OPTIONAL</text>'
        )
    ly = y + PAD + len(row["art_title"]) * LH_TITLE + 4 + FS_ART_LINE - 2
    for line in row["art_lines"]:
        parts.append(
            f'<text x="{x + PAD}" y="{ly}" font-size="{FS_ART_LINE}" '
            f'fill="{C_MUTED}">{line}</text>'
        )
        ly += LH_BODY
    return "".join(parts)


def arrow_down(x, y1, y2, dashed=False):
    d = ' stroke-dasharray="4 3"' if dashed else ""
    return (
        f'<line x1="{x}" y1="{y1}" x2="{x}" y2="{y2 - 11}" stroke="{C_LINE}" '
        f'stroke-width="2"{d}/>'
        f'<polygon points="{x - 8},{y2 - 12} {x + 8},{y2 - 12} {x},{y2}" fill="{C_LINE}"/>'
    )


def arrow_right(x1, y, x2, dashed=False):
    d = ' stroke-dasharray="4 3"' if dashed else ""
    return (
        f'<line x1="{x1}" y1="{y}" x2="{x2 - 10}" y2="{y}" stroke="{C_LINE}" '
        f'stroke-width="2"{d}/>'
        f'<polygon points="{x2 - 9},{y - 6} {x2 - 9},{y + 6} {x2},{y}" fill="{C_LINE}"/>'
    )


def render_panel(x, y, w, h, title, lines, chips=None):
    parts = [
        f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="14" '
        f'fill="{C_CARD}" stroke="{C_LINE}" stroke-width="1.5"/>',
        f'<text x="{x + 22}" y="{y + 30}" font-size="16" font-weight="700" '
        f'fill="{C_INK}">{title}</text>',
    ]
    ly = y + 56
    for line in lines:
        parts.append(
            f'<text x="{x + 22}" y="{ly}" font-size="{FS_PANEL}" fill="{C_MUTED}">{line}</text>'
        )
        ly += 20
    if chips:
        cx = x + 22
        cy = ly - 4
        for chip in chips:
            cw = text_width(chip, 12) + 24
            parts.append(
                f'<rect x="{cx}" y="{cy}" width="{cw}" height="24" rx="12" '
                f'fill="{C_BG}" stroke="{C_LINE}" stroke-width="1.5"/>'
            )
            parts.append(
                f'<text x="{cx + cw / 2}" y="{cy + 16}" text-anchor="middle" '
                f'font-size="12" font-family="{MONO}" fill="{C_ART_NAME}">{chip}</text>'
            )
            cx += cw + 8
    return "".join(parts)


def main():
    heights = [row_height(r) for r in ROWS]
    positions = []
    y = 176
    HEADER = 176
    for i, h in enumerate(heights):
        positions.append(y)
        y += h
        if i < len(ROWS) - 1:
            y += VGAP
    last_bottom = positions[-1] + heights[-1]
    footer_y = last_bottom + VGAP
    panel_h = 132
    H = footer_y + panel_h + 52

    def step_top(i):
        return positions[i] + (heights[i] - STEP_H) / 2

    def step_bottom(i):
        return step_top(i) + STEP_H

    def art_top(i):
        return positions[i] + (heights[i] - artifact_height(ROWS[i])) / 2

    def checks_top(i):
        ch = checks_height(ROWS[i]["checks"])
        return positions[i] + (heights[i] - ch) / 2

    s = []
    s.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" role="img" aria-labelledby="t d">'
    )
    s.append('<title id="t">Givn change lifecycle: explore, ideation, and the main chain</title>')
    s.append(
        '<desc id="d">Top-to-bottom lifecycle with three columns: action, artifact, and checks. '
        'givn-explore starts without an artifact and routes either through optional givn-ideation '
        'or directly to givn-propose. The main chain runs proposal, specs, design, arc42-docs '
        '(addon), design-review, tasks, implementation, review, and archive. Blocking checks stop '
        'the workflow; advisory checks only report.</desc>'
    )
    s.append('<defs>')
    s.append(
        '<linearGradient id="stepbg" x1="0" y1="0" x2="1" y2="1">'
        '<stop offset="0" stop-color="#1d4ed8"/>'
        '<stop offset="1" stop-color="#3b82f6"/>'
        '</linearGradient>'
    )
    s.append('</defs>')
    s.append(f'<rect width="{W}" height="{H}" fill="{C_BG}"/>')
    s.append(f'<g font-family="{FONT}">')

    # header
    s.append(
        f'<text x="{MARGIN}" y="46" font-size="{FS_TITLE}" font-weight="800" fill="{C_INK}">'
        'givn &#8212; change lifecycle: explore to archive</text>'
    )
    header_lines = [
        "Start with givn-explore: clarify without artifacts. Two routes leave it:",
        "optional givn-ideation, or the dashed direct rail to givn-propose. The middle",
        "column names the artifact; the right column holds the checks. Only BLOCKING checks stop the workflow.",
    ]
    sy = 76
    for line in header_lines:
        s.append(
            f'<text x="{MARGIN}" y="{sy}" font-size="{FS_SUB}" fill="{C_MUTED}">{line}</text>'
        )
        sy += 20

    lx = MARGIN
    legend = [
        (C_STEP, None, "action", False),
        (C_CARD, C_ART_STROKE, "artifact", False),
        (C_CHK_BG, C_CHK_STROKE, "blocking check", False),
        (C_ADV_BG, C_ADV_STROKE, "advisory check", True),
        (C_CARD, C_ADV_STROKE, "optional entry", True),
    ]
    for fill, stroke, label, dashed in legend:
        d = ' stroke-dasharray="4 3"' if dashed else ""
        s.append(
            f'<rect x="{lx}" y="128" width="15" height="15" rx="4" fill="{fill}" '
            f'stroke="{stroke or "None"}" stroke-width="1.5"{d}/>'
        )
        s.append(
            f'<text x="{lx + 22}" y="140" font-size="{FS_LEGEND}" fill="{C_MUTED}">{label}</text>'
        )
        lx += 22 + text_width(label, FS_LEGEND) + 28

    # connectors and the compact direct rail from explore to propose
    for i in range(1, len(ROWS)):
        dashed = i <= 2
        s.append(
            f'<line x1="{SPINE_X}" y1="{step_bottom(i - 1) + 3}" x2="{SPINE_X}" '
            f'y2="{step_top(i) - 11}" stroke="{C_LINE}" stroke-width="2"'
            f'{" stroke-dasharray=\"4 3\"" if dashed else ""}/>'
        )
        s.append(
            f'<polygon points="{SPINE_X - 8},{step_top(i) - 12} {SPINE_X + 8},{step_top(i) - 12} '
            f'{SPINE_X},{step_top(i)}" fill="{C_LINE}"/>'
        )

    # Compact direct rail: starts at the lower right of explore, runs in the
    # gap between the action and artifact columns, and enters propose at its
    # lower right. It never crosses the ideation row's cards.
    x_lane = MARGIN + STEP_W + 24
    start_y = step_bottom(0) - 8
    end_y = step_bottom(2) - 8
    s.append(
        f'<polyline points="{MARGIN + STEP_W},{start_y} {x_lane},{start_y} '
        f'{x_lane},{end_y} {MARGIN + STEP_W + 12},{end_y}" '
        f'fill="none" stroke="{C_ADV_STROKE}" stroke-width="2" stroke-dasharray="4 3"/>'
    )
    s.append(
        f'<polygon points="{MARGIN + STEP_W},{end_y} {MARGIN + STEP_W + 12},{end_y - 6} '
        f'{MARGIN + STEP_W + 12},{end_y + 6}" fill="{C_ADV_STROKE}"/>'
    )
    label_y = positions[0] + heights[0] + 8
    s.append(
        f'<rect x="{MARGIN + STEP_W - 64}" y="{label_y}" width="88" height="18" rx="9" '
        f'fill="{C_ADV_BG}" stroke="{C_ADV_STROKE}" stroke-width="1.2" stroke-dasharray="4 3"/>'
    )
    s.append(
        f'<text x="{MARGIN + STEP_W - 20}" y="{label_y + 12.5}" text-anchor="middle" '
        f'font-size="10.5" font-weight="700" fill="{C_ADV_STROKE}">direct route</text>'
    )

    # rows
    for i, row in enumerate(ROWS):
        ry = positions[i]
        rh = heights[i]
        st = step_top(i)
        s.append(render_step(MARGIN, st, STEP_W, row))
        s.append(
            arrow_right(
                MARGIN + STEP_W + 4,
                st + STEP_H / 2,
                ART_X,
                dashed=row.get("optional", False),
            )
        )
        at = art_top(i)
        s.append(render_artifact(ART_X, at, ART_W, artifact_height(row), row))

        checks = row["checks"]
        if checks:
            cy = checks_top(i)
            for card in checks:
                s.append(render_card(card, CHK_X, cy, CHK_W))
                cy += card.height + CHECK_GAP

    # footer panels
    panel_w = (W - 2 * MARGIN - 20) / 2
    left_x = MARGIN
    right_x = MARGIN + panel_w + 20
    s.append(
        render_panel(
            left_x,
            footer_y,
            panel_w,
            panel_h,
            "Chokepoints",
            [
                "Three lifecycle checkpoints stop the change between phases:",
                "givn lint and givn commit also fail closed on their own contracts.",
            ],
            chips=["givn check tasks", "givn check review", "givn archive"],
        )
    )
    s.append(
        render_panel(
            right_x,
            footer_y,
            panel_w,
            panel_h,
            "Entry and alternate paths",
            [
                "givn-challenge examines any artifact with evidence; no gate.",
                "Reverse engineering: givn-reverse-engineer &#8594;",
                "givn-characterize &#8594; archive. Scaffold with:",
                "givn new &lt;id&gt; --skip proposal,design,design-review",
            ],
        )
    )
    s.append(
        f'<text x="{MARGIN}" y="{footer_y + panel_h + 30}" font-size="12" fill="{C_MUTED}">'
        'workflow.md &#183; reference.md &#183; README.md</text>'
    )

    s.append("</g></svg>")

    out = "\n".join(s)
    with open("docs/workflow.svg", "w", encoding="utf-8") as fh:
        fh.write(out)
    print(f"wrote docs/workflow.svg ({W}x{H})")


if __name__ == "__main__":
    main()
