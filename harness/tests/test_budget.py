from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.budget import (
    BudgetError,
    BudgetExceeded,
    PriceTable,
    SpendLedger,
    estimate_prompt_tokens,
    estimate_request_usd,
    load_prices,
)


class BudgetTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.ledger = SpendLedger(self.root / "spend.json", 5.0)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_admission_refuses_when_reserved_plus_new_exceeds_cap(self) -> None:
        self.ledger.reserve("one", 3.0)
        with self.assertRaises(BudgetExceeded):
            self.ledger.reserve("two", 2.5)
        self.assertEqual(self.ledger.totals()["reserved_usd"], 3.0)

    def test_commit_replaces_reservation_with_actual(self) -> None:
        self.ledger.reserve("one", 2.0)
        self.ledger.mark_dispatched("one")
        self.ledger.commit("one", 1.25)
        totals = self.ledger.totals()
        self.assertEqual(totals["committed_usd"], 1.25)
        self.assertEqual(totals["reserved_usd"], 0.0)
        self.assertEqual(totals["remaining_usd"], 3.75)

    def test_dispatched_request_cannot_be_refunded(self) -> None:
        self.ledger.reserve("one", 1.0)
        self.ledger.mark_dispatched("one")
        with self.assertRaises(BudgetError):
            self.ledger.release_undispatched("one")

    def test_undispatched_request_can_be_released(self) -> None:
        self.ledger.reserve("one", 1.0)
        self.ledger.release_undispatched("one")
        self.assertEqual(self.ledger.totals()["reserved_usd"], 0.0)

    def test_crash_leaves_reservation_in_place(self) -> None:
        self.ledger.reserve("one", 2.0)
        self.ledger.mark_dispatched("one")
        # Simulate a crash: construct a new ledger over the same file.
        recovered = SpendLedger(self.root / "spend.json", 5.0)
        totals = recovered.totals()
        self.assertEqual(totals["reserved_usd"], 2.0)
        with self.assertRaises(BudgetExceeded):
            recovered.reserve("two", 3.5)

    def test_missing_cost_becomes_uncertain_and_blocks_further_dispatch(self) -> None:
        self.ledger.reserve("one", 1.0)
        self.ledger.mark_dispatched("one")
        self.ledger.commit("one", None, note="usage missing")
        totals = self.ledger.totals()
        self.assertEqual(totals["uncertain_usd"], 1.0)
        with self.assertRaises(BudgetError):
            self.ledger.assert_reconciled()

    def test_price_table_requires_dated_source(self) -> None:
        price_path = self.root / "prices.toml"
        price_path.write_text(
            """
schema = 1
currency = "USD"
source = "test fixture"
date = "2026-01-01"
safety_factor = 1.25
[per_million_tokens]
input = 1.0
output = 2.0
""",
            encoding="utf-8",
        )
        prices = load_prices(price_path)
        self.assertEqual(prices.source, "test fixture")
        estimate = estimate_request_usd("x" * 3000, 1000, prices)
        expected = ((1000 * 1.0 + 1000 * 2.0) / 1_000_000.0) * 1.25
        self.assertAlmostEqual(estimate, expected)
        self.assertEqual(estimate_prompt_tokens("x" * 3000), 1000)

    def test_incomplete_price_table_is_rejected(self) -> None:
        price_path = self.root / "bad.toml"
        price_path.write_text("schema = 1\nsource = 'x'\ndate = 'y'\n", encoding="utf-8")
        with self.assertRaises(Exception):
            load_prices(price_path)

    def test_placeholder_price_table_is_rejected(self) -> None:
        price_path = self.root / "placeholder.toml"
        price_path.write_text(
            """
schema = 1
currency = "USD"
source = "example"
date = "2026-01-01"
placeholder = true
[per_million_tokens]
input = 0.0
output = 0.0
""",
            encoding="utf-8",
        )
        with self.assertRaises(Exception):
            load_prices(price_path)


if __name__ == "__main__":
    unittest.main()
