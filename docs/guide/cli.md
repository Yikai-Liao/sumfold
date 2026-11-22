<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Command-line convolution

```sh
cargo run -p sumfold-cli -- convolve examples/left.txt examples/right.txt
cargo run -p sumfold-cli -- convolve --recover examples/left.txt examples/right.txt
```

`sumfold convolve [--recover] LEFT RIGHT` reads two UTF-8 text files. Each nonempty, noncomment line contains exactly two whitespace-separated decimal integers: `INDEX WEIGHT`. Leading and trailing whitespace is accepted. Lines whose first nonwhitespace character is `#` are comments; inline comments are not accepted. Indices parse as `u64`, weights as `u128`.

The default command uses direct ordered accumulation. `--recover` selects experimental exact folded recovery and its complete direct fallback. It does not enable or disable SIMD. `--help` and `--version` are top-level commands.

Output is one `INDEX WEIGHT` term per line, in ascending index order, with a final newline for each term. An empty result writes zero bytes. Diagnostics go to stderr with a file and line number for malformed input. Exit status is zero on success and nonzero on parse, arithmetic or I/O failure. Both files are read and convolution completes before output begins, so parse and arithmetic failures cannot emit a partial result. Output-device failures can interrupt a stream.

File names use native OS paths. Standard input and signed coordinates are not supported in this version. For lag correlation, an API with a signed output axis must be designed before adding a convenience command.
