# Domain terms: local-only-integrations

Working vocabulary for this topic. Status: working, proposed, confirmed, or
rejected. Terms merge into the durable glossary only at Handoff.

| Term | Working definition | Status |
|---|---|---|
| Givn rule | A skill, slash command, plugin, or hook that `givn skills sync` renders for an agent target (`.opencode`, `.agents`, `.claude`) or installs as shared plumbing (`.githooks/commit-msg`). User's term for what the glossary currently calls "generated Givn integrations"; "agent integration" is the older synonym. | confirmed |
| Rules policy | The setting that selects whether Givn rules are versioned or local-only: `rules: local-only` (default) or `rules: versioned`, a flat scalar in `givn/config.yaml`. Confirmed 2026-09-23 (Q1): the default applies to new and existing projects alike; an unrecognized value warns and continues as local-only. | confirmed |
| Strict rules | Every Givn-owned generated file that must stay local under the local-only policy: skills, commands, plugins, the shared `commit-msg` dispatcher, and the Claude hook. Excludes the `AGENTS.md` fence (Q3) and every user-owned file. | working |
| Managed generated paths | The broader code list Givn computes for sync and staging (`managed_generated_paths`); it also names `AGENTS.md` and user-owned files such as the shared `.claude/settings.json`. The staging allowlist is derived from it per policy. | working |
| Ignore set | The paths entered into both ignore destinations: the strict rules, each as a precise path, never a directory. The tracked destination is a fenced block (`.gitignore`, `.fossil-settings/ignore-glob`); the local destination is `.git/info/exclude` and the Fossil checkout-local ignore setting. The project-local Claude settings file joins the set once Q19 settles the hook home. | working |
| Tracked / untracked | Git mechanics vocabulary used in the event mechanics; the neutral state terms are versioned and local-only. Fossil's `EXTRA` files are the untracked analogue. | working |
| Local-only transition | The state change from versioned to local-only: the path leaves the current revision but stays on disk, and ignore rules keep it out afterwards. Git realises it with `git rm --cached`; Fossil with `fossil rm --soft` or `fossil forget` (pinned so the user's `mv-rm-files` setting cannot delete files). The event is phrased "became local-only". | working |
| Local-only rule | A Givn rule whose home is the developer's machine, not the repository: the active SCM backend neither tracks nor receives it, and Givn never stages it. `AGENTS.md` is the confirmed exception (Q3): managed fence, never auto-committed, user decides tracking. | confirmed |
| Versioned rule | The opposite state: the rule is repository content, staged and committed like source. Selected by the new setting. | working |
| Givn-owned fenced block | The `<!-- givn:begin:* -->` region inside a user-owned file, currently `AGENTS.md`. Mixed ownership: user prose outside, Givn content inside. Exempt from the strict local-only rule; Givn never auto-commits it. | confirmed |
| Mixed-ownership file | A file whose content is partly user-owned and partly Givn-generated: `AGENTS.md` (fenced blocks), `.claude/settings.json` (merged hook entry). Git cannot version or ignore part of a file. | working |
| Local rules file | Rejected design (Q17/Q18 dropped): a separate generated rules file referenced from a tracked rule file. Superseded by the confirmed `AGENTS.md` boundary (Q3); the only surviving local-file question is the Claude hook home (Q19). | rejected |
| Non-Givn rule file | A rule or guidance file the developer owns, including hand-written skills next to generated ones (example: `.agents/skills/gh-stack/SKILL.md` in this repository), user prose in `AGENTS.md`, and user hooks in `.claude/settings.json`. Givn must never ignore, untrack, or delete these. | working |
| Publishable project | A project whose repository and history carry no generated Givn rule files, and whose build, run, and test paths work with no Givn installed. | working |
| Clean history | Confirmed 2026-09-23 (Q2): no reachable revision contains a strict rule; occurrences are reported and never block; an explicit rewrite (`rewrite-history`) is the remediation. Unreachable objects, remotes, and forks are outside the guarantee. | confirmed |
