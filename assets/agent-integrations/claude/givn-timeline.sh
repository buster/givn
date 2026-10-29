#!/bin/sh
# Givn development timeline for Claude Code.
#
# Recording is best effort and may be incomplete. The hook never fails or
# indefinitely delays the agent action it reports: a helper that stalls is
# stopped after the same two-second bound the other reporters use.
command -v givn >/dev/null 2>&1 || exit 0
# A non-interactive shell gives a background job /dev/null on stdin, so the
# hook payload is staged first; the helper still reads the real payload.
payload=$(mktemp "${TMPDIR:-/tmp}/givn-timeline.XXXXXX") || exit 0
cat > "$payload"
givn history harness-hook --harness claude < "$payload" &
pid=$!
(
  sleep 2
  kill -9 "$pid" 2>/dev/null || true
) &
watchdog=$!
wait "$pid" 2>/dev/null || true
kill "$watchdog" 2>/dev/null || true
rm -f "$payload"
exit 0
