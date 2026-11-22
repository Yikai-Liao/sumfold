// SPDX-License-Identifier: BSD-2-Clause
//! Text interface to exact sparse convolution.

use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::ExitCode;

use sumfold::{Term, convolve, convolve_recovering};

const HELP: &str = "Usage: sumfold convolve [--recover] LEFT RIGHT

Read whitespace-separated INDEX WEIGHT lines from each file.
Indices are u64 and weights are u128 nonnegative integers.
Blank lines and lines beginning with # are ignored.
Write sorted INDEX WEIGHT lines to stdout. Errors go to stderr.
--recover enables experimental exact moment recovery with a direct fallback.
Use --help or --version without a subcommand.";

/// Reads a histogram, retaining line numbers in parse diagnostics.
fn read_terms(path: &Path) -> Result<Vec<Term>, String> {
    let file = std::fs::File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut terms = Vec::new();
    for (line, text) in io::BufReader::new(file).lines().enumerate() {
        let text = text.map_err(|error| format!("{}:{}: {error}", path.display(), line + 1))?;
        let text = text.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = text.split_whitespace().collect();
        let parse = || -> Option<Term> {
            if fields.len() != 2 {
                return None;
            }
            Some(Term::new(fields[0].parse().ok()?, fields[1].parse().ok()?))
        };
        terms.push(parse().ok_or_else(|| {
            format!(
                "{}:{}: expected INDEX WEIGHT (u64 u128)",
                path.display(),
                line + 1
            )
        })?);
    }
    Ok(terms)
}

/// Runs one complete request before opening stdout for result emission.
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("{HELP}");
        return Ok(());
    }
    if args.len() == 1 && args[0] == "--version" {
        println!("sumfold {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let recover = args.get(1).is_some_and(|arg| arg == "--recover");
    let offset = if recover { 2 } else { 1 };
    if args.first().is_none_or(|arg| arg != "convolve") || args.len() != offset + 2 {
        return Err(HELP.into());
    }
    let left = read_terms(Path::new(&args[offset]))?;
    let right = read_terms(Path::new(&args[offset + 1]))?;
    let result = if recover {
        convolve_recovering(&left, &right)
    } else {
        convolve(&left, &right)
    }
    .map_err(|error| error.to_string())?;
    let mut output = io::BufWriter::new(io::stdout().lock());
    for term in result {
        writeln!(output, "{} {}", term.index, term.weight).map_err(|error| error.to_string())?;
    }
    output.flush().map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("sumfold: {message}");
            ExitCode::FAILURE
        }
    }
}
