use grid::Grid;

pub type DistanceMatrix = Grid<usize>;

pub fn print_table(table: &DistanceMatrix) {
    (0..table.rows()).for_each(|row| {
        table.iter_row(row).for_each(|item| print!(" {}", item));
        println!("");
    });
}

// Returns an initialized distance table of dimensions m+1 * n+1
// Where the first row is 0..n+1
// The First column is 0..m+1
// And the rest of the values are usize::MAX
pub fn get_distance_table(m: usize, n: usize) -> DistanceMatrix {
    let mut distances = Grid::init(m + 1, n + 1, usize::MAX);
    (0..=n).for_each(|i| distances[0][i] = i);
    (1..=m).for_each(|i| distances[i][0] = i);
    distances
}

pub fn up_to_last<T>(slice: &[T]) -> &[T] {
    slice.split_last().map_or(&[], |(_, rest)| rest)
}
