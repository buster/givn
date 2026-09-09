#!/bin/sh
set -u

# Claude Code invokes this through its Bash PreToolUse hook. The Givn command
# returns an updated Bash command only when the payload contains a Git commit.
command -v givn >/dev/null 2>&1 || exit 0
exec givn attribution claude-pre-tool-use
