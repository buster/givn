# Local subtree synchronization

The standalone repository at `/home/buster/projects/givn-harness` is
authoritative. The parent Rust repository keeps a downstream copy under
`harness/`. This is Git subtree synchronization, not a symlink and not a
submodule.

The join was validated in a disposable clone of the parent on 2026-10-19:

- standalone base: `25672fe` (`feat(harness): build evaluation harness for
  records and staged benchmarks`)
- parent pre-join: `c76720db`
- disposable join commit: `6d02443f`
- `git ls-tree -r --name-only` under `harness/` matched the standalone tracked
  tree exactly; no path outside `harness/` changed.

The real parent was not modified. Its working tree currently contains local
harness work (untracked `harness/improv-plan/`, `harness/givn/.gitignore`,
`harness/tasks/.gitignore`, modified generated `SOURCES.txt`). Preserve or
commit that work before the real join.

## Why a manual subtree join

`git subtree pull` refuses two unrelated histories with no
`--allow-unrelated-histories` passthrough. The supported operation is therefore
the underlying subtree merge: create a merge commit whose second parent is the
standalone history, then replace the `harness/` prefix with the standalone
tree. No files outside the prefix change.

Run this in a disposable clone first:

```sh
git clone /home/buster/projects/givn /tmp/givn-parent-sync
cd /tmp/givn-parent-sync
git remote add harness-local /home/buster/projects/givn-harness
git fetch harness-local

git merge --allow-unrelated-histories --no-commit -s ours harness-local/main
git rm -r --cached harness
rm -rf harness
git read-tree --prefix=harness/ -u harness-local/main^{tree}
git commit -m "chore(harness): synchronize standalone evaluation harness"
```

Validate:

```sh
git ls-tree -r --name-only HEAD harness | sed 's#^harness/##' | sort > /tmp/parent-files
git -C /home/buster/projects/givn-harness ls-tree -r --name-only HEAD | sort > /tmp/standalone-files
diff -u /tmp/standalone-files /tmp/parent-files
git diff --stat HEAD^1..HEAD -- . ':(exclude)harness'
```

The second diff must be empty. The first must be empty after sorting.

## Subsequent synchronization

Repeat the same replace-prefix merge from the parent clone, fetching the
standalone `main` first. Do not use `git subtree add` over the existing prefix.
Do not edit `harness/` independently: fixes belong in the standalone
repository, then flow down.

## Never synchronize

- `.venv`, caches, `__pycache__`, `*.egg-info`
- `givn/history/`, `runs/`, `analysis/`, private reports
- credentials, auth stores, provider tokens
- generated runtime workspaces

These are already excluded by the standalone `.gitignore`. Verify with
`git status --ignored` before syncing.
