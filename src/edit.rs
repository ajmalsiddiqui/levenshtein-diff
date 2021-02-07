use std::cmp::min;
use std::error::Error;
use std::fmt;

use crate::util::DistanceMatrix;

/// Represents an error specific to working with the Levenshtein distance, or the generated
/// distance matrix
#[derive(Debug)]
pub enum LevenshteinError {
    // The supplied distance matrix is invalid
    InvalidDistanceMatrixError,
}

impl fmt::Display for LevenshteinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            LevenshteinError::InvalidDistanceMatrixError => "Invalid matrix error",
        };

        write!(f, "{}", error)
    }
}

impl Error for LevenshteinError {}

/// Represents an Edit applied on a source sequence.
#[derive(Clone, PartialEq)]
pub enum Edit<T: PartialEq> {
    Delete(usize),        // Delete item at index
    Insert(usize, T),     // Insert item T at index
    Substitute(usize, T), // Substitute item at index with T
}

/// Applies a sequence of edits on the source sequence, and returns a vector representing the
/// target sequence.
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `edits` - A reference to a vector of edits of the same type as elements of source
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "FLOWER";
/// let expected_s2 = "FOLLOWER";
///
/// let (_, matrix) = levenshtein::distance(s1.as_bytes(), expected_s2.as_bytes());
///
/// let edits = levenshtein::generate_edits(s1.as_bytes(), expected_s2.as_bytes(), &matrix).unwrap();
///
/// let target = levenshtein::apply_edits(s1.as_bytes(), &edits);
///
/// let s2 = match std::str::from_utf8(&target) {
///     Ok(v) => v,
///     Err(_) => panic!("Not a valid UTF-8 sequence!"),
/// };
///
/// assert_eq!(s2, expected_s2);
/// ```
pub fn apply_edits<T: Clone + PartialEq>(source: &[T], edits: &[Edit<T>]) -> Vec<T> {
    // Convert each item of source into Some(item)
    let mut target_constructor: Vec<Option<T>> =
        source.iter().map(|item| Some(item.clone())).collect();

    let mut inserts = Vec::<Edit<T>>::with_capacity(source.len());

    // We iterate in the reverse order because we want to populate the inserts vector in the
    // reverse order of indices. This ensures that we don't need any operational transforms on the
    // inserts.
    for edit in edits.iter().rev() {
        match edit {
            Edit::Substitute(idx, val) => target_constructor[idx - 1] = Some(val.clone()),
            Edit::Delete(idx) => target_constructor[idx - 1] = None,
            Edit::Insert(idx, val) => inserts.push(Edit::Insert(*idx, val.clone())),
        }
    }

    for i in &inserts {
        if let Edit::Insert(idx, val) = i {
            target_constructor.insert(*idx, Some(val.clone()));
        }
    }

    let mut target = Vec::<T>::new();
    for i in &target_constructor {
        match i {
            Some(val) => target.push(val.clone()),
            None => (),
        }
    }

    target
}

/// Generate a vector of edits that, when applied to the source sequence, transform it into the
/// target sequence.
///
/// # Arguments
///
/// * `source` - The source sequence
/// * `target` - The target sequence
/// * `distances` - A reference to the `DistanceMatrix` for converting source to target
///
/// # Examples
///
/// ```
/// use levenshtein_diff as levenshtein;
///
/// let s1 = "SATURDAY";
/// let s2 = "SUNDAY";
///
/// let (_, matrix) = levenshtein::distance(s1.as_bytes(), s2.as_bytes());
///
/// // This can be used with the `apply_edits` function to transform source to target
/// let edits = levenshtein::generate_edits(s1.as_bytes(), s2.as_bytes(), &matrix).unwrap();
/// ```
pub fn generate_edits<T: Clone + PartialEq>(
    source: &[T],
    target: &[T],
    distances: &DistanceMatrix,
) -> Result<Vec<Edit<T>>, LevenshteinError> {
    let mut source_idx = source.len();
    let mut target_idx = target.len();

    if source_idx + 1 != distances.len() || target_idx + 1 != distances[0].len() {
        return Err(LevenshteinError::InvalidDistanceMatrixError);
    }

    let mut edits = Vec::<Edit<T>>::new();

    // When both source and target indices are 0, we have succesfully computed all the edits
    // required to transform the source into the target
    while source_idx != 0 || target_idx != 0 {
        let current_item = distances[source_idx][target_idx];

        // These represent the options we have: substitute, insert and delete
        let substitute = Some(distances[source_idx - 1][target_idx - 1])
            .filter(|_| source_idx > 0 && target_idx > 0);

        let delete = Some(distances[source_idx - 1][target_idx]).filter(|_| source_idx > 0);

        let insert = Some(distances[source_idx][target_idx - 1]).filter(|_| target_idx > 0);

        let min = min(min(insert, delete), substitute);

        if min == Some(current_item) {
            source_idx = source_idx - 1;
            target_idx = target_idx - 1;
        } else if min == Some(current_item - 1) {
            if min == insert {
                // The edits are expected to be 1-indexed, but the slices obviously aren't
                // Hence we do target_idx - 1 to access the right value
                edits.push(Edit::Insert(source_idx, target[target_idx - 1].clone()));
                target_idx = target_idx - 1;
            } else if min == delete {
                edits.push(Edit::Delete(source_idx));
                source_idx = source_idx - 1;
            } else if min == substitute {
                edits.push(Edit::Substitute(source_idx, target[target_idx - 1].clone()));
                source_idx = source_idx - 1;
                target_idx = target_idx - 1;
            } else {
                return Err(LevenshteinError::InvalidDistanceMatrixError);
            };
        } else {
            return Err(LevenshteinError::InvalidDistanceMatrixError);
        };
    }

    Ok(edits)
}

#[cfg(test)]
mod tests {
    use crate::edit::*;

    // Copied verbatim from
    // https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn edit_list_is_correct() {
        let s1 = "SATURDAY";
        let s2 = "SUNDAY";

        // This is the distance matrix for the strings
        // SATURDAY and SUNDAY
        let distances = vec![
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![1, 0, 1, 2, 3, 4, 5],
            vec![2, 1, 1, 2, 3, 3, 4],
            vec![3, 2, 2, 2, 3, 4, 4],
            vec![4, 3, 2, 3, 3, 4, 5],
            vec![5, 4, 3, 3, 4, 4, 5],
            vec![6, 5, 4, 4, 3, 4, 5],
            vec![7, 6, 5, 5, 4, 3, 4],
            vec![8, 7, 6, 6, 5, 4, 3],
        ];

        let expected_edits = vec![
            Edit::<u8>::Substitute(5, 78),
            Edit::<u8>::Delete(3),
            Edit::<u8>::Delete(2),
        ];

        let edits = generate_edits(s1.as_bytes(), s2.as_bytes(), &distances).unwrap();

        assert_eq!(do_vecs_match(&edits, &expected_edits), true);
    }

    #[test]
    fn edits_are_applied_correctly() {
        let s1 = "SATURDAY";
        let expected_s2 = "SUNDAY";

        // Edits that convert SATURDAY to SUNDAY
        let mut edits = vec![
            Edit::<u8>::Substitute(5, 78),
            Edit::<u8>::Delete(3),
            Edit::<u8>::Delete(2),
        ];

        let s2_bytes_vec = apply_edits(s1.as_bytes(), &mut edits);

        let s2 = match std::str::from_utf8(&s2_bytes_vec) {
            Ok(v) => v,
            Err(_) => panic!("Not a valid UTF-8 sequence!"),
        };

        assert_eq!(s2, expected_s2);
    }
}
