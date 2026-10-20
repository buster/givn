# Lending register — public CLI contract

This contract is authoritative. Every independently graded obligation appears
here. Hidden grading may exercise combinations and boundary sequences, but it
does not introduce unstated expectations.

## Startup

```sh
python3 app.py --data-dir DIR
```

* `DIR` is a writable directory chosen by the caller. Create it if missing.
* The program reads one JSON object per line from standard input and writes
  exactly one JSON object per line to standard output.
* Flush standard output after every response. Do not write other text to
  standard output; diagnostics go to standard error.
* Exit with status 0 when standard input closes.
* State is stored under `DIR` and must survive a normal restart. A successful
  response means the mutation is already durable enough to survive a restart.
* Do not use the network. The standard library is sufficient.

## Requests

Every request has a `command` field naming one operation.

| Command | Fields | Success response |
|---|---|---|
| `ping` | — | `{"ok": true}` |
| `add_member` | `member_id` | `{"ok": true}` |
| `add_book` | `book_id`, `title` | `{"ok": true}` |
| `borrow` | `member_id`, `book_id` | `{"ok": true}` |
| `return` | `member_id`, `book_id` | `{"ok": true}` |
| `set_reference_only` | `book_id`, `reference_only` (boolean) | `{"ok": true}` |

Identifiers are non-empty strings. `title` is a non-empty string; it is
descriptive only.

## Errors

A rejected request returns:

```json
{"ok": false, "error": "error_code"}
```

Recognized error codes:

| Code | Condition |
|---|---|
| `member_exists` | `add_member` with an existing member ID |
| `book_exists` | `add_book` with an existing book ID |
| `unknown_member` | A member ID that was never added |
| `unknown_book` | A book ID that was never added |
| `book_already_loaned` | `borrow` while that book is loaned to anyone |
| `loan_limit_reached` | `borrow` when the member already has five active loans |
| `loan_not_found` | `return` when that member has no active loan of that book |
| `reference_only` | `borrow` of a book currently marked reference-only |
| `unknown_command` | A `command` value not in the table |
| `bad_request` | Missing fields, wrong field types, or invalid JSON line |

A rejected request must not change any member, book or loan state.

## Lending rules

1. A member may have at most **five active loans** at one time.
2. A book may be loaned to at most **one member** at a time.
3. Returning a book removes that member's active loan of it and releases both
   the book and the member's capacity.
4. A successful borrow followed by a successful return leaves the system in a
   state where the same member can borrow the same book again.
5. All state persists across restarts under `--data-dir`.

## Reference-only books (follow-up change)

Every book starts with `reference_only = false`.

* `set_reference_only` marks a book reference-only (`true`) or available again
  (`false`). The flag persists across restarts.
* While a book is reference-only, new `borrow` requests for it fail with
  `reference_only`.
* A book that was already loaned when it became reference-only can still be
  returned normally.
* The flag does not affect other books, nor any other lending rule.

## Examples

```text
-> {"command":"add_member","member_id":"m1"}
<- {"ok":true}
-> {"command":"add_book","book_id":"b1","title":"Dune"}
<- {"ok":true}
-> {"command":"borrow","member_id":"m1","book_id":"b1"}
<- {"ok":true}
-> {"command":"borrow","member_id":"m2","book_id":"b1"}
<- {"ok":false,"error":"book_already_loaned"}
-> {"command":"return","member_id":"m1","book_id":"b1"}
<- {"ok":true}
```
