pub type DistanceMatrix = Vec<Vec<usize>>;

pub fn print_table(table: &DistanceMatrix) {
    for row in table {
        for item in row {
            print!("{} ", item);
        }
        println!("");
    }
}

pub fn remove_common_affix<'a, 'b, T: PartialEq>(s1: &'a[T], s2: &'b[T]) -> (&'a[T], &'b[T]) {
    let prefix_len = s1.iter()
        .zip(s2.iter())
        .take_while(|t| t.0 == t.1)
        .count();
    let suffix_len = s1[prefix_len..].iter()
        .rev()
        .zip(s2[prefix_len..].iter().rev())
        .take_while(|t| t.0 == t.1)
        .count();

    let s1 = &s1[prefix_len .. s1.len() - suffix_len];
    let s2 = &s2[prefix_len .. s2.len() - suffix_len];
    (s1, s2)
}

// Returns an initialized distance table of dimensions m+1 * n+1
// Where the first row is 0..n+1
// The First column is 0..m+1
// And the rest of the values are usize::MAX
pub fn get_distance_table(m: usize, n: usize) -> DistanceMatrix {
    let mut distances = Vec::with_capacity(m + 1);

    // The first row
    distances.push((0..n + 1).collect());

    for i in 1..m + 1 {
        // initialize the whole row to sentinel
        distances.push(vec![usize::MAX; n + 1]);
        // update the first item in the row
        distances[i][0] = i;
    }

    distances
}

pub fn up_to_last<T>(slice: &[T]) -> &[T] {
    slice.split_last().map_or(&[], |(_, rest)| rest)
}
