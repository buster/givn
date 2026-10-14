# Instruction: grill-me

Socratically grill the current plan or design — one hard question at a time —
before implementation begins.

For every user-facing question, follow the effective policy from
`givn instructions dialog`: name artifacts in domain language, and give every
option its meaning, gain, cost, and a concrete example.

This is a **stress test**: it interrogates the proposal, specs, design, and
tasks for a givn change question by question, refusing to let weak assumptions
hide behind "looks reasonable."

## How to grill

Read the change in order: `proposal.md`, the `.feature` specs, `design.md`,
`tasks.md`. Then pose a single sharp question, wait for the answer, then
follow up. Do not dump a checklist. Good grills:

- What is the single most likely way this breaks in production?
- Which step's assertion is fake — a no-op disguised as a check?
- What does this change assume that the proposal never actually states?
- If you deleted this scenario, which behaviour would regress silently?
- Where does the design duplicate an existing convention instead of reusing it?
- Which task's "Design constraints" block is missing or copy-pasted?

## Push until concrete

Reject answers that stay at the architecture level. Push for the specific file,
function, branch, or edge value. If the answer is "it should be fine," ask what
"fine" means observably.

## Record outcomes

After each resolved question, note it as a bullet: the assumption, the risk, and
the decision. Surface any that need a human call before implementation starts.

## Output

A short list of hardened decisions and the unresolved questions that block
implementation. Do not proceed to implement until the blocking questions are
answered.
