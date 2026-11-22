# SPDX-License-Identifier: BSD-2-Clause
"""Check PR titles and author-matching Developer Certificate sign-offs."""

import os
import re
import subprocess

TYPES = "feat|fix|perf|refactor|docs|test|ci|build|chore|revert"
SCOPES = "core|simd|recovery|cli|bench|site|deps"
TITLE = re.compile(rf"(?:{TYPES})(?:\((?:{SCOPES})\))?!?: [a-zA-Z0-9].+")


def valid_title(title: str) -> bool:
    """Match the contribution policy without accepting multiline titles."""
    return bool(len(title) <= 64 and TITLE.fullmatch(title) and not title.endswith("."))


def valid_signoff(name: str, email: str, message: str) -> bool:
    """Require the matching sign-off in Git's parsed trailer block."""
    trailers = subprocess.check_output(
        ["git", "interpret-trailers", "--parse"], input=message, text=True
    )
    expected = f"Signed-off-by: {name} <{email}>"
    return expected in trailers.splitlines()


def main() -> None:
    """Validate trusted event values against the base checkout's policy."""
    if not valid_title(os.environ["PR_TITLE"]):
        raise SystemExit("title must follow CONTRIBUTING.md and be at most 64 characters")
    base, head = os.environ["BASE"], os.environ["HEAD"]
    if not all(re.fullmatch("[0-9a-f]{40}", ref) for ref in [base, head]):
        raise SystemExit("expected full commit hashes")
    commits = subprocess.check_output(["git", "rev-list", f"{base}..{head}"], text=True)
    for commit in commits.splitlines():
        result = subprocess.check_output(
            ["git", "show", "-s", "--format=%an%x00%ae%x00%B", commit], text=True
        )
        name, email, message = result.split("\0", 2)
        if not valid_signoff(name, email, message):
            raise SystemExit(f"{commit}: missing author-matching Signed-off-by trailer")


if __name__ == "__main__":
    main()
