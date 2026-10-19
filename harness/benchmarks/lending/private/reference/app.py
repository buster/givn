#!/usr/bin/env python3
"""Reference implementation of the lending register public contract."""

from __future__ import annotations

import argparse
import json
import os
import sys
import tempfile
from pathlib import Path

STATE_FILENAME = "state.json"
STATE_VERSION = 1
MAX_ACTIVE_LOANS = 5


class RequestError(Exception):
    def __init__(self, code: str) -> None:
        super().__init__(code)
        self.code = code


class Store:
    def __init__(self, data_dir: Path) -> None:
        self.data_dir = data_dir
        self.data_dir.mkdir(parents=True, exist_ok=True)
        self.path = self.data_dir / STATE_FILENAME
        self.members: dict[str, dict] = {}
        self.books: dict[str, dict] = {}
        self.load()

    def load(self) -> None:
        if not self.path.is_file():
            return
        try:
            loaded = json.loads(self.path.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            return
        if not isinstance(loaded, dict) or loaded.get("version") != STATE_VERSION:
            return
        members = loaded.get("members")
        books = loaded.get("books")
        if isinstance(members, dict):
            self.members = {
                str(key): value if isinstance(value, dict) else {}
                for key, value in members.items()
            }
        if isinstance(books, dict):
            self.books = {
                str(key): {
                    "title": str(value.get("title") or ""),
                    "reference_only": bool(value.get("reference_only")),
                    "loaned_to": value.get("loaned_to") if isinstance(value.get("loaned_to"), str) else None,
                }
                for key, value in books.items()
                if isinstance(value, dict)
            }

    def save(self) -> None:
        payload = {
            "version": STATE_VERSION,
            "members": self.members,
            "books": self.books,
        }
        fd, temporary = tempfile.mkstemp(prefix=".state-", dir=str(self.data_dir), text=True)
        try:
            with os.fdopen(fd, "w", encoding="utf-8") as handle:
                json.dump(payload, handle, sort_keys=True)
                handle.flush()
                os.fsync(handle.fileno())
            os.replace(temporary, self.path)
            directory_fd = os.open(self.data_dir, os.O_RDONLY)
            try:
                os.fsync(directory_fd)
            finally:
                os.close(directory_fd)
        except BaseException:
            try:
                os.unlink(temporary)
            except OSError:
                pass
            raise

    def active_loan_count(self, member_id: str) -> int:
        return sum(
            1 for book in self.books.values() if book.get("loaned_to") == member_id
        )

    def add_member(self, member_id: str) -> None:
        if member_id in self.members:
            raise RequestError("member_exists")
        self.members[member_id] = {}
        self.save()

    def add_book(self, book_id: str, title: str) -> None:
        if book_id in self.books:
            raise RequestError("book_exists")
        self.books[book_id] = {
            "title": title,
            "reference_only": False,
            "loaned_to": None,
        }
        self.save()

    def borrow(self, member_id: str, book_id: str) -> None:
        if member_id not in self.members:
            raise RequestError("unknown_member")
        if book_id not in self.books:
            raise RequestError("unknown_book")
        book = self.books[book_id]
        if book.get("reference_only"):
            raise RequestError("reference_only")
        if book.get("loaned_to") is not None:
            raise RequestError("book_already_loaned")
        if self.active_loan_count(member_id) >= MAX_ACTIVE_LOANS:
            raise RequestError("loan_limit_reached")
        book["loaned_to"] = member_id
        self.save()

    def return_book(self, member_id: str, book_id: str) -> None:
        if member_id not in self.members:
            raise RequestError("unknown_member")
        if book_id not in self.books:
            raise RequestError("unknown_book")
        book = self.books[book_id]
        if book.get("loaned_to") != member_id:
            raise RequestError("loan_not_found")
        book["loaned_to"] = None
        self.save()

    def set_reference_only(self, book_id: str, reference_only: bool) -> None:
        if book_id not in self.books:
            raise RequestError("unknown_book")
        self.books[book_id]["reference_only"] = reference_only
        self.save()


def require_string(request: dict, key: str) -> str:
    value = request.get(key)
    if not isinstance(value, str) or not value:
        raise RequestError("bad_request")
    return value


def require_boolean(request: dict, key: str) -> bool:
    value = request.get(key)
    if not isinstance(value, bool):
        raise RequestError("bad_request")
    return value


def handle(store: Store, request: object) -> dict:
    if not isinstance(request, dict):
        return {"ok": False, "error": "bad_request"}
    command = request.get("command")
    if not isinstance(command, str) or not command:
        return {"ok": False, "error": "bad_request"}
    try:
        if command == "ping":
            return {"ok": True}
        if command == "add_member":
            store.add_member(require_string(request, "member_id"))
            return {"ok": True}
        if command == "add_book":
            store.add_book(
                require_string(request, "book_id"),
                require_string(request, "title"),
            )
            return {"ok": True}
        if command == "borrow":
            store.borrow(
                require_string(request, "member_id"),
                require_string(request, "book_id"),
            )
            return {"ok": True}
        if command == "return":
            store.return_book(
                require_string(request, "member_id"),
                require_string(request, "book_id"),
            )
            return {"ok": True}
        if command == "set_reference_only":
            store.set_reference_only(
                require_string(request, "book_id"),
                require_boolean(request, "reference_only"),
            )
            return {"ok": True}
        return {"ok": False, "error": "unknown_command"}
    except RequestError as error:
        return {"ok": False, "error": error.code}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--data-dir", required=True)
    args = parser.parse_args(argv)
    store = Store(Path(args.data_dir))
    for line in sys.stdin:
        if not line.strip():
            continue
        try:
            request = json.loads(line)
        except ValueError:
            response = {"ok": False, "error": "bad_request"}
        else:
            response = handle(store, request)
        sys.stdout.write(json.dumps(response) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
