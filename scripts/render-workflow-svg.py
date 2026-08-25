#!/usr/bin/env python3
"""Generate docs/workflow.svg — GitHub-safe swimlane rendering of the givn lifecycle.

Constraints honoured: no <style>, no classes, no foreignObject, no filters,
no external resources; presentation attributes only; explicit viewBox;
solid light background so the diagram reads identically in GitHub light/dark.

Run from repo root:  python3 scripts/render-workflow-svg.py
"""

W = 1240
MARGIN = 24
STEP_W = 200
GAP_W = 56
ART_W = 360
ART_X = MARGIN + STEP_W + GAP_W
CHK_X = ART_X + ART_W + 24
CHK_W = W - CHK_X - MARGIN

FONT = "'Segoe UI',-apple-system,Roboto,Helvetica,Arial,sans-serif"
MONO = "ui-monospace,'Cascadia Code',Menlo,Consolas,monospace"

# ---- type scale -----------------------------------------------------------
FS_TITLE = 24
FS_SUB = 14
FS_LEGEND = 13
FS_NUM = 13
FS_STEP = 16
FS_STEP_SUB = 11
FS_ART = 15          # artifact filename (mono)
FS_ART_LINE = 13     # artifact description
FS_CHK = 15          # check card title
FS_CHK_LINE = 13     # check card body
FS_LOOP = 12.5       # loopback note
FS_TAG = 10          # BLOCKING / ADVISORY pill
FS_PANEL = 16        # panel heading
FS_MODEL = 13.5      # model id (mono)
FS_MODEL_ROLE = 12
FS_THR = 13
FS_FACT = 12
FS_CHOKE = 11

LH_TITLE = 20        # card title line-height
LH_BODY = 18         # card body line-height
PAD = 12             # card padding
STEP_H = 62          # step pill height
VGAP = 42            # vertical gap between lanes
TAG_H = 17           # tag pill height
CHAR = 0.56 * FS_ART_LINE   # conservative glyph width for audits

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


PHASES = [
    dict(
        num="1", name="propose", sub="/givn-propose",
        artifact="proposal.md",
        art_lines=["WHAT and WHY, domain language,", "no implementation detail"],
        checks=[],
    ),
    dict(
        num="2", name="spec", sub="/givn-spec",
        artifact="specs/*.feature",
        art_lines=[
            "delta scenarios (add/modified/removed),",
            "&#8805; 1 E2E-tested per capability",
        ],
        checks=[
            Card("givn spec search / explore",
                 ["E5 search for existing spec and overlap",
                  "to avoid duplicates"], blocking=False),
            Card("givn lint",
                 ["syntax &#183; @wip tags &#183; overlap warnings (severity-ranked exit code)"],
                 blocking=False,
                 loop="runs again as a blocking built-in hook at every gate"),
        ],
    ),
    dict(
        num="3", name="design", sub="/givn-design",
        artifact="design.md",
        art_lines=["HOW to implement, config,", "e2e infrastructure"],
        checks=[],
    ),
    dict(
        num="4", name="design-review", sub="/givn-design-review",
        artifact="design-review.md",
        art_lines=["plan stress-tested one question", "at a time, then hardened"],
        checks=[
            Card("sign-off marker gate",
                 ["file must end with &#8220;DESIGN-REVIEW: PASS&#8221; &#8212; otherwise the artifact counts as",
                  "partial and blocks archiving; design deviations un-gate this review"]),
        ],
    ),
    dict(
        num="5", name="tasks", sub="/givn-tasks",
        artifact="tasks.md",
        art_lines=["setup task first, then RED / GREEN /",
                   "REFACTOR / COMMIT per scenario",
                   "&#8212; @e2e scenarios last"],
        checks=[
            Card("givn check tasks",
                 ["@wip gate: no scenario may remain work in progress"]),
        ],
    ),
    dict(
        num="6", name="implement", sub="/givn-implement",
        artifact="production code",
        art_lines=["one targeted TDD cycle per scenario, one",
                   "atomic commit &#8212; hash recorded in",
                   "tasks.md immediately"],
        checks=[
            Card("per-scenario runner proof",
                 ["RED must exit non-zero (exit 0 = hard stop)",
                  "GREEN and REFACTOR must exit zero"]),
        ],
    ),
    dict(
        num="7", name="review", sub="givn check review",
        artifact="review.md + semantic-review.md",
        art_lines=["coverage gap classification,", "fabrication audit, worklist"],
        checks=[
            Card("check review battery",
                 ["verify suite &#8594; verify-e2e anti-bypass &#8594; integrity (commit hashes,",
                  "promised files) &#8594; overlap dispositions"]),
            Card("semantic duplicate review",
                 ["E5 checks specs for duplicates",
                  "classify DUPLICATE / VARIANT / VALID-BOUNDARY"],
                 loop="unresolved &#8658; rewrite or delete the scenario, rerun"),
            Card("coverage sign-off",
                 ["every uncovered line classified (dead code / missing test /",
                  "justified) &#8212; file ends with &#8220;REVIEW: PASS&#8221;"]),
        ],
    ),
    dict(
        num="8", name="archive", sub="archive --change &lt;id&gt;",
        artifact="givn/specs/* &#183; givn/archive/&lt;id&gt;/",
        art_lines=["deltas merged into permanent specs,",
                   "@givn.* stripped, change moved",
                   "to givn/archive/&lt;id&gt;/"],
        checks=[
            Card("archive gates",
                 ["required artifacts complete &#183; PASS markers present &#183; hooks green &#183; fresh",
                  "similarity gate re-run directly before the merge"]),
            Card("post-merge proof",
                 ["verify suite (+ e2e) against merged specs &#183; coverage addon gate &#8212; any",
                  "failure rolls specs and README back from the backup"]),
        ],
    ),
]


def phase_geometry(p):
    art_h = PAD * 2 + LH_TITLE + 4 + len(p["art_lines"]) * LH_BODY
    chk_gap = 12
    chk_h = sum(c.height for c in p["checks"]) + chk_gap * (len(p["checks"]) - 1)
    return max(art_h, chk_h, STEP_H), art_h, chk_h, chk_gap


def main():
    y = 148
    rows = []
    for p in PHASES:
        lane_h, art_h, chk_h, chk_gap = phase_geometry(p)
        rows.append(dict(p=p, top=y, lane_h=lane_h, art_h=art_h, chk_h=chk_h, chk_gap=chk_gap))
        y += lane_h + VGAP
    lanes_end = y - VGAP

    H = lanes_end + MARGIN

    s = []
    s.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" role="img" aria-labelledby="t d">'
    )
    s.append('<title id="t">Givn change lifecycle: proposal to archive</title>')
    s.append(
        '<desc id="d">Swimlane view: one lane per artifact, blue steps create, '
        'grey cards are artifacts, yellow cards are blocking or advisory checks, '
        'ending in merge into the permanent specs.</desc>'
    )
    s.append(f'<rect width="{W}" height="{H}" fill="#ffffff"/>')
    s.append(f'<g font-family="{FONT}">')

    s.append(f'<text x="{MARGIN}" y="50" font-size="{FS_TITLE}" font-weight="800" fill="{C_INK}">')
    s.append('givn &#8212; change lifecycle: proposal to archive</text>')
    sub_lines = [
        'One lane per artifact. Every check blocks unless tagged ADVISORY. Between gates the order is',
        'advisory only &#8212; enforcement concentrates at the three chokepoints listed below.',
    ]
    sy_ = 74
    for ln in sub_lines:
        s.append(f'<text x="{MARGIN}" y="{sy_}" font-size="{FS_SUB}" fill="{C_MUTED}">{ln}</text>')
        sy_ += 19

    lx = MARGIN
    legend = [
        (C_STEP, None, "action / step"),
        ("#ffffff", C_ART_STROKE, "artifact created"),
        (C_CHK_BG, C_CHK_STROKE, "blocking check"),
        (C_ADV_BG, C_ADV_STROKE, "advisory check"),
    ]
    for fill, stroke, label in legend:
        dash = ' stroke-dasharray="4 3"' if fill == C_ADV_BG else ""
        s.append(
            f'<rect x="{lx}" y="100" width="15" height="15" rx="4" fill="{fill}" '
            f'stroke="{stroke}" stroke-width="1.5"{dash}/>'
        )
        s.append(
            f'<text x="{lx + 22}" y="112" font-size="{FS_LEGEND}" fill="{C_MUTED}">{label}</text>'
        )
        lx += 22 + len(label) * CHAR * (FS_LEGEND / FS_ART_LINE) + 30

    for i, r in enumerate(rows):
        p = r["p"]
        top, lane_h = r["top"], r["lane_h"]
        scy = top + lane_h / 2
        sx, sy = MARGIN, scy - STEP_H / 2

        if i > 0:
            prev = rows[i - 1]
            prev_cy = prev["top"] + prev["lane_h"] / 2
            x = MARGIN + STEP_W / 2
            s.append(
                f'<line x1="{x}" y1="{prev_cy + STEP_H / 2 + 4}" x2="{x}" y2="{sy - 10}" '
                f'stroke="{C_LINE}" stroke-width="2"/>'
            )
            s.append(
                f'<polygon points="{x - 8},{sy - 11} {x + 8},{sy - 11} {x},{sy}" fill="{C_LINE}"/>'
            )

        s.append(
            f'<rect x="{sx}" y="{sy}" width="{STEP_W}" height="{STEP_H}" rx="12" fill="{C_STEP}"/>'
        )
        ccx, ccy = sx + 26, sy + STEP_H / 2
        s.append(f'<circle cx="{ccx}" cy="{ccy}" r="12" fill="#ffffff" fill-opacity=".25"/>')
        s.append(
            f'<text x="{ccx}" y="{ccy + 4.5}" text-anchor="middle" font-size="{FS_NUM}" '
            f'font-weight="700" fill="#ffffff">{p["num"]}</text>'
        )
        s.append(
            f'<text x="{sx + STEP_W / 2 + 16}" y="{ccy - 2}" text-anchor="middle" font-size="{FS_STEP}" '
            f'font-weight="700" fill="#ffffff">{p["name"]}</text>'
        )
        s.append(
            f'<text x="{sx + STEP_W / 2 + 16}" y="{ccy + 17}" text-anchor="middle" font-size="{FS_STEP_SUB}" '
            f'fill="#dbeafe">{p["sub"]}</text>'
        )

        ay = scy - 1
        s.append(
            f'<line x1="{sx + STEP_W + 4}" y1="{ay}" x2="{ART_X - 10}" y2="{ay}" '
            f'stroke="{C_LINE}" stroke-width="2"/>'
        )
        s.append(
            f'<polygon points="{ART_X - 9},{ay - 6} {ART_X - 9},{ay + 6} {ART_X},{ay}" fill="{C_LINE}"/>'
        )

        acy = top + (lane_h - r["art_h"]) / 2
        s.append(
            f'<rect x="{ART_X}" y="{acy}" width="{ART_W}" height="{r["art_h"]}" rx="12" '
            f'fill="#ffffff" stroke="{C_ART_STROKE}" stroke-width="1.5"/>'
        )
        s.append(
            f'<text x="{ART_X + PAD}" y="{acy + PAD + FS_ART - 3}" font-size="{FS_ART}" font-weight="700" '
            f'font-family="{MONO}" fill="{C_ART_NAME}">{p["artifact"]}</text>'
        )
        lyy = acy + PAD + LH_TITLE + 4 + FS_ART_LINE - 2
        for ln in p["art_lines"]:
            s.append(
                f'<text x="{ART_X + PAD}" y="{lyy}" font-size="{FS_ART_LINE}" '
                f'fill="{C_MUTED}">{ln}</text>'
            )
            lyy += LH_BODY

        if p["checks"]:
            kcy = top + (lane_h - r["chk_h"]) / 2
            for c in p["checks"]:
                s.append(render_card(c, CHK_X, kcy, CHK_W))
                kcy += c.height + r["chk_gap"]

    s.append("</g></svg>")

    out = "\n".join(s)
    with open("docs/workflow.svg", "w", encoding="utf-8") as fh:
        fh.write(out)
    print(f"wrote docs/workflow.svg ({W}x{H})")


if __name__ == "__main__":
    main()
