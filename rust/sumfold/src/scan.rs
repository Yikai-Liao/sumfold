// SPDX-License-Identifier: BSD-2-Clause
//! SIMD input validation shared by both convolution engines.

use crate::Term;
use fearless_simd::prelude::*;
use fearless_simd::{Level, Simd, dispatch, u64x4};

/// Finds the largest positive-weight input index with runtime SIMD dispatch.
pub(crate) fn max_index(terms: &[Term]) -> Option<u64> {
    dispatch!(Level::new(), simd => max_index_with(simd, terms))
}

/// Reduces active indices, masking zero weights before computing a maximum.
fn max_index_with<S: Simd>(simd: S, terms: &[Term]) -> Option<u64> {
    let mut maximum = u64x4::splat(simd, 0);
    let mut present = false;
    let (chunks, remainder) = terms.as_chunks::<4>();
    for chunk in chunks {
        let indices = std::array::from_fn::<_, 4, _>(|lane| {
            let term = chunk[lane];
            present |= term.weight != 0;
            if term.weight == 0 { 0 } else { term.index }
        });
        maximum = maximum.max(u64x4::from_slice(simd, &indices));
    }
    let mut maximum = maximum.reduce_max();
    for term in remainder {
        if term.weight != 0 {
            present = true;
            maximum = maximum.max(term.index);
        }
    }
    present.then_some(maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_lane_and_tail() {
        for len in 0..35 {
            for active in 0..len {
                let mut terms = vec![Term::new(u64::MAX, 0); len];
                terms[active] = Term::new(active as u64, 1);
                let expected = terms
                    .iter()
                    .filter(|term| term.weight != 0)
                    .map(|term| term.index)
                    .max();
                for level in [Level::baseline(), Level::new()] {
                    assert_eq!(
                        dispatch!(level, simd => max_index_with(simd, &terms)),
                        expected
                    );
                }
            }
        }
        assert_eq!(max_index(&[]), None);
        assert_eq!(max_index(&[Term::new(u64::MAX, 0); 8]), None);
        assert_eq!(max_index(&[Term::new(u64::MAX, 1); 8]), Some(u64::MAX));
    }
}
