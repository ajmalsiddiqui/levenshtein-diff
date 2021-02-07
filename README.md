# Levenshtein Algorithm and a Bit More

The Levenshtein algorithm is a technique traditionally used to calculate the Levenshtein distance (i.e. edit distance) between two strings. However, nothing about the algorithm restricts it to strings. The algorithm works just as well to find the edit distance between any two sequences whose elements have an equivalence relation defined (i.e. for elements `a` and `b` in the sequence, you can tell if `a == b` and `a != b` are true or false).

This crate implements a generic Levenshtein's algorithm that works for any sequence of types that can be compared and cloned.

In addition to that, it let's you generate a sequence of `Edit` values that represent the transformations that, when applied to the source sequence, will convert it into the target sequence. A function that applies the edits to the source sequence and reconstructs the target sequence is also supplied.

## Features

* Three implementations of Levenshtein's algorithm: naive recursive, DP with tabulation and DP with memoization. Useful if you want to analyze and compare performance.
* Generate the sequence of edits that, when applied to the source sequence, will regenrate the target sequence. Useful when you want to sync with a remote copy of your sequence efficiently.
* Function to apply edits to a sequence in order to generate the target sequence.
* Generic: works on a sequence of any type that implements `PartialEq` (though the sequence will also have to implement `Clone` if you want to use the features related to generating and applying edits).

## Usage

In your `Cargo.toml`:

```toml
[dependencies]
levenshtein-diff = "0.2.1"
```

In your project:

```rust
    use levenshtein_diff as levenshtein;

    // This example uses strings
    let source = "SATURDAY";
    let target = "SUNDAY";

    let expected_leven = 3;

    // dist: usize is the Levenshtein distance, and the mat is the distance matrix
    let (dist, mat) = levenshtein::distance(source.as_bytes(), target.as_bytes());

    assert_eq!(expected_leven, dist);

    // Generate a sequence of edits (i.e. differences between source and target)
    let edits = levenshtein::generate_edits(source.as_bytes(), target.as_bytes(), &mat)
        .unwrap_or_else(|err| panic!(err));

    // Apply edits to source to regenerate target. This results in a Vec
    let generated_target_vec = levenshtein::apply_edits(source.as_bytes(), &edits);

    // Convert the vector from above into a string
    let generated_target = match std::str::from_utf8(&generated_target_vec) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    assert_eq!(target, generated_target);
```
