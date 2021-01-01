#[test]
fn test_regeneration_of_target() {
    use levenshtein_diff as levenshtein;

    let source = "SATURDAY";
    let target = "SUNDAY";

    let expected_leven = 3;

    // dist: usize is the Levenshtein distance, and the mat is the distance matrix
    let (dist, mat) = levenshtein::distance(source, target);

    assert_eq!(expected_leven, dist);

    // Generate a sequence of edits (i.e. differences between source and target)
    let edits = levenshtein::generate_edits(source, target, &mat)
        .unwrap_or_else(|err| panic!(err));

    // Apply edits to source to regenerate target. This results in a Vec
    let generated_target_vec = levenshtein::apply_edits(source, &edits);

    // Convert the vector from above into a string
    let generated_target = match std::str::from_utf8(&generated_target_vec) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    assert_eq!(target, generated_target);
}
