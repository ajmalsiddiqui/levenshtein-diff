use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use levenshtein_diff as levenshtein;

#[test]
fn test_regeneration_of_target() {
    let source = "SATURDAY";
    let target = "SUNDAY";

    let expected_leven = 3;

    // dist: usize is the Levenshtein distance, and the mat is the distance matrix
    let (dist, mat) = levenshtein::distance(source.as_bytes(), target.as_bytes());

    assert_eq!(expected_leven, dist);

    // Generate a sequence of edits (i.e. differences between source and target)
    let edits = levenshtein::generate_edits(source.as_bytes(), target.as_bytes(), &mat)
        .unwrap_or_else(|err| panic!("{}", err));

    // Apply edits to source to regenerate target. This results in a Vec
    let generated_target_vec = levenshtein::apply_edits(source.as_bytes(), &edits);

    // Convert the vector from above into a string
    let generated_target = match std::str::from_utf8(&generated_target_vec) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    assert_eq!(target, generated_target);
}

#[test]
fn test_strings_have_same_distances() {
    fn rand_alnum_string(n: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
    }

    let mut rng = thread_rng();

    let random_len_1: usize = rng.gen_range(1..10);
    let random_len_2: usize = rng.gen_range(1..10);

    let random_str_1 = rand_alnum_string(random_len_1);
    let random_str_2 = rand_alnum_string(random_len_2);

    let leven_naive =
        levenshtein::levenshtein_naive(random_str_1.as_bytes(), random_str_2.as_bytes());
    let (leven_tab, _) =
        levenshtein::levenshtein_tabulation(random_str_1.as_bytes(), random_str_2.as_bytes());
    let (leven_memo, _) =
        levenshtein::levenshtein_memoization(random_str_1.as_bytes(), random_str_2.as_bytes());

    // Putting all three assertions here though one would be redundant to easily identify the
    // broken function if the test fails
    assert_eq!(leven_naive, leven_tab);
    assert_eq!(leven_naive, leven_memo);
    assert_eq!(leven_tab, leven_memo);
}

#[test]
fn test_for_cases_that_differ_by_one_item() {
    // Test for issue https://github.com/ajmalsiddiqui/levenshtein-diff/issues/2

    let source_collection: Vec<String> = vec!["Hello".into(), "World".into()];
    let target_collection: Vec<String> = vec!["World".into()];

    let (distance, matrix) = levenshtein_diff::distance(&source_collection, &target_collection);

    assert_eq!(distance, 1);

    let edits =
        levenshtein_diff::generate_edits(&source_collection, &target_collection, &matrix).unwrap();

    let generated_target_vec = levenshtein::apply_edits(&source_collection, &edits);

    // Just a sanity check
    assert_eq!(target_collection.len(), generated_target_vec.len())
}
