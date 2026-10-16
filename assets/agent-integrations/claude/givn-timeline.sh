#!/bin/sh
command -v givn >/dev/null 2>&1 || exit 0
exec givn history harness-hook --harness claude
