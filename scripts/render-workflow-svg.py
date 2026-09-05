#!/usr/bin/env python3
"""Generate docs/workflow.svg — GitHub-safe rendering of the givn lifecycle.

Layout targets an ~830 px content column (GitHub README preview): the diagram
is exactly 830 px wide so it renders 1:1 without downscaling. Flow reads top
to bottom: action pill -> its artifact (arrow to the right) -> check card(s)
below -> next action. No <style>, no classes, no foreignObject, no filters,
no external resources; solid light background for theme independence.

Run from repo root:  python3 scripts/render-workflow-svg.py
"""

W = 830
MARGIN = 20
STEP_W = 170
ART_X = MARGIN + STEP_W + 40          # horizontal arrow lives in the gap
ART_W = W - MARGIN - ART_X            # artifact card fills the rest
CHK_X = MARGIN
CHK_W = W - 2 * MARGIN                # check cards span the full column
SPINE_X = MARGIN + STEP_W / 2         # vertical connector position

FONT = "'Segoe UI',-apple-system,Roboto,Helvetica,Arial,sans-serif"
MONO = "ui-monospace,'Cascadia Code',Menlo,Consolas,monospace"

# ---- type scale -----------------------------------------------------------
FS_TITLE = 24
FS_SUB = 14
FS_LEGEND = 13
FS_NUM = 13
FS_STEP = 16
FS_STEP_SUB = 11
FS_ART = 15           # artifact filename (mono)
FS_ART_LINE = 13      # artifact description
FS_CHK = 15           # check card title
FS_CHK_LINE = 13      # check card body
FS_LOOP = 12.5        # loopback note
FS_TAG = 10           # BLOCKING / ADVISORY pill

LH_TITLE = 20
LH_BODY = 18
PAD = 12
STEP_H = 62
VGAP = 36             # vertical connector length between stacked nodes
TAG_H = 17
CHAR = 0.56 * FS_CHK_LINE

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
        art_lines=["WHAT and WHY, domain language, no implementation detail"],
        checks=[],
    ),
    dict(
        num="2", name="spec", sub="/givn-spec",
        artifact="specs/*.feature",
        art_lines=["delta scenarios (add/modified/removed), &#8805; 1 E2E-tested per capability"],
        checks=[
            Card("givn spec search / explore",
                 ["E5 search for existing spec and overlap to avoid duplicates"],
                 blocking=False),
            Card("givn lint",
                 ["syntax &#183; @wip tags &#183; overlap warnings (severity-ranked exit code)"],
                 blocking=False,
                 loop="runs again as a blocking built-in hook at every gate"),
        ],
    ),
    dict(
        num="3", name="design", sub="/givn-design",
        artifact="design.md",
        art_lines=["HOW to implement, config, e2e infrastructure"],
        checks=[],
    ),
    dict(
        num="4", name="design-review", sub="/givn-design-review",
        artifact="design-review.md",
        art_lines=["plan stress-tested one question at a time, then hardened"],
        checks=[
            Card("sign-off marker gate",
                 ["file must end with &#8220;DESIGN-REVIEW: PASS&#8221; &#8212; otherwise partial;",
                  "design deviations un-gate this review"]),
        ],
    ),
    dict(
        num="5", name="tasks", sub="/givn-tasks",
        artifact="tasks.md",
        art_lines=["setup task first, then RED / GREEN / REFACTOR / COMMIT",
                   "per scenario &#8212; @e2e scenarios last"],
        checks=[
            Card("givn check tasks",
                 ["@wip gate: no scenario may remain work in progress"]),
        ],
    ),
    dict(
        num="6", name="implement", sub="/givn-implement",
        artifact="production code",
        art_lines=["one targeted TDD cycle per scenario, one atomic commit",
                   "&#8212; hash recorded in tasks.md immediately"],
        checks=[
            Card("per-scenario runner proof",
                 ["RED must exit non-zero (exit 0 = hard stop) &#183;",
                  "GREEN and REFACTOR must exit zero"]),
        ],
    ),
    dict(
        num="7", name="review", sub="givn check review",
        artifact="review.md + semantic-review.md",
        art_lines=["coverage gap classification, fabrication audit, worklist"],
        checks=[
            Card("check review battery",
                 ["verify suite &#8594; verify-e2e anti-bypass &#8594; integrity (commit hashes, promised files)",
                  "&#8594; overlap dispositions"]),
            Card("semantic duplicate review",
                 ["E5 checks specs for duplicates &#8212; classify",
                  "DUPLICATE / VARIANT / VALID-BOUNDARY"],
                 loop="unresolved &#8658; rewrite or delete the scenario, rerun"),
            Card("coverage sign-off",
                 ["every uncovered line classified (dead code / missing test / justified)",
                  "&#8212; file ends with &#8220;REVIEW: PASS&#8221;"]),
        ],
    ),
    dict(
        num="8", name="archive", sub="archive --change &lt;id&gt;",
        artifact="givn/specs/* &#183; givn/archive/&lt;id&gt;/",
        art_lines=["deltas merged into permanent specs, @givn.* stripped,",
                   "change moved to givn/archive/&lt;id&gt;/"],
        checks=[
            Card("archive gates",
                 ["required artifacts complete &#183; PASS markers present &#183; hooks green &#183;",
                  "fresh similarity gate re-run directly before the merge"]),
            Card("post-merge proof",
                 ["verify suite (+ e2e) against merged specs &#183; coverage addon gate &#8212;",
                  "any failure rolls specs and README back from the backup"]),
        ],
    ),
]


# Optional upstream entry: a domain-driven event-storming flow that precedes
# the chain. Produces topic state under givn/ideation/ and change seeds — no
# manifest artifact, no gates, rendered dashed to mark it as skippable.
ENTRY = dict(
    name="ideation",
    sub="/givn-ideation &#183; event storming",
    artifact="givn/ideation/&lt;topic&gt;/",
    art_lines=[
        "multi-session storm: domain events, personas, use cases, change seeds",
        "hands off to propose &#8212; creates no chain artifact",
    ],
)


def main():
    # ---- measure ----------------------------------------------------------
    nodes = []  # ("entry", ENTRY, h) | ("row", phase, h) | ("check", card, h)
    entry_h = max(PAD * 2 + LH_TITLE + 4 + len(ENTRY["art_lines"]) * LH_BODY, STEP_H)
    nodes.append(["entry", ENTRY, entry_h])
    for p in PHASES:
        art_h = PAD * 2 + LH_TITLE + 4 + len(p["art_lines"]) * LH_BODY
        nodes.append(["row", p, max(art_h, STEP_H)])
        for c in p["checks"]:
            nodes.append(["check", c, c.height])

    y = 148
    HEADER = 148
    positions = []
    for i, n in enumerate(nodes):
        positions.append(y)
        y += n[2]
        if i < len(nodes) - 1:
            y += VGAP
    H = y + MARGIN

    s = []
    s.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" role="img" aria-labelledby="t d">'
    )
    s.append('<title id="t">Givn change lifecycle: proposal to archive</title>')
    s.append(
        '<desc id="d">Top-to-bottom flow: an optional ideation entry (domain-driven event '
        'storming) feeds phase 1; each action pill creates the artifact shown to its '
        'right; check cards between the actions gate the transition. Advisory checks never '
        'block; every other check must pass before the next action starts.</desc>'
    )
    s.append(f'<rect width="{W}" height="{H}" fill="#ffffff"/>')
    s.append(f'<g font-family="{FONT}">')

    s.append(f'<text x="{MARGIN}" y="50" font-size="{FS_TITLE}" font-weight="800" fill="{C_INK}">')
    s.append('givn &#8212; change lifecycle: proposal to archive</text>')
    sub_lines = [
        'One action per row, the artifact it creates to its right. Every check between rows',
        'blocks unless tagged ADVISORY.',
    ]
    sy_ = 74
    for ln in sub_lines:
        s.append(f'<text x="{MARGIN}" y="{sy_}" font-size="{FS_SUB}" fill="{C_MUTED}">{ln}</text>')
        sy_ += 19

    lx = MARGIN
    legend = [
        (C_STEP, None, "action", False),
        ("#ffffff", C_ART_STROKE, "artifact", False),
        (C_CHK_BG, C_CHK_STROKE, "blocking check", False),
        (C_ADV_BG, C_ADV_STROKE, "advisory check", True),
        ("#ffffff", C_ART_STROKE, "optional entry", True),
    ]
    for fill, stroke, label, dashed in legend:
        dash = ' stroke-dasharray="4 3"' if dashed else ""
        s.append(
            f'<rect x="{lx}" y="100" width="15" height="15" rx="4" fill="{fill}" '
            f'stroke="{stroke}" stroke-width="1.5"{dash}/>'
        )
        s.append(
            f'<text x="{lx + 22}" y="112" font-size="{FS_LEGEND}" fill="{C_MUTED}">{label}</text>'
        )
        lx += 22 + len(label) * CHAR * (FS_LEGEND / FS_ART_LINE) + 28

    def arrow_down(x, y1, y2, dashed=False):
        d = ' stroke-dasharray="4 3"' if dashed else ""
        return (
            f'<line x1="{x}" y1="{y1}" x2="{x}" y2="{y2 - 11}" stroke="{C_LINE}" '
            f'stroke-width="2"{d}/>'
            f'<polygon points="{x - 8},{y2 - 12} {x + 8},{y2 - 12} {x},{y2}" fill="{C_LINE}"/>'
        )

    prev_bottom = None
    prev_dashed = False
    for idx, ((kind, payload, h), ny) in enumerate(zip(nodes, positions)):
        if prev_bottom is not None:
            s.append(arrow_down(SPINE_X, prev_bottom + 3, ny, dashed=prev_dashed))
        if kind == "entry":
            sx = MARGIN
            sy = ny + (h - STEP_H) / 2
            s.append(
                f'<rect x="{sx}" y="{sy}" width="{STEP_W}" height="{STEP_H}" rx="12" '
                f'fill="#ffffff" stroke="{C_ART_STROKE}" stroke-width="1.5" '
                f'stroke-dasharray="4 3"/>'
            )
            ecy = sy + STEP_H / 2
            s.append(
                f'<text x="{sx + STEP_W / 2}" y="{ecy - 2}" text-anchor="middle" '
                f'font-size="{FS_STEP}" font-weight="700" fill="{C_ART_NAME}">'
                f'{payload["name"]}</text>'
            )
            s.append(
                f'<text x="{sx + STEP_W / 2}" y="{ecy + 17}" text-anchor="middle" '
                f'font-size="{FS_STEP_SUB}" fill="{C_MUTED}">{payload["sub"]}</text>'
            )
            ay = sy + STEP_H / 2
            s.append(
                f'<line x1="{sx + STEP_W + 4}" y1="{ay}" x2="{ART_X - 10}" y2="{ay}" '
                f'stroke="{C_LINE}" stroke-width="2" stroke-dasharray="4 3"/>'
            )
            s.append(
                f'<polygon points="{ART_X - 9},{ay - 6} {ART_X - 9},{ay + 6} {ART_X},{ay}" '
                f'fill="{C_LINE}"/>'
            )
            acy = ny + (h - (PAD * 2 + LH_TITLE + 4 + len(payload["art_lines"]) * LH_BODY)) / 2
            s.append(
                f'<rect x="{ART_X}" y="{acy}" width="{ART_W}" height="{h}" rx="12" '
                f'fill="#ffffff" stroke="{C_ART_STROKE}" stroke-width="1.5" '
                f'stroke-dasharray="4 3"/>'
            )
            s.append(
                f'<text x="{ART_X + PAD}" y="{acy + PAD + FS_ART - 3}" font-size="{FS_ART}" '
                f'font-weight="700" font-family="{MONO}" fill="{C_ART_NAME}">'
                f'{payload["artifact"]}</text>'
            )
            tw = len("OPTIONAL") * 6.6 + 18
            s.append(
                f'<rect x="{ART_X + ART_W - PAD - tw}" y="{acy + PAD - 3}" width="{tw}" '
                f'height="{TAG_H}" rx="{TAG_H / 2}" fill="{C_TAG_ADV}"/>'
            )
            s.append(
                f'<text x="{ART_X + ART_W - PAD - tw / 2}" y="{acy + PAD + TAG_H / 2 - 4}" '
                f'text-anchor="middle" font-size="{FS_TAG}" font-weight="700" '
                f'letter-spacing=".06em" fill="#ffffff">OPTIONAL</text>'
            )
            lyy = acy + PAD + LH_TITLE + 4 + FS_ART_LINE - 2
            for ln in payload["art_lines"]:
                s.append(
                    f'<text x="{ART_X + PAD}" y="{lyy}" font-size="{FS_ART_LINE}" '
                    f'fill="{C_MUTED}">{ln}</text>'
                )
                lyy += LH_BODY
            prev_bottom = ny + h
            prev_dashed = True
        elif kind == "row":
            p = payload
            sx = MARGIN
            sy = ny + (h - STEP_H) / 2
            s.append(
                f'<rect x="{sx}" y="{sy}" width="{STEP_W}" height="{STEP_H}" rx="12" fill="{C_STEP}"/>'
            )
            ccx, ccy = sx + 24, sy + STEP_H / 2
            s.append(f'<circle cx="{ccx}" cy="{ccy}" r="12" fill="#ffffff" fill-opacity=".25"/>')
            s.append(
                f'<text x="{ccx}" y="{ccy + 4.5}" text-anchor="middle" font-size="{FS_NUM}" '
                f'font-weight="700" fill="#ffffff">{p["num"]}</text>'
            )
            s.append(
                f'<text x="{sx + STEP_W / 2 + 14}" y="{ccy - 2}" text-anchor="middle" font-size="{FS_STEP}" '
                f'font-weight="700" fill="#ffffff">{p["name"]}</text>'
            )
            s.append(
                f'<text x="{sx + STEP_W / 2 + 14}" y="{ccy + 17}" text-anchor="middle" font-size="{FS_STEP_SUB}" '
                f'fill="#dbeafe">{p["sub"]}</text>'
            )

            ay = sy + STEP_H / 2
            s.append(
                f'<line x1="{sx + STEP_W + 4}" y1="{ay}" x2="{ART_X - 10}" y2="{ay}" '
                f'stroke="{C_LINE}" stroke-width="2"/>'
            )
            s.append(
                f'<polygon points="{ART_X - 9},{ay - 6} {ART_X - 9},{ay + 6} {ART_X},{ay}" '
                f'fill="{C_LINE}"/>'
            )
            acy = ny + (h - (PAD * 2 + LH_TITLE + 4 + len(p["art_lines"]) * LH_BODY)) / 2
            s.append(
                f'<rect x="{ART_X}" y="{acy}" width="{ART_W}" height="{h}" rx="12" '
                f'fill="#ffffff" stroke="{C_ART_STROKE}" stroke-width="1.5"/>'
            )
            s.append(
                f'<text x="{ART_X + PAD}" y="{acy + PAD + FS_ART - 3}" font-size="{FS_ART}" '
                f'font-weight="700" font-family="{MONO}" fill="{C_ART_NAME}">{p["artifact"]}</text>'
            )
            lyy = acy + PAD + LH_TITLE + 4 + FS_ART_LINE - 2
            for ln in p["art_lines"]:
                s.append(
                    f'<text x="{ART_X + PAD}" y="{lyy}" font-size="{FS_ART_LINE}" '
                    f'fill="{C_MUTED}">{ln}</text>'
                )
                lyy += LH_BODY
            prev_bottom = ny + h
            prev_dashed = False
        else:
            s.append(render_card(payload, CHK_X, ny, CHK_W))
            prev_bottom = ny + h
            prev_dashed = False

    s.append("</g></svg>")

    out = "\n".join(s)
    with open("docs/workflow.svg", "w", encoding="utf-8") as fh:
        fh.write(out)
    print(f"wrote docs/workflow.svg ({W}x{H})")


if __name__ == "__main__":
    main()
