# SPDX-License-Identifier: BSD-2-Clause
"""Exercise policy boundaries and real Git trailer parsing."""

import unittest

from policy import valid_signoff, valid_title


class PolicyTests(unittest.TestCase):
    """Keep title enforcement and DCO matching independent of PR authorship."""

    def test_titles(self):
        for title in [
            "feat(core): compute exact convolution",
            "fix!: reject overflow",
            "docs: explain recovery",
        ]:
            self.assertTrue(valid_title(title), title)
        for title in [
            "feat: bad.",
            "wrong: title",
            "feat(other): title",
            "feat: a\nb",
            "feat: " + "a" * 60,
        ]:
            self.assertFalse(valid_title(title), title)

    def test_signoff_matches_author(self):
        message = "feat: implement convolution\n\nSigned-off-by: Ada <ada@example.org>\nAssisted-by: Codex\n"
        self.assertTrue(valid_signoff("Ada", "ada@example.org", message))
        self.assertFalse(valid_signoff("Other", "ada@example.org", message))
        self.assertFalse(valid_signoff("Ada", "other@example.org", message))

    def test_body_is_not_trailer(self):
        message = (
            "feat: explain sign-off\n\nSigned-off-by: Ada <ada@example.org>\n\nThis is body text.\n"
        )
        self.assertFalse(valid_signoff("Ada", "ada@example.org", message))


if __name__ == "__main__":
    unittest.main()
