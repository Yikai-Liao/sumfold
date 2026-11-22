// SPDX-License-Identifier: BSD-2-Clause
//! Alternating baseline A/A and recovery measurements with equality checks.

use std::hint::black_box;
use std::time::Instant;

use sumfold::{Term, convolve, convolve_recovering};

/// Builds repeatable inputs spanning output overlap and coordinate width.
fn inputs(length: u64, distribution: &str) -> (Vec<Term>, Vec<Term>) {
    let (left_stride, right_stride) = match distribution {
        "overlap" => (1, 1),
        "wide" => (1 << 32, 1 << 32),
        "distinct" => (length, 1),
        _ => panic!("unknown distribution: {distribution}"),
    };
    let left = (0..length)
        .map(|index| Term::new(index * left_stride, u128::from(index % 7 + 1)))
        .collect();
    let right = (0..length)
        .map(|index| Term::new(index * right_stride, u128::from(index % 5 + 1)))
        .collect();
    (left, right)
}

/// Hashes all output bits so timing records retain input/result identity.
fn checksum(terms: &[Term]) -> u64 {
    terms.iter().fold(0xcbf29ce484222325, |hash, term| {
        [term.index, term.weight as u64, (term.weight >> 64) as u64]
            .into_iter()
            .fold(hash, |hash, value| {
                (hash ^ value).wrapping_mul(0x100000001b3)
            })
    })
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(
        args.len(),
        3,
        "usage: sumfold-bench LENGTH DISTRIBUTION SAMPLES"
    );
    let length: u64 = args[0].parse().expect("integer length");
    let samples: usize = args[2].parse().expect("integer samples");
    assert!((1..=4096).contains(&length) && (1..=1000).contains(&samples));
    let (left, right) = inputs(length, &args[1]);
    let expected = convolve(&left, &right).unwrap();
    assert_eq!(convolve_recovering(&left, &right).unwrap(), expected);
    let input_hash = checksum(&left).wrapping_add(checksum(&right).rotate_left(17));
    let output_hash = checksum(&expected);
    for sample in 0..samples {
        let order = if sample % 2 == 0 {
            ["direct-a", "direct-b", "recovery"]
        } else {
            ["recovery", "direct-b", "direct-a"]
        };
        for algorithm in order {
            let start = Instant::now();
            let result = if algorithm == "recovery" {
                convolve_recovering(black_box(&left), black_box(&right))
            } else {
                convolve(black_box(&left), black_box(&right))
            }
            .unwrap();
            let elapsed_ns = start.elapsed().as_nanos();
            assert_eq!(result, expected);
            println!(
                "{{\"algorithm\":\"{algorithm}\",\"length\":{length},\"distribution\":\"{}\",\"sample\":{sample},\"elapsed_ns\":{elapsed_ns},\"output_len\":{},\"input_hash\":\"{input_hash:016x}\",\"output_hash\":\"{output_hash:016x}\"}}",
                args[1],
                result.len()
            );
            black_box(result);
        }
    }
}
