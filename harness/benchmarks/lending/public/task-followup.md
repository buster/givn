# Follow-up task: reference-only books

This is a follow-up change to the lending register already in this repository.
Use the repository as it stands. Do not rebuild it from scratch.

Staff need to mark a book as **reference-only**:

- a reference-only book cannot be borrowed;
- an existing loan of that book can still be returned;
- the flag can be turned off again;
- the flag persists across restarts;
- all existing lending rules and existing lending data keep working.

The public contract is still `contract.md`. Implement the
`set_reference_only` command and the `reference_only` borrowing error exactly
as documented. Keep the JSON-lines interface unchanged.

Use only the Python standard library. Do not use the network, a server, a
database service, or a graphical interface.

When an open choice has an unambiguous recommended option, take it and record
the choice in `decisions.md`; do not wait for a human. Do not modify
`contract.md` or `task.md`.
