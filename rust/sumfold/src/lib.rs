// SPDX-License-Identifier: BSD-2-Clause
//! Exact convolution of sparse nonnegative integer histograms.
//!
//! Inputs may be unordered and contain duplicates or zero weights. Results contain
//! one positive coefficient per index, in ascending order. Arithmetic errors return
//! [`Error`]; no partial output escapes. See [`convolve`] for the baseline and,
//! with the `recovery` feature, [`convolve_recovering`] for moment recovery.
//!
//! ```
//! use sumfold::{Term, convolve};
//! let left = [Term::new(0, 2), Term::new(10, 3)];
//! let right = [Term::new(1, 4), Term::new(11, 5)];
//! assert_eq!(convolve(&left, &right)?, vec![
//!     Term::new(1, 8), Term::new(11, 22), Term::new(21, 15),
//! ]);
//! # Ok::<(), sumfold::Error>(())
//! ```

use std::collections::BTreeMap;
use std::fmt;

mod scan;

#[cfg(feature = "recovery")]
mod recovery;
#[cfg(feature = "recovery")]
pub use recovery::convolve_recovering;

/// A histogram index and its nonnegative integer weight.
///
/// The same representation is used for input and output. A zero weight contributes
/// nothing, including when its index would otherwise overflow during addition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Term {
    /// Position on the nonnegative integer axis.
    pub index: u64,
    /// Exact weight at this position.
    pub weight: u128,
}

impl Term {
    /// Constructs a term, including a permitted zero-weight term.
    #[must_use]
    pub const fn new(index: u64, weight: u128) -> Self {
        Self { index, weight }
    }
}

/// An exact result cannot be represented by [`Term`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A sum of two positive-weight input indices exceeds `u64::MAX`.
    IndexOverflow,
    /// An output coefficient exceeds `u128::MAX`.
    WeightOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::IndexOverflow => "output index exceeds u64::MAX",
            Self::WeightOverflow => "output weight exceeds u128::MAX",
        })
    }
}

impl std::error::Error for Error {}

/// Computes exact sparse nonnegative convolution by enumerating positive pairs.
///
/// Each pair contributes `left.weight * right.weight` at `left.index + right.index`.
/// Inputs need not be sorted or unique and are never modified. Empty or all-zero
/// inputs yield an empty vector. Output indices are unique and ascending.
///
/// # Errors
///
/// Returns [`Error::IndexOverflow`] or [`Error::WeightOverflow`] when an output
/// cannot be represented. If both apply, either may be reported. Allocation failure
/// follows Rust's allocator behavior rather than returning an arithmetic error.
///
/// # Memory
///
/// For `a` and `b` input entries and `t` output indices, takes
/// `O(a + b + a*b*log(t + 1))` time and `O(t)` auxiliary memory.
/// No array proportional to the coordinate span is allocated.
///
/// # Examples
///
/// ```
/// use sumfold::{Term, convolve};
/// assert_eq!(convolve(&[Term::new(7, 2)], &[Term::new(9, 3)])?,
///            vec![Term::new(16, 6)]);
/// # Ok::<(), sumfold::Error>(())
/// ```
pub fn convolve(left: &[Term], right: &[Term]) -> Result<Vec<Term>, Error> {
    if let (Some(left), Some(right)) = (scan::max_index(left), scan::max_index(right)) {
        left.checked_add(right).ok_or(Error::IndexOverflow)?;
    }
    let mut coefficients = BTreeMap::<u64, u128>::new();
    for left in left.iter().filter(|term| term.weight != 0) {
        for right in right.iter().filter(|term| term.weight != 0) {
            let index = left
                .index
                .checked_add(right.index)
                .ok_or(Error::IndexOverflow)?;
            let weight = left
                .weight
                .checked_mul(right.weight)
                .ok_or(Error::WeightOverflow)?;
            let coefficient = coefficients.entry(index).or_default();
            *coefficient = coefficient
                .checked_add(weight)
                .ok_or(Error::WeightOverflow)?;
        }
    }
    Ok(coefficients
        .into_iter()
        .map(|(index, weight)| Term::new(index, weight))
        .collect())
}
