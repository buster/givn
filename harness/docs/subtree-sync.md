# Local subtree synchronization

The standalone repository at `/home/buster/projects/givn-harness` is
authoritative. The parent Rust repository keeps a downstream copy under
`harness/`. This is Git subtree synchronization, not a symlink and not a
submodule.

## Current join

The real parent join was performed on 2026-10-19:

- standalone base: `f4bc0ee` (`docs(harness): document the
  observe-hypothesize-compare loop`)
- parent pre-join: `c76720db`
- parent join commit: `937494bf` (`chore(harness): join standalone evaluation
  harness under harness/`)
- parent safety tag: `harness-pre-subtree` at `c76720db`
- local backup of the pre-join working tree:
  `/tmp/opencode/givn-harness-parent-backup-20261019T144435Z/harness`
  (includes untracked `harness/improv-plan/`, modified generated
  `SOURCES.txt`, the old `.venv`, and private reports)

Validation after the join:

- `git ls-tree -r --name-only HEAD harness` matched the standalone tracked
  tree exactly after removing the prefix;
- no path outside `harness/` changed;
- `./run-tests.sh` in the parent `harness/` passed all 57 tests;
- no `.venv`, `__pycache__`, `*.egg-info`, `reports/`, `givn/history/`,
  `runs/` or `analysis/` paths entered the synchronized tree.

The unrelated untracked `tools/.gitignore` in the parent was left untouched.

## Why a manual subtree join

`git subtree pull` refuses two unrelated histories with no
`--allow-unrelated-histories` passthrough. The supported operation is the
underlying subtree merge: create a merge commit whose second parent is the
standalone history, then replace the `harness/` prefix with the standalone
tree. No files outside the prefix change.

## Subsequent synchronization

Run from the parent repository after committing in the standalone repository:

```sh
cd /home/buster/projects/givn
git fetch harness-local

# Prepare a merge whose second parent is the standalone revision.
git merge --no-commit -s ours harness-local/main

# Replace only the prefix. Move ignored runtime state aside first if present.
[ -d harness/.venv ] && mv harness/.venv /tmp/givn-harness-reusable-venv
git rm -r --cached harness
rm -rf harness
git read-tree --prefix=harness/ -u harness-local/main^{tree}
[ -d /tmp/givn-harness-reusable-venv ] && mv /tmp/givn-harness-reusable-venv harness/.venv

git commit -m "chore(harness): synchronize standalone evaluation harness"
```

If the histories are still unrelated (first join only), add
`--allow-unrelated-histories` to the merge command.

The merge commit records the standalone revision as its second parent. The
`harness/` prefix is a snapshot of that revision, not a second editable source.

## Validation

```sh
git ls-tree -r --name-only HEAD harness | sed 's#^harness/##' | sort > /tmp/parent-files
git -C /home/buster/projects/givn-harness ls-tree -r --name-only HEAD | sort > /tmp/standalone-files
diff -u /tmp/standalone-files /tmp/parent-files
git diff --stat HEAD^1..HEAD -- . ':(exclude)harness'
cd harness && ./run-tests.sh
```

The first diff must be empty after sorting. The second must be empty: the sync
changes nothing outside `harness/`.

## Never synchronize

- `.venv`, caches, `__pycache__`, `*.egg-info`
- `givn/history/`, `runs/`, `analysis/`, private reports
- credentials, auth stores, provider tokens
- generated runtime workspaces

These are excluded by the standalone `.gitignore`. Verify with
`git status --ignored` before syncing.

## Old harness state

The pre-join committed tree remains reachable at tag `harness-pre-subtree`.
The old givn specifications, archived changes, tasks and reports are preserved
there and in the local backup. They are not active specifications in the new
standalone project; the new project's own `givn/` state is authoritative.
