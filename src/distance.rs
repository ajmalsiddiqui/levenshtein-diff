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

/// let leven_naive = levenshtein::levenshtein_naive(s1.as_bytes(), s2.as_bytes());
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_naive<T: PartialEq>(source: &[T], target: &[T]) -> usize {
    // base case
    if source.is_empty() || target.is_empty() {
        return max(source.len(), target.len());
    }

    if source.last() == target.last() {
        // The item being looked at is the same, so it wouldn't contribute to the distance
        return levenshtein_naive(up_to_last(source), up_to_last(target));
    }

    // The items being looked at are different, so we must consider all possibilities

    let delete = levenshtein_naive(up_to_last(source), target) + 1;
    let insert = levenshtein_naive(source, up_to_last(target)) + 1;
    let substitute = levenshtein_naive(up_to_last(source), up_to_last(target)) + 1;

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

/// let (leven_naive, _) = levenshtein::levenshtein_tabulation(s1.as_bytes(), s2.as_bytes());
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_tabulation<T: PartialEq>(source: &[T], target: &[T]) -> (usize, DistanceMatrix) {
    let (source, target) = remove_common_affix(source, target);
    let m = source.len();
    let n = target.len();

    // table of distances
    let mut distances = get_distance_table(m, n);

    for i in 1..distances.len() {
        for j in 1..distances[0].len() {
            if source[i - 1] == target[j - 1] {
                // The item being looked at is the same, so the distance won't increase
                distances[i][j] = distances[i - 1][j - 1];
                continue;
            }

            let delete = distances[i - 1][j] + 1;
            let insert = distances[i][j - 1] + 1;
            let substitute = distances[i - 1][j - 1] + 1;

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

/// let (leven_naive, _) = levenshtein::levenshtein_memoization(s1.as_bytes(), s2.as_bytes());
/// assert_eq!(leven_naive, expected_leven);
/// ```
pub fn levenshtein_memoization<T: PartialEq>(
    source: &[T],
    target: &[T],
) -> (usize, DistanceMatrix) {
    fn levenshtein_memoization_helper<T: PartialEq>(
        source: &[T],
        target: &[T],
        distances: &mut DistanceMatrix,
    ) -> usize {
        // check the cache first
        if distances[source.len()][target.len()] < usize::MAX {
            return distances[source.len()][target.len()];
        }

        // base case
        if source.is_empty() || target.is_empty() {
            return max(source.len(), target.len());
        }

        // couldn't find the value, time to recursively calculate it

        let k = if source.last() == target.last() { 0 } else { 1 };

        let delete = levenshtein_memoization_helper(up_to_last(source), target, distances) + 1;
        let insert = levenshtein_memoization_helper(source, up_to_last(target), distances) + 1;
        let substitute =
            levenshtein_memoization_helper(up_to_last(source), up_to_last(target), distances) + k;

        let distance = min(min(delete, insert), substitute);

        // update the cache
        distances[source.len()][target.len()] = distance;

        distance
    }

    let (source, target) = remove_common_affix(source, target);
    let mut distances = get_distance_table(source.len(), target.len());

    let distance = levenshtein_memoization_helper(source, target, &mut distances);

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

        let leven_naive = levenshtein_naive(s1.as_bytes(), s2.as_bytes());

        assert_eq!(leven_naive, expected_leven);
    }

    #[test]
    fn levenshtein_memoization_test() {
        let s1 = String::from("LAWN");
        let s2 = String::from("FFLAWANN");
        let expected_leven = 4;

        let (leven_memo, _) = levenshtein_memoization(s1.as_bytes(), s2.as_bytes());

        assert_eq!(leven_memo, expected_leven);
    }

    #[test]
    fn levenshtein_tabulation_test() {
        let s1 = String::from("LAWN");
        let s2 = String::from("FFLAWANN");
        let expected_leven = 4;

        let (leven_tab, _) = levenshtein_tabulation(s1.as_bytes(), s2.as_bytes());

        assert_eq!(leven_tab, expected_leven);
    }
}
