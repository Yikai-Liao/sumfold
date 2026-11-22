// SPDX-License-Identifier: BSD-2-Clause
//! End-to-end CLI output and diagnostic contracts.
use std::process::Command;

#[test]
fn convolution_emits_sorted() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    for arguments in [vec!["convolve"], vec!["convolve", "--recover"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_sumfold"))
            .args(arguments)
            .arg(root.join("left.txt"))
            .arg(root.join("right.txt"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{:?}", output);
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            "1 8\n11 22\n21 15\n"
        );
    }
}

#[test]
fn invalid_arguments_fail() {
    let output = Command::new(env!("CARGO_BIN_EXE_sumfold"))
        .arg("convolve")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr).unwrap().contains("Usage:"));
}

#[test]
fn malformed_input_is_atomic() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let output = Command::new(env!("CARGO_BIN_EXE_sumfold"))
        .arg("convolve")
        .arg(root.join("invalid.txt"))
        .arg(root.join("invalid.txt"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("invalid.txt:3:")
    );
}
