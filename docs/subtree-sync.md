# Local subtree synchronization

The standalone repository at `/home/buster/projects/givn-harness` is
authoritative. The parent Rust repository keeps a downstream copy under
`harness/`. This is Git subtree synchronization, not a symlink and not a
submodule.

The parent repository's `harness/` prefix already existed before this project.
Do not run `git subtree add` over that prefix. Fetch and merge instead.

## One-time local remote

From the parent repository:

```sh
git remote add harness-local /home/buster/projects/givn-harness
git fetch harness-local
```

The standalone repository must have at least one commit. The synchronization
base is the first commit that contains the standalone tree; record its hash here
after bootstrap:

```text
sync base: <commit>
```

## Pull standalone changes into the parent prefix

From the parent repository, on a disposable branch first:

```sh
git switch -c harness-sync-check
git subtree pull --prefix=harness harness-local main \
  -m "chore(harness): synchronize standalone evaluation harness"
git diff --stat HEAD^
```

Inspect the diff. The parent prefix should become the standalone root contents.
If the histories are unrelated, the first synchronization needs an explicit
merge strategy; test it in a disposable clone before touching the real parent.

## Push a focused parent-prefix revision back

Do not edit `harness/` directly. If a parent-only fix is unavoidable, keep it
small and immediately copy it back to the standalone repository. Then re-pull
from the standalone repository so the prefix returns to a downstream copy.

## Validation

A synchronization is demonstrated only when:

1. the standalone repository is its own Git root and has a givn project;
2. `./run-tests.sh` passes in the standalone repository;
3. after pull, `harness/` in a disposable parent clone matches the standalone
   tracked files (excluding private runtime data);
4. no credentials, `.venv`, caches, private reports or generated runtime state
   are copied.
