use std::cmp::{max, min};

use crate::util::*;

/// Returns the Levenshtein distance between source and target using Naive Recursion
///
/// **It is ill-advised to use this function because of it's terrible performance
/// characteristics.**
///
/// This implementation has a time complexity of O(3^n).
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `target` - The target sequence
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "SATURDAY";
/// let s2 = "SUNDAY";
/// let expected_leven = 3;

/// let leven_naive = levenshtein::levenshtein_naive(s1, s2);
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_naive<T, U>(source: U, target: U) -> usize
where
    T: PartialEq,
    U: AsRef<[T]>,
{
    let source = source.as_ref();
    let target = target.as_ref();

    // indices
    let i: usize = source.len();
    let j: usize = target.len();

    // base case
    if min(i, j) == 0 {
        return max(i, j);
    }

    let k = if source[i - 1] == target[j - 1] { 0 } else { 1 };

    let delete = levenshtein_naive(&source[..i - 1], target) + 1;
    let insert = levenshtein_naive(source, &target[..j - 1]) + 1;
    let substitute = levenshtein_naive(&source[..i - 1], &target[..j - 1]) + k;

    min(min(insert, delete), substitute)
}

/// Returns the Levenshtein distance and the distance matrix between source and target using
/// dynamic programming with tabulation.
///
/// This implementation has a time complexity of O(n^2) and a space complexity of O(n^2).
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `target` - The target sequence
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "SATURDAY";
/// let s2 = "SUNDAY";
/// let expected_leven = 3;

/// let (leven_naive, _) = levenshtein::levenshtein_tabulation(s1, s2);
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_tabulation<T, U>(i1: U, i2: U) -> (usize, DistanceMatrix)
where
    T: PartialEq + Clone,
    U: AsRef<[T]>,
{
    let i1 = i1.as_ref();
    let i2 = i2.as_ref();

    let m = i1.len();
    let n = i2.len();

    // table of distances
    let mut distances = get_distance_table(m, n);

    for i in 1..distances.len() {
        for j in 1..distances[0].len() {
            let k = if i1[i - 1] == i2[j - 1] { 0 } else { 1 };

            let delete = distances[i - 1][j] + 1;
            let insert = distances[i][j - 1] + 1;
            let substitute = distances[i - 1][j - 1] + k;

            distances[i][j] = min(min(delete, insert), substitute);
        }
    }

    (distances[m][n], distances)
}

/// Returns the Levenshtein distance and the distance matrix between source and target using
/// dynamic programming with memoization.
///
/// This implementation has a time complexity of O(n^2) and a space complexity of O(n^2).
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `target` - The target sequence
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "SATURDAY";
/// let s2 = "SUNDAY";
/// let expected_leven = 3;

/// let (leven_naive, _) = levenshtein::levenshtein_memoization(s1, s2);
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_memoization<T, U>(i1: U, i2: U) -> (usize, DistanceMatrix)
where
    T: PartialEq,
    U: AsRef<[T]>,
{
    fn levenshtein_memoization_helper<T>(
        i1: &[T],
        i2: &[T],
        i: usize,
        j: usize,
        distances: &mut DistanceMatrix,
    ) -> usize
    where
        T: PartialEq,
    {
        // check the cache first
        if distances[i][j] < usize::MAX {
            return distances[i][j];
        }

        // base case
        if min(i1[..i].len(), i2[..j].len()) == 0 {
            return max(i1[..i].len(), i2[..j].len());
        }

        // couldn't find the value, time to recursively calculate it

        let k = if i1[i - 1] == i2[j - 1] { 0 } else { 1 };

        let delete = levenshtein_memoization_helper(i1, i2, i - 1, j, distances) + 1;
        let insert = levenshtein_memoization_helper(i1, i2, i, j - 1, distances) + 1;
        let substitute = levenshtein_memoization_helper(i1, i2, i - 1, j - 1, distances) + k;

        let distance = min(min(delete, insert), substitute);

        // update the cache
        distances[i][j] = distance;

        distance
    }

    let i1 = i1.as_ref();
    let i2 = i2.as_ref();

    let m = i1.len();
    let n = i2.len();

    let mut distances = get_distance_table(m, n);

    let distance = levenshtein_memoization_helper(i1, i2, m, n, &mut distances);

    (distance, distances)
}

#[cfg(test)]
mod tests {
    use crate::distance::*;

    #[test]
    fn levenshtein_naive_test() {
        let s1 = String::from("LAWN");
        let s2 = String::from("FFLAWANN");
        let expected_leven = 4;

        let leven_naive = levenshtein_naive(s1, s2);

        assert_eq!(leven_naive, expected_leven);
    }

    #[test]
    fn levenshtein_memoization_test() {
        let s1 = String::from("LAWN");
        let s2 = String::from("FFLAWANN");
        let expected_leven = 4;

        let (leven_memo, _) = levenshtein_memoization(s1, s2);

        assert_eq!(leven_memo, expected_leven);
    }

    #[test]
    fn levenshtein_tabulation_test() {
        let s1 = String::from("LAWN");
        let s2 = String::from("FFLAWANN");
        let expected_leven = 4;

        let (leven_tab, _) = levenshtein_tabulation(s1, s2);

        assert_eq!(leven_tab, expected_leven);
    }
}
