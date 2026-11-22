// SPDX-License-Identifier: BSD-2-Clause
//! Behavioral conformance against an independent pair-list oracle.
use sumfold::{Error, Term, convolve};

/// Independent oracle: enumerate, sort, then run-length aggregate.
fn oracle(left: &[Term], right: &[Term]) -> Vec<Term> {
    let mut pairs: Vec<_> = left
        .iter()
        .flat_map(|a| {
            right
                .iter()
                .map(move |b| Term::new(a.index + b.index, a.weight * b.weight))
        })
        .filter(|term| term.weight != 0)
        .collect();
    pairs.sort_by_key(|term| term.index);
    let mut result: Vec<Term> = Vec::new();
    for term in pairs {
        if let Some(last) = result.last_mut().filter(|last| last.index == term.index) {
            last.weight += term.weight;
        } else {
            result.push(term);
        }
    }
    result
}

fn check(left: &[Term], right: &[Term], expected: Result<Vec<Term>, Error>) {
    assert_eq!(convolve(left, right), expected);
    assert_eq!(convolve(right, left), expected);
    #[cfg(feature = "recovery")]
    {
        assert_eq!(sumfold::convolve_recovering(left, right), expected);
        assert_eq!(sumfold::convolve_recovering(right, left), expected);
    }
}

#[test]
fn duplicates_and_zeros() {
    check(
        &[Term::new(9, 3), Term::new(0, 0), Term::new(9, 2)],
        &[Term::new(1, 4)],
        Ok(vec![Term::new(10, 20)]),
    );
}

#[test]
fn empty_ignores_indices() {
    check(&[Term::new(u64::MAX, u128::MAX)], &[], Ok(vec![]));
    check(&[Term::new(u64::MAX, 0)], &[Term::new(1, 1)], Ok(vec![]));
}

#[test]
fn index_boundary_checked() {
    check(
        &[Term::new(u64::MAX - 1, 1)],
        &[Term::new(1, 1)],
        Ok(vec![Term::new(u64::MAX, 1)]),
    );
    check(
        &[Term::new(u64::MAX, 1)],
        &[Term::new(1, 1)],
        Err(Error::IndexOverflow),
    );
}

#[test]
fn weights_do_not_wrap() {
    check(
        &[Term::new(0, u128::MAX)],
        &[Term::new(0, 2)],
        Err(Error::WeightOverflow),
    );
    check(
        &[Term::new(0, u128::MAX), Term::new(0, 1)],
        &[Term::new(0, 1)],
        Err(Error::WeightOverflow),
    );
}

#[test]
fn total_mass_unbounded() {
    check(
        &[Term::new(0, u128::MAX), Term::new(u64::MAX, u128::MAX)],
        &[Term::new(0, 1)],
        Ok(vec![
            Term::new(0, u128::MAX),
            Term::new(u64::MAX, u128::MAX),
        ]),
    );
}

#[test]
fn exhaustive_small_histograms() {
    // All pairs of four-bin histograms over weights 0, 1, 2.
    let histograms: Vec<Vec<Term>> = (0..81)
        .map(|mut code| {
            (0..4)
                .map(|index| {
                    let term = Term::new(index, code % 3);
                    code /= 3;
                    term
                })
                .collect()
        })
        .collect();
    for left in &histograms {
        for right in &histograms {
            check(left, right, Ok(oracle(left, right)));
        }
    }
}

#[test]
fn collisions_remain_distinct() {
    // Every coordinate collides in all scheduled moduli; recovery must fall back.
    let stride = 17 * 67 * 257 * 1031;
    let terms = [
        Term::new(0, 2),
        Term::new(stride, 3),
        Term::new(stride * 2, 4),
    ];
    check(&terms, &terms, Ok(oracle(&terms, &terms)));
}

#[test]
fn wide_sparse_coordinates() {
    let left = [Term::new(1 << 60, 17), Term::new(5, 19), Term::new(5, 23)];
    let right = [Term::new(1 << 61, 29), Term::new(7, 31)];
    check(&left, &right, Ok(oracle(&left, &right)));
}
