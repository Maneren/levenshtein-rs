#![no_std]
//! A crate for calculating the levenshtein distance of two strings
//!
//! All functions have two versions, one with custom weights and one with equal weights.
//!
//! # Examples
//! ```
//! # use levenshtein_distance::levenshtein;
//! assert_eq!(levenshtein("kitten", "kitten"), 0);
//! assert_eq!(levenshtein("kitten", "kittens"), 1);
//! assert_eq!(levenshtein("kitten", "sitting"), 3);
//! ```
//! ```
//! # use levenshtein_distance::levenshtein_with_weigths;
//! assert_eq!(levenshtein_with_weigths("kitten", "sitting", 3, 2, 1), 6);
//! ```
//! ```
//! # use levenshtein_distance::levenshtein_is_within_limit;
//! assert!(levenshtein_is_within_limit("kitten", "kitten", 0));
//! assert!(levenshtein_is_within_limit("kitten", "kittens", 1));
//! assert!(!levenshtein_is_within_limit("kitten", "sitting", 1));
//! ```
//! ```
//! # use levenshtein_distance::levenshtein_is_within_limit_with_weigths;
//! assert!(levenshtein_is_within_limit_with_weigths("kitten", "sitting", 6, 3, 2, 1));
//! ```

extern crate alloc;
use alloc::vec::Vec;

/// Calculates the levenshtein distance of the two strings
///
/// Uses weight 1 for all operations, for more control use [`levenshtein_with_weigths`](levenshtein_with_weigths)
///
/// # Examples
/// ```
/// # use levenshtein_distance::levenshtein;
/// assert_eq!(levenshtein("kitten", "kitten"), 0);
/// assert_eq!(levenshtein("kitten", "kittens"), 1);
/// assert_eq!(levenshtein("kitten", "sitting"), 3);
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    levenshtein_with_weigths(a, b, 1, 1, 1)
}

/// Calculates the levenshtein distance of the two strings with custom weights
///
/// For equal weights use [`levenshtein`](levenshtein)
///
/// # Example
/// ```
/// # use levenshtein_distance::levenshtein_with_weigths;
/// assert_eq!(levenshtein_with_weigths("kitten", "sitting", 3, 2, 1), 6);
/// ```
pub fn levenshtein_with_weigths(
    a: &str,
    b: &str,
    substitute: usize,
    delete: usize,
    insert: usize,
) -> usize {
    // short circuit if equal
    if a == b {
        return 0;
    }

    // use `chars::count` instead of `str::length` to prevent weird unicode issues
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    // short circuit if empty
    if a.is_empty() {
        return b_len;
    }
    if b.is_empty() {
        return a_len;
    }

    // initialize as the first row, filled with ascending numbers
    let mut cache = (1..=b_len).collect::<Vec<_>>();

    // keep track of the value directly to the left of the current field
    // at the end of the algorithm this will be the bottom right field aka the result
    let mut left = 1;

    for (i, a) in a.chars().enumerate() {
        let mut diagonal = i; // first column is just ascending numbers
        left = diagonal + 1; // left is always one row below the current field

        for (j, b) in b.chars().enumerate() {
            // calculate the three possible values for the current field
            let m1 = diagonal + usize::from(a != b) * substitute; // substitute
            let m2 = cache[j] + delete; // delete
            let m3 = left + insert; // insert

            let best = m1.min(m2).min(m3);

            // prepare for next iteration

            // cache[j] is the field directly above the current field
            // so it will be the diagonal in the next iteration
            diagonal = cache[j];

            // current best value will be the left field in the next iteration
            left = best;

            // and store the best value for the next row as well
            cache[j] = best;
        }
    }

    left
}

/// Checks if the levenshtein distance of the two strings is within the given limit. This is faster
/// than calculating the distance and comparing it to the limit since it allows for early termination
///
/// Uses weight 1 for all operations, for more control use [`levenshtein_is_within_limit_with_weigths`](levenshtein_is_within_limit_with_weigths)
///
/// # Examples
/// ```
/// # use levenshtein_distance::levenshtein_is_within_limit;
/// assert!(levenshtein_is_within_limit("kitten", "kitten", 0));
/// assert!(levenshtein_is_within_limit("kitten", "kittens", 1));
/// assert!(!levenshtein_is_within_limit("kitten", "sitting", 1));
/// ```
pub fn levenshtein_is_within_limit(a: &str, b: &str, limit: usize) -> bool {
    levenshtein_is_within_limit_with_weigths(a, b, limit, 1, 1, 1)
}

/// Checks if the levenshtein distance of the two strings is within the given limit with custom
/// weights. This is faster than calculating the distance and comparing it to the limit since it allows for
/// early termination
///
/// For equal weights use [`levenshtein_is_within_limit`](levenshtein_is_within_limit)
///
/// # Example
/// ```
/// # use levenshtein_distance::levenshtein_is_within_limit_with_weigths;
/// assert!(levenshtein_is_within_limit_with_weigths("kitten", "sitting", 6, 3, 2, 1));
/// ```
pub fn levenshtein_is_within_limit_with_weigths(
    a: &str,
    b: &str,
    limit: usize,
    substitute: usize,
    delete: usize,
    insert: usize,
) -> bool {
    // short circuit if equal
    if a == b {
        return true;
    }

    // use `chars::count` instead of `str::length` to prevent weird unicode issues
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    // short circuit if empty
    if a.is_empty() {
        return b_len < limit;
    }
    if b.is_empty() {
        return a_len < limit;
    }

    // initialize as the first row, filled with ascending numbers
    let mut cache = (1..=b_len).collect::<Vec<_>>();

    // keep track of the value directly to the left of the current field
    // at the end of the algorithm this will be the bottom right field aka the result
    let mut left;

    for (i, a) in a.chars().enumerate() {
        let mut diagonal = i; // first column is just ascending numbers
        left = diagonal + 1; // left is always one row below the current field

        let mut best_of_row = usize::MAX;

        for (j, b) in b.chars().enumerate() {
            // calculate the three possible values for the current field
            let m1 = diagonal + usize::from(a != b) * substitute; // substitute
            let m2 = cache[j] + delete; // delete
            let m3 = left + insert; // insert

            let best = m1.min(m2).min(m3);

            // prepare for next iteration

            // cache[j] is the field directly above the current field
            // so it will be the diagonal in the next iteration
            diagonal = cache[j];

            // current best value will be the left field in the next iteration
            left = best;

            // and store the best value for the next row as well
            cache[j] = best;

            if best < best_of_row {
                best_of_row = best;
            }
        }

        if best_of_row > limit {
            return false;
        }
    }

    true
}
