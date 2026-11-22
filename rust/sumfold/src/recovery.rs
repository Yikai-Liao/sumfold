// SPDX-License-Identifier: BSD-2-Clause
//! Exact folded moments, with a bounded schedule and a direct fallback.

use std::collections::BTreeMap;

use num_bigint::BigUint;

use crate::{Error, Term, convolve};

/// Computes exact convolution using certified singleton recovery when possible.
///
/// Has the same input, output and error contract as [`convolve`]. This experimental
/// implementation folds indices modulo 17, 67, 257 and 1031, certifies singleton
/// buckets using exact moments, and checks recovered mass against total input mass.
/// Previously recovered indices are deduplicated, never added twice. If those
/// passes do not recover everything, [`convolve`] supplies the complete result.
///
/// This deterministic schedule does **not** implement the output-sensitive
/// complexity bound in Vaughn's algorithm. It is an executable recovery mechanism,
/// using arbitrary-precision packed multiplication rather than an NTT.
///
/// # Errors
///
/// Returns the arithmetic errors documented on [`convolve`]. Intermediate moments
/// use arbitrary precision, so a moment overflow cannot reject a representable result.
///
/// # Memory
///
/// Uses `O(a + b + t)` fixed-width words for input copies and recovered terms,
/// plus big integers for at most 1031 folded buckets per moment. Integer widths
/// depend on total weight and coordinate bits. The fallback has [`convolve`]'s costs.
///
/// # Examples
///
/// ```
/// use sumfold::{Term, convolve_recovering};
/// assert_eq!(convolve_recovering(&[Term::new(1, 3)], &[Term::new(8, 2)])?,
///            vec![Term::new(9, 6)]);
/// # Ok::<(), sumfold::Error>(())
/// ```
pub fn convolve_recovering(left: &[Term], right: &[Term]) -> Result<Vec<Term>, Error> {
    let left: Vec<_> = left
        .iter()
        .copied()
        .filter(|term| term.weight != 0)
        .collect();
    let right: Vec<_> = right
        .iter()
        .copied()
        .filter(|term| term.weight != 0)
        .collect();
    if left.is_empty() || right.is_empty() {
        return Ok(Vec::new());
    }
    // Nonnegative coordinates make the maximum sum a complete overflow check.
    let max_left = crate::scan::max_index(&left).unwrap_or(0);
    let max_right = crate::scan::max_index(&right).unwrap_or(0);
    max_left
        .checked_add(max_right)
        .ok_or(Error::IndexOverflow)?;
    let total = mass(&left) * mass(&right);
    let mut recovered = BTreeMap::new();
    let mut recovered_mass = BigUint::default();
    for buckets in [17, 67, 257, 1031] {
        let left_moments = fold(&left, buckets);
        let right_moments = fold(&right, buckets);
        let zeroth = multiply(&left_moments[0], &right_moments[0]);
        let first_left = multiply(&left_moments[1], &right_moments[0]);
        let first_right = multiply(&left_moments[0], &right_moments[1]);
        let second_left = multiply(&left_moments[2], &right_moments[0]);
        let second_right = multiply(&left_moments[0], &right_moments[2]);
        let cross = multiply(&left_moments[1], &right_moments[1]);
        for bucket in 0..buckets {
            let weight = &zeroth[bucket];
            if weight == &BigUint::default() {
                continue;
            }
            let first = &first_left[bucket] + &first_right[bucket];
            let second = &second_left[bucket] + &second_right[bucket] + (&cross[bucket] << 1);
            // With positive weights, zero variance proves one distinct index.
            if weight * second != &first * &first {
                continue;
            }
            let index = (&first / weight).to_u64_digits();
            let index = index.first().copied().unwrap_or(0);
            if let std::collections::btree_map::Entry::Vacant(entry) = recovered.entry(index) {
                let digits = weight.to_u64_digits();
                if digits.len() > 2 {
                    return Err(Error::WeightOverflow);
                }
                let value = u128::from(digits.first().copied().unwrap_or(0))
                    | (u128::from(digits.get(1).copied().unwrap_or(0)) << 64);
                entry.insert(value);
                recovered_mass += weight;
            }
        }
        if recovered_mass == total {
            return Ok(recovered
                .into_iter()
                .map(|(index, weight)| Term::new(index, weight))
                .collect());
        }
    }
    convolve(&left, &right)
}

/// Sums weights without restricting intermediate mass to the output word size.
fn mass(terms: &[Term]) -> BigUint {
    terms.iter().map(|term| BigUint::from(term.weight)).sum()
}

/// Folds the zeroth, first and second coordinate moments into additive buckets.
fn fold(terms: &[Term], buckets: usize) -> [Vec<BigUint>; 3] {
    let mut moments = std::array::from_fn(|_| vec![BigUint::default(); buckets]);
    for term in terms {
        let bucket = (term.index % buckets as u64) as usize;
        let weight = BigUint::from(term.weight);
        let first = &weight * term.index;
        moments[0][bucket] += weight;
        moments[1][bucket] += &first;
        moments[2][bucket] += first * term.index;
    }
    moments
}

/// Computes cyclic convolution by carry-free packing into one big integer.
fn multiply(left: &[BigUint], right: &[BigUint]) -> Vec<BigUint> {
    let buckets = left.len();
    let bound = left.iter().sum::<BigUint>() * right.iter().sum::<BigUint>();
    // Every linear coefficient is at most the product of the two masses. One
    // extra bit makes the lane base strictly greater, including at powers of two.
    let lane_bits = bound.bits() as usize + 1;
    let pack = |coefficients: &[BigUint]| {
        coefficients
            .iter()
            .rev()
            .fold(BigUint::default(), |packed, value| {
                (packed << lane_bits) + value
            })
    };
    let mut product = pack(left) * pack(right);
    let mask = (BigUint::from(1_u8) << lane_bits) - BigUint::from(1_u8);
    let mut result = vec![BigUint::default(); buckets];
    for index in 0..(2 * buckets - 1) {
        result[index % buckets] += &product & &mask;
        product >>= lane_bits;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_product_wraps() {
        let left = [1_u32, 2, 3].map(BigUint::from);
        let right = [4_u32, 5, 6].map(BigUint::from);
        assert_eq!(multiply(&left, &right), [31_u32, 31, 28].map(BigUint::from));
    }
}
