#!/bin/sh
# One-time setup for the @e2e browser tier: install the Playwright pinned in
# tests/e2e/package.json and the Chromium that Playwright bundles.
#
# This is the only step of the change that touches the network, and it runs
# once, outside the suite. The tier itself needs no server, no port, no
# database, and no external service: the report under test is one
# self-contained file and the record is a local file, so the browser loads the
# report from a file:// URL inside the temporary project root a scenario
# created. Node.js is resolved from PATH (the design does not pin it); the
# browser under test is always this Chromium, never a system browser.
set -eu

cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

npm ci --prefix tests/e2e
npx --prefix tests/e2e playwright install chromium
