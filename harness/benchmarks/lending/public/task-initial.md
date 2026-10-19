# Task: small lending register

Build a small lending register for a community library. Read `contract.md`
first; it is the authoritative interface and contains every requirement.

Deliver a Python command-line application started as:

```sh
python3 app.py --data-dir DATA_DIR
```

Users drive it with JSON lines on standard input and read JSON lines on
standard output, exactly as the contract specifies.

The initial delivery must support:

- creating members and books through the documented interface;
- borrowing an available book;
- rejecting an already-loaned book;
- enforcing at most five active loans per member;
- returning a book and releasing capacity;
- retaining state across restarts;
- rejecting invalid requests without corrupting state.

Use only the Python standard library. Do not use the network, a server, a
database service, or a graphical interface.

Work through the repository as it stands. You may add tests and documentation.
When an open choice has an unambiguous recommended option, take it and record
the choice in `decisions.md`; do not wait for a human. Do not modify
`contract.md` or `task.md`.
