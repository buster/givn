# Manual transition to the 1.0 contract

Givn 1.0 is the supported migration baseline. The shipped migration catalog is
empty: there are no embedded 0.x routes, no automatic 0.7/0.8/0.9 conversion,
and no compatibility alias for retired configuration. A pre-1.0 project moves
by repairing its configuration and regenerating guidance, then reviewing the
result through its normal version control.

This guide is a manual transition, not a promise that every 0.x project layout
is supported. A project whose configuration uses an unknown format must keep its
own copy of the old values and translate the settings it actually uses.

## Before you start

1. Use the target Givn binary. Do not run a mixed set of old and new binaries
   during the transition.
2. Commit or stash unrelated work. `givn upgrade` requires a clean checkout and
   refuses a dirty project before writing.
3. Note the current revision as your checkpoint:

   ```sh
   git rev-parse HEAD
   # or
   fossil info
   ```

4. Keep the old `givn/config.yaml` and `givn/commands.yaml` diff visible. There
   is no automatic rollback; recovery is manual from the checkpoint or fix
   forward.

## 1. Remove retired addon keys

The renamed addons are:

| Retired key | Current key | What changes for the project |
|---|---|---|
| `absolute_mode` | `communication_policy` | The managed `AGENTS.md` block is now `communication-policy`; the mandatory dialog obligation is independent of the addon. |
| `grill_me` | `challenge` | The two grill skills/commands become the single `givn-challenge` skill and the canonical challenge instruction. No lifecycle gate is added. |
| `grill_with_docs` | `challenge` | Same replacement; project-specific review prompts belong in the challenge instruction or a project-owned rule, not a second generated command. |

Edit `givn/config.yaml` by hand:

```yaml
addons:
  communication_policy: true
  challenge: true
```

Remove the retired keys. Givn 1.0 refuses them by name and performs no automatic
conversion. If the old project relied on the former addon's generated files,
delete them after the transition; `givn skills sync` removes recognizable Givn
commands and reports custom files for manual review.

## 2. Move executable settings into `givn/commands.yaml`

The configuration authority is split:

- `givn/config.yaml` owns policy and structure: artifact graph, addons, skills
  targets, rules policy, record mode, and instruction extensions.
- `givn/commands.yaml` owns executable settings: `verify`, `coverage`, and
  `run`.

Move any `verify`, `coverage`, or `run` block out of `config.yaml`. Givn 1.0
refuses those keys in `config.yaml` before writing. The expected commands file
shape is:

```yaml
verify:
  command: "./run-tests.sh --non-e2e"
  e2e_command: "./run-tests.sh --e2e"
  features_path: "givn/specs"

coverage:
  measure_command: "./measure-coverage.sh"
  merge_command: "./merge-coverages.sh"
  non_e2e_path: "coverage/non-e2e-cobertura.xml"
  e2e_path: "coverage/e2e-cobertura.xml"
  output_path: "coverage/cobertura-coverage.xml"

run:
  command: "./start.sh"
  product_type: server
  port_env: "PORT"
  ready_path: "/healthz"
```

`run` is a whole-block replacement. Keep `givn missing-run-command` as the
start command only for a project with no startable product yet; review and
archive refuse an unconfigured declaration.

## 3. Transition from generated commands to skills

Givn 1.0 generates skills only. It does not generate `/givn-*` commands. The
installed invocation surface depends on the target:

| Target | Invocation after the transition |
|---|---|
| Claude Code, OpenCode | native skill slash, for example `/givn-propose` |
| OMP | `/skill:givn-propose` |
| Maki | an explicit request or an agent-selected skill; no generated command |
| `agents` | neutral skill discovery by name |

Persisted targets live in `givn/config.yaml`:

```yaml
skills:
  targets: [opencode, claude, omp, maki]
```

Run:

```sh
givn skills sync
```

Sync regenerates the selected skills, removes recognizable Givn command files
from earlier releases, and preserves a custom command whose name matches but
whose content is not Givn-generated. Review that manual notice and either port
the custom behavior into the project's own rules or delete the stale command
yourself. Do not keep a project command that restates a retired policy.

`absolute_mode` content becomes the `communication_policy` addon block. The
retired `absolute-mode` block in `AGENTS.md` is removed during sync; the
surrounding human content is preserved. Review the diff.

## 4. Update commit usage

`givn commit` requires explicit paths:

```sh
givn commit -m "<release note>" -- givn/changes/<id> src/path tests/steps/path
```

It never stages every pending change. The release note is the message subject;
the batch name and, where applicable, the scenario title are in the body. Givn
does not enforce Conventional Commits and does not compose the subject. Commit
one coherent completed behavior batch, preserving unrelated work. For a raw Git
commit, the checked-in `commit-msg` hook stamps attribution when context exists;
a raw Fossil check-in carries no attribution. `givn commit` works on both.

## 5. Run the managed refresh

After the manual repairs:

```sh
givn upgrade
```

The command requires a clean checkout and captures a checkpoint revision. It
refreshes the managed configuration scaffold, generated skills, managed
`AGENTS.md` blocks, and ignore state, and creates one allowlist-only managed
revision when files changed. It does not create a migration change: the shipped
catalog has no pre-1.0 route.

A project that still has retired keys, executable settings in `config.yaml`, or
malformed YAML is refused before any file is written. Fix the named setting and
run it again.

## 6. Verify the transition

1. `givn status` reports the expected change, stage, and SCM backend.
2. `givn instructions` reports the next required action and names the skill.
3. `givn instructions dialog` loads the mandatory dialog policy; the
   communication policy addon only controls presentation, not the obligation to
   ask answerable questions.
4. `givn instructions challenge` loads the challenge policy if the project uses
   it.
5. `givn check tasks --change <id>` runs the configured changed-scenario
   verification and writes a fresh receipt.
6. `git diff` (or `fossil diff`) shows only intended managed changes plus your
   manual repairs.

## 7. Recovery and limitations

`givn upgrade` is not a transaction. It reports:

- the checkpoint revision;
- the writes it completed;
- the revisions it actually published;
- the best-effort restoration it performed;
- remaining partial state and pending work.

It performs no reset, clean, or history rewrite and promises no complete
rollback. Recover from the checkpoint manually (`git restore` or
`fossil update`) or fix forward, then rerun. An interrupted archive is a
separate recovery path: repair the committed candidate and rerun
`givn archive --change <id>`; see
[the reference](../reference.md#verification-and-recovery).

Not supported by this transition:

- automatic conversion of an arbitrary 0.x configuration;
- aliases for retired addon keys;
- embedded pre-1.0 migration bundles or aggregate plans;
- a claim that a `givn_config_version` marker proves migration completion.

## Future migration bundles

The catalog remains the release-owned mechanism for future transitions. A
future bundle declares a unique id, strictly advancing source and target
versions, a deterministic order, and Proposal/Design assets under
`assets/migrations/`. The resolver follows multiple bundles in order, treats a
bundle as covering later patch releases of its source minor line, and creates
one aggregate maintenance change when a route exists. The current catalog
declares its schema and ships `migrations: []`; synthetic fixtures exercise the
future-route contract without fabricating a shipped route.

*[Documentation index](../INDEX.md)*
